use anyhow::Result;
use chrono::{Datelike, NaiveDate};
use uuid::Uuid;
use num_traits::ToPrimitive;
use crate::domain::protos::schema::profile_capnp::profile as ProfileProto;
use crate::domain::entities::CreateProfileEntity;


#[derive(Default)]
pub struct ProfileEventMapper;

impl ProfileEventMapper {
    pub async fn proto(entity: &CreateProfileEntity) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<ProfileProto::Builder>();
        proto.set_user_id(&entity.user_id.to_string());
        proto.set_username(&entity.username);
        proto.set_names(&entity.names);
        if let Some(photo) = &entity.photo {
            proto.set_photo(&photo);
        }
        if let Some(photo_hash) = &entity.photo_hash {
            proto.set_photo_hash(&photo_hash);
        }
        let mut date = proto.reborrow().init_birthday();
        date.set_year(entity.birthday.year().to_i16().unwrap());
        date.set_month(entity.birthday.month().to_u8().unwrap());
        date.set_day(entity.birthday.day().to_u8().unwrap());

        proto.set_verified(false);
        proto.set_privacy(false);

        let mut proto_geo = proto.init_geo();
        proto_geo.set_lat(entity.lat);
        proto_geo.set_lng(entity.lng);
        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message).unwrap();
        return Ok(buf_slice);
    }


    pub async fn entity(payload: &[u8]) -> Result<CreateProfileEntity> {
        let message_reader = capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let profile = message_reader.get_root::<ProfileProto::Reader>()?;
        let birthday = profile.get_birthday()?;
        let geo = profile.get_geo()?;
        Ok(
            CreateProfileEntity {
                user_id: Uuid::parse_str(profile.get_user_id()?)?,
                username: profile.get_username()?.to_string(),
                names: profile.get_names()?.to_string(),
                photo: if profile.has_photo() { Some(profile.get_photo()?.to_string()) } else { None },
                photo_hash:  if profile.has_photo_hash() { Some(profile.get_photo_hash()?.to_string()) } else { None },
                portrait: None,
                portrait_hash: None,
                birthday: NaiveDate::from_ymd_opt(birthday.get_year().into(), birthday.get_month().into(), birthday.get_day().into()).unwrap_or_default(),
                description: String::new(),
                verified: profile.get_verified(),
                privacy: profile.get_privacy(),
                lat: geo.get_lat(),
                lng: geo.get_lng(),
                is_active: true
            }
        )
    }
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use uuid::Uuid;
    use super::*;

    fn init() {
        // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
        let _ = env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("info")).try_init();
    }

    #[actix_web::main]
    #[test]
    async fn test_create_profile_event_mapper_to_proto() {
        init();
        let entity = CreateProfileEntity {
            user_id: Uuid::new_v4(),
            username: "demo".to_string(),
            names: "demo".to_string(),
            photo: None,
            photo_hash: None,
            portrait: None,
            portrait_hash: None,
            birthday: NaiveDate::default(),
            description: "".to_string(),
            verified: false,
            privacy: false,
            lat: 4.0,
            lng: -5.0,
            is_active: true
        };

        let data = ProfileEventMapper::proto(&entity).await.expect("Error getting proto");
        info!("Create Profile {:?}", data);
        let entity = ProfileEventMapper::entity(&data).await.expect("Cant read Profile proto");
        info!("Get Profile {:?}", entity);
        assert_eq!(false, data.is_empty());
    }
}