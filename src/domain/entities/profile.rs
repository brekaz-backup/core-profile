use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct CreateProfileEntity {
    pub user_id: Uuid,
    pub username: String,
    pub names: String,
    pub photo: Option<String>,
    pub photo_hash: Option<String>,
    pub portrait: Option<String>,
    pub portrait_hash: Option<String>,
    pub birthday: NaiveDate,
    pub description: String,
    pub verified: bool,
    pub privacy: bool,
    pub lat: f32,
    pub lng: f32,
    pub is_active: bool,
}


#[derive(Clone, Debug)]
pub enum PhotoTypeEntity {
    Profile,
    Portrait
}

#[derive(Clone, Debug)]
pub struct ProfilePhotoEntity {
    pub user_id: Uuid,
    pub photo_type: PhotoTypeEntity,
    pub photo: String,
    pub photo_hash: String,
}

#[derive(Debug, scylla::FromRow)]
pub struct ProfileEntity {
    pub user_id: Uuid,
    pub username: String,
    pub names: String,
    pub photo: Option<String>,
    pub photo_hash: Option<String>,
    pub portrait: Option<String>,
    pub portrait_hash: Option<String>,
    pub description: String,
    pub verified: bool,
    pub privacy: bool,
    pub is_active: bool,
}

#[derive(Debug, scylla::FromRow)]
pub struct ProfileDetailsEntity {
    pub user_id: Uuid,
    pub birthday: NaiveDate,
    pub lat: f32,
    pub lng: f32,
}

#[derive(Clone, Debug)]
pub struct DeleteProfileEntity {
    pub user_id: Uuid,
    pub reason: String,
}