use anyhow::Result;
use crate::domain::protos::schema::profile_photo_capnp::profile_photo as ProfilePhoto;
use crate::domain::protos::schema::profile_photo_capnp::PhotoType;
use crate::domain::entities::{PhotoTypeEntity, ProfilePhotoEntity};

use uuid::Uuid;

#[derive(Default)]
pub struct ProfilePhotoEventMapper;

impl ProfilePhotoEventMapper {
    #[allow(unused)]
    pub async fn proto(entity: &ProfilePhotoEntity) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<ProfilePhoto::Builder>();
        proto.set_user_id(&entity.user_id.to_string());
        match entity.photo_type {
            PhotoTypeEntity::Profile => proto.set_photo_type(PhotoType::Profile),
            PhotoTypeEntity::Portrait =>  proto.set_photo_type(PhotoType::Portrait)
        }
        proto.set_photo(&entity.photo);
        proto.set_photo_hash(&entity.photo_hash);
        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message)?;
        return Ok(buf_slice);
    }

    pub async fn entity(payload: &[u8]) -> Result<ProfilePhotoEntity> {
        let message_reader = capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let proto = message_reader.get_root::<ProfilePhoto::Reader>()?;
        Ok(
            ProfilePhotoEntity{
                user_id: Uuid::parse_str(proto.get_user_id()?)?,
                photo_type: match proto.get_photo_type()? {
                    PhotoType::Profile => PhotoTypeEntity::Profile,
                    PhotoType::Portrait =>  PhotoTypeEntity::Portrait
                },
                photo: proto.get_photo()?.to_string(),
                photo_hash: proto.get_photo_hash()?.to_string()
            }
        )

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("info")).try_init();
    }

    #[actix_web::main]
    #[test]
    async fn test_profile_photo_event_mapper_to_proto() {
        init();
        let entity = ProfilePhotoEntity {
            user_id: Uuid::new_v4(),
            photo: "dedede".to_string(),
            photo_hash: "dede".to_string(),
            photo_type: PhotoTypeEntity::Profile
        };

        let data = ProfilePhotoEventMapper::proto(&entity).await.expect("Error getting proto");
        info!("Photo Email {:?}", data);
        let entity = ProfilePhotoEventMapper::entity(&data).await.expect("Error decoding p´rofile photo");
        info!("Photo Email {:?}", entity);
        assert_eq!(false, data.is_empty());
    }
}
