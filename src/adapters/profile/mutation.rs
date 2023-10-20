use async_graphql::*;




use crate::adapters::shared::state::AppState;
use crate::domain::events::aggregation::{AGGREGATE_PROFILE_EDIT_TOPIC, ProfileEditEventProducer};

use crate::domain::usecases::profile::{EditDescriptionUseCase, EditNamesUseCase, EditPrivacyUseCase, EditUsernameUseCase};
use crate::infrastructure::kafka::KafkaProducerInterface;
use blumer_lib_auth_rs::{User, RoleGuard, Role};

#[derive(Default)]
pub struct ProfileMutation;


#[Object]
impl ProfileMutation {
    #[graphql(guard = "RoleGuard::new(vec![Role::USER])")]
    pub async fn profile_edit_username(&self, ctx: &Context<'_>, #[graphql(validator(min_length = 4, max_length = 20))] username: String) -> FieldResult<bool> {
        let user = User::get_user(ctx).extend()?;
        let state = ctx.data::<AppState>()?;
        let repository = &state.profile_repository;
        let redis = &state.redis_repository;
        let producer = &state.producer;
        let profile = EditUsernameUseCase::execute(
            &repository,
            &redis,
            &user.user_id,
            &username,
        ).await.extend()?;
        let profile_edit_event = ProfileEditEventProducer::new(producer.clone());
        profile_edit_event.send_message(AGGREGATE_PROFILE_EDIT_TOPIC, &profile).await?;
        Ok(true)
    }

    #[graphql(guard = "RoleGuard::new(vec![Role::USER])")]
    pub async fn profile_edit_names(&self, ctx: &Context<'_>, #[graphql(validator(min_length = 2, max_length = 30))] names: String) -> FieldResult<bool> {
        let user = User::get_user(ctx).extend()?;
        let state = ctx.data::<AppState>()?;
        let repository = &state.profile_repository;
        let redis = &state.redis_repository;
        let producer = &state.producer;

        let profile = EditNamesUseCase::execute(&repository, &redis, &user.user_id, &names).await.extend()?;
        let profile_edit_event = ProfileEditEventProducer::new(producer.clone());
        profile_edit_event.send_message(AGGREGATE_PROFILE_EDIT_TOPIC, &profile).await?;
        Ok(true)
    }


    #[graphql(guard = "RoleGuard::new(vec![Role::USER])")]
    pub async fn profile_edit_description(&self, ctx: &Context<'_>, #[graphql(validator(max_length = 150))] description: String) -> FieldResult<bool> {
        let user = User::get_user(ctx).extend()?;
        let state = ctx.data::<AppState>()?;
        let repository = &state.profile_repository;
        let redis = &state.redis_repository;
        let _ = EditDescriptionUseCase::execute(&repository, &redis, &user.user_id, &description).await.extend()?;
        Ok(true)
    }

    #[graphql(guard = "RoleGuard::new(vec![Role::USER])")]
    pub async fn profile_edit_privacy(&self, ctx: &Context<'_>, privacy: bool) -> FieldResult<bool> {
        let user = User::get_user(ctx).extend()?;
        let state = ctx.data::<AppState>()?;
        let repository = &state.profile_repository;
        let redis = &state.redis_repository;
        let producer = &state.producer;

        let profile = EditPrivacyUseCase::execute(&repository, &redis, &user.user_id, &privacy).await?;
        let profile_edit_event = ProfileEditEventProducer::new(producer.clone());
        profile_edit_event.send_message(AGGREGATE_PROFILE_EDIT_TOPIC, &profile).await?;
        // send user edit event
        Ok(true)
    }
}