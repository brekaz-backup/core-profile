use actix_web::{guard, web, HttpRequest, HttpResponse, Result};
use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql::dataloader::DataLoader;
use async_graphql::EmptySubscription;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use async_graphql::MergedObject;
use blumer_lib_auth_rs::User;
use crate::adapters::profile::{AppDataLoader, ProfileMutation, ProfileQuery};
use super::shared::state::AppState;


#[derive(MergedObject, Default)]
pub struct Query(ProfileQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(ProfileMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(index))
            .route(
                web::get()
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .route(web::get().to(index_graphiql)),
    );
}

async fn index(
    schema: web::Data<AppSchema>,
    http_req: HttpRequest,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut query = req.into_inner();
    if let Ok(maybe_user) = User::get_user_from_headers(http_req) {
        if let Some(user) = maybe_user {
            query = query.data(user);
        }
    }

    schema.execute(query).await.into()
}

async fn index_ws(
    schema: web::Data<AppSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint(&format!(
                    "http://{}",
                    std::env::var("GRAPHQL_HOST").unwrap()
                ))
                .finish(),
        ))
}

pub fn create_schema_with_context(state: AppState, app_data_loader: DataLoader<AppDataLoader>) -> Schema<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        // limits are commented out, because otherwise introspection query won't work
        // .limit_depth(3)
        // .limit_complexity(15)
        //    .data(pool.clone())
        .data(state.clone())
        .data(app_data_loader)
        .finish()
}
