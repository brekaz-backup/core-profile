use async_graphql::*;

#[derive(SimpleObject, Default, Clone)]
#[graphql(complex)]
pub struct Profile {
    pub id: ID,
    pub username: String,
    pub names: String,
    pub photo: Option<String>,
    pub photo_hash: Option<String>,
    pub portrait: Option<String>,
    pub portrait_hash: Option<String>,
    pub description: String,
    pub verified: bool,
    pub privacy: bool,
}


#[derive(SimpleObject, Default, Clone)]
pub struct SimpleProfile {
    pub id: ID,
    pub username: String,
    pub names: String,
    pub photo: Option<String>,
    pub photo_hash: Option<String>,
    pub verified: bool,
    pub privacy: bool,
}

#[derive(SimpleObject, Default, Clone)]
pub struct ProfileDetail {
    pub id: ID,
    pub lat: f32,
    pub lng: f32,
}