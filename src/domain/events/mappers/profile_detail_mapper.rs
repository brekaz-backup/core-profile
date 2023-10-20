use anyhow::Result;
use chrono::{Datelike, NaiveDate};
use uuid::Uuid;
use num_traits::ToPrimitive;
use crate::domain::protos::schema::profile_entity_capnp::profile_detail as ProfileDetailProto;
use crate::domain::entities::ProfileDetailsEntity;


#[derive(Default)]
pub struct ProfileDetailMapper;

impl ProfileDetailMapper {
    pub async fn proto(entity: &ProfileDetailsEntity) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<ProfileDetailProto::Builder>();
        proto.set_user_id(&entity.user_id.to_string());
        let mut date = proto.reborrow().init_birthday();
        date.set_year(entity.birthday.year().to_i16().unwrap());
        date.set_month(entity.birthday.month().to_u8().unwrap());
        date.set_day(entity.birthday.day().to_u8().unwrap());
        proto.set_lat(entity.lat);
        proto.set_lng(entity.lng);
        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message).unwrap();
        return Ok(buf_slice);
    }


    pub async fn entity(payload: &[u8]) -> Result<ProfileDetailsEntity> {
        let message_reader = capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let profile = message_reader.get_root::<ProfileDetailProto::Reader>()?;
        let birthday = profile.get_birthday()?;
        Ok(
            ProfileDetailsEntity {
                user_id: Uuid::parse_str(profile.get_user_id()?)?,
                birthday: NaiveDate::from_ymd_opt(birthday.get_year().into(), birthday.get_month().into(), birthday.get_day().into()).unwrap_or_default(),
                lat: profile.get_lat(),
                lng: profile.get_lng(),
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
    async fn test_profile_detail_mapper_to_proto() {
        init();
        let entity = ProfileDetailsEntity {
            user_id: Uuid::new_v4(),
            birthday: NaiveDate::default(),
            lat: 4.0,
            lng: -5.0,
        };

        let data = ProfileDetailMapper::proto(&entity).await.expect("Error getting proto");
        info!("Create Profile {:?}", data);
        let entity = ProfileDetailMapper::entity(&data).await.expect("Cant read Profile proto");
        info!("Get Profile {:?}", entity);
        assert_eq!(false, data.is_empty());
    }
}