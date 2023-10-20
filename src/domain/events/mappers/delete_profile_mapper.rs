use crate::domain::protos::schema::delete_profile_capnp::delete_profile as DeleteProfile;
use anyhow::Result;
use uuid::Uuid;
use crate::domain::entities::DeleteProfileEntity;


#[derive(Default)]
pub struct DeleteProfileEventMapper;

impl DeleteProfileEventMapper {
    pub async fn proto(entity: &DeleteProfileEntity) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<DeleteProfile::Builder>();
        proto.set_user_id(&entity.user_id.to_string());
        proto.set_reason(&entity.reason);
        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message).unwrap();
        return Ok(buf_slice);
    }

    pub async fn read(payload: &[u8]) -> Result<DeleteProfileEntity> {
        let message_reader = capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let reader = message_reader.get_root::<DeleteProfile::Reader>()?;

        Ok(
            DeleteProfileEntity {
                user_id: Uuid::parse_str(reader.get_user_id()?)?,
                reason: reader.get_reason()?.to_string(),
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
    async fn test_edit_geo_mapper_proto() {
        init();
        let entity = DeleteProfileEntity {
            user_id: Uuid::new_v4(),
            reason: "Fraud".to_string(),
        };

        let data = DeleteProfileEventMapper::proto(&entity).await.expect("Error getting proto");
        let values = DeleteProfileEventMapper::read(&data).await.expect("Error reading proto");
        info!("DeleteProfile {:?}", values);
        assert_eq!(false, values.reason.is_empty());
    }
}