use uuid::Uuid;

#[derive(Debug)]
pub struct PhoneEntity {
    pub code: String,
    pub phone: String,
}

#[derive(Debug)]
pub struct EmailEntity {
    pub email: String,
}


#[derive(Debug)]
pub struct Geo {
    pub lat: f32,
    pub lng: f32,
}

#[derive(Debug)]
pub struct EditGeoEntity {
    pub user_id: Uuid,
    pub lat: f32,
    pub lng: f32,
}

#[derive(Debug)]
pub struct Location {
    pub city: String,
    pub region: String,
    pub country: String,
    pub timezone: String,
}

#[derive(Debug)]
pub struct IpDataEntity {
    pub user_id: Uuid,
    pub ip: String,
    pub valid: bool,
    pub geo: Option<Geo>,
    pub location: Option<Location>,
}