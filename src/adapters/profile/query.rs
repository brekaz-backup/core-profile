use async_graphql::*;
use async_graphql::dataloader::DataLoader;
use blumer_lib_errors::AppError;
use uuid::Uuid;
use crate::adapters::profile::AppDataLoader;
use crate::adapters::profile::mappers::{ProfileDetailMapper, SimpleProfileMapper};
use crate::adapters::profile::objects::{Profile, ProfileDetail, SimpleProfile};
use crate::adapters::shared::state::AppState;
use crate::domain::mappers::graphql_mapper::GraphqlMapper;
use crate::domain::usecases::profile::{FindByUsernameUsecase, FindDetailsUseCase};
use crate::utils::errors::ProfileError;
use blumer_lib_auth_rs::{User, RoleGuard, Role};

#[derive(Default)]
pub struct ProfileQuery;

#[Object]
impl ProfileQuery {
    #[graphql(guard = "RoleGuard::new(vec![Role::TEMP, Role::USER])")]
    pub async fn profile_by_id(&self, ctx: &Context<'_>, id: Option<ID>) -> FieldResult<Profile> {
        let user = User::get_user(ctx).extend()?;

        can_view_permission(ctx, match id.clone() {
            Some(profile_id) => profile_id,
            _ => user.user_id.into()
        }).await.extend()?;

        find_profile_by_id_internal(ctx,
                                    match id {
                                        Some(profile_id) => profile_id,
                                        _ => user.user_id.into()
                                    },
        ).await
    }

    #[graphql(guard = "RoleGuard::new(vec![Role::TEMP, Role::USER])")]
    pub async fn profile_by_username(&self, ctx: &Context<'_>, #[graphql(validator(min_length = 4, max_length = 20))] username: String) -> FieldResult<Profile> {
        let state = ctx.data::<AppState>()?;
        let repository = &state.profile_repository;
        let redis_repository = &state.redis_repository;
        let user_id = &FindByUsernameUsecase::execute(&repository, &redis_repository, &username).await.extend()?;

        can_view_permission(ctx, user_id.into()).await.extend()?;

        let profile = find_profile_by_id_internal(ctx, user_id.into()).await.extend()?;
        Ok(profile)
    }

    #[graphql(entity)]
    pub async fn find_profile_by_id(&self, ctx: &Context<'_>, id: ID) -> Option<Profile> {
        find_profile_by_id_internal(ctx, id).await.ok()
    }


    #[graphql(entity)]
    pub async fn find_simple_profile_by_id(&self, ctx: &Context<'_>, id: ID) -> Option<SimpleProfile> {
        if let Ok(profile) = find_profile_by_id_internal(ctx, id).await {
            return Some(SimpleProfileMapper::to_object(profile));
        }
        None
    }
}

async fn can_view_permission(ctx: &Context<'_>, id: ID) -> Result<(), AppError> {
    return Ok(());
    let state = ctx.data::<AppState>().map_err(|_| AppError::ServerError("AppState Error".to_string()))?;
    let mut auth_client = state.auth_client.clone();
    let user = User::get_user(ctx)?;
    let user_id = Uuid::parse_str(&id.to_string()).map_err(|e| AppError::ServerError(e.to_string()))?;

    if user_id != user.user_id {
        let auth = auth_client.can_view_profile(
            user_id,
            user.user_id,
            false,
        ).await?;

        if !auth.valid {
            return Err(
                ProfileError::CantViewThisUser.into()
            );
        }
    }
    Ok(())
}


#[ComplexObject]
impl Profile {
    #[graphql(guard = "RoleGuard::new(vec![Role::USER])")]
    async fn details(&self, ctx: &Context<'_>) -> FieldResult<Option<ProfileDetail>> {
        let state = ctx.data::<AppState>()?;
        let user = User::get_user(ctx)?;
        if user.user_id != Uuid::parse_str(&self.id.to_string())? {
            return Ok(None);
        }
        let repository = &state.profile_repository;
        let redis_repository = &state.redis_repository;
        let details = FindDetailsUseCase::execute(&repository, &redis_repository, &user.user_id).await.extend()?;

        Ok(Some(ProfileDetailMapper::to_object(details)))
    }
}


async fn find_profile_by_id_internal(ctx: &Context<'_>, id: ID) -> FieldResult<Profile> {
    let user_id = Uuid::parse_str(&id.to_string())?;
    let data_loader = ctx.data::<DataLoader<AppDataLoader>>()?;
    let profiles = data_loader.load_one(user_id).await?;
    profiles.ok_or_else(|| "Not found".into())
}