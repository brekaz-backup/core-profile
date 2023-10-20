use anyhow::Result;



use crate::domain::protos::schema::profile_edit_capnp::profile_edit as ProfileEditProto;
use crate::domain::entities::ProfileEntity;


#[derive(Default)]
pub struct ProfileEditEventMapper;

impl ProfileEditEventMapper {
    pub async fn proto(entity: &ProfileEntity) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<ProfileEditProto::Builder>();
        proto.set_user_id(&entity.user_id.to_string());
        proto.set_username(&entity.username);
        proto.set_names(&entity.names);
        if let Some(photo) = &entity.photo {
            proto.set_photo(&photo);
        }
        if let Some(photo_hash) = &entity.photo_hash {
            proto.set_photo_hash(&photo_hash);
        }

        proto.set_verified(entity.verified);
        proto.set_privacy(entity.privacy);

        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message).unwrap();
        return Ok(buf_slice);
    }


    pub async fn entity(payload: &[u8]) -> Result<()> {
        let message_reader = capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let _ = message_reader.get_root::<ProfileEditProto::Reader>()?;
        Ok(())
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
    async fn test_create_profile_edit_event_mapper_to_proto() {
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
            is_active: true
        };

        let data = ProfileEditEventMapper::proto(&entity).await.expect("Error getting proto");
        info!("Create Profile {:?}", data);
        ProfileEditEventMapper::entity(&data).await.expect("Cant read Profile proto");
        assert_eq!(false, data.is_empty());
    }
}