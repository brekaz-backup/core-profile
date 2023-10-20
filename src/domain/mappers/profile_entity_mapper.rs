use anyhow::Result;


use uuid::Uuid;
use crate::domain::entities::ProfileEntity;
use crate::domain::protos::schema::profile_entity_capnp::profile as ProfileProto;

pub struct ProfileEntityMapper;

impl ProfileEntityMapper {
    pub async fn proto(entity: &ProfileEntity) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<ProfileProto::Builder>();
        proto.set_user_id(&entity.user_id.to_string());
        proto.set_username(&entity.username);
        proto.set_names(&entity.names);
        if let Some(photo) = &entity.photo {
            proto.set_photo(&photo);
        }

        if let Some(hash) = &entity.photo_hash {
            proto.set_photo_hash(&hash);
        }
        
        if let Some(portrait) = &entity.portrait {
            proto.set_portrait(&portrait);
        }

        if let Some(portrait_hash) = &entity.portrait_hash {
            proto.set_portrait_hash(&portrait_hash);
        }
        proto.set_description(&entity.description);
        proto.set_verified(entity.verified);
        proto.set_privacy(entity.privacy);
        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message).unwrap();
        return Ok(buf_slice);
    }


    pub async fn entity(payload: &[u8]) -> Result<ProfileEntity> {
        let message_reader = capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let profile = message_reader.get_root::<ProfileProto::Reader>()?;
        Ok(
            ProfileEntity {
                user_id: Uuid::parse_str(profile.get_user_id()?)?,
                username: profile.get_username()?.to_string(),
                names: profile.get_names()?.to_string(),
                photo: if profile.has_photo() { Some(profile.get_photo()?.to_string()) } else { None },
                photo_hash: if profile.has_photo_hash() { Some(profile.get_photo_hash()?.to_string()) } else { None },
                portrait: if profile.has_portrait() { Some(profile.get_portrait()?.to_string()) } else { None },
                portrait_hash: if profile.has_portrait_hash() { Some(profile.get_portrait_hash()?.to_string()) } else { None },
                description: profile.get_description()?.to_string(),
                verified: profile.get_verified(),
                privacy: profile.get_privacy(),
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
        let entity = ProfileEntity {
            user_id: Uuid::new_v4(),
            username: "demo".to_string(),
            names: "demo".to_string(),
            photo: None,
            photo_hash: None,
            portrait: None,
            portrait_hash: None,
            description: "".to_string(),
            verified: false,
            privacy: false,
            is_active: false
        };

        let data = ProfileEntityMapper::proto(&entity).await.expect("Error getting proto");
        info!("Create Profile {:?}", data);
        let entity = ProfileEntityMapper::entity(&data).await.expect("Cant read Profile proto");
        info!("Get Profile {:?}", entity);
        assert_eq!(false, data.is_empty());
    }
}