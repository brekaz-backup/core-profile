use crate::domain::protos::schema::ip_lookup_capnp::ip_lookup as IpLookupProto;
use anyhow::Result;
use uuid::Uuid;

#[derive(Default)]
pub struct CheckIpEventMapper;

impl CheckIpEventMapper {
    pub async fn proto(user_id: &Uuid, ip: &String) -> Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut proto = message.init_root::<IpLookupProto::Builder>();
        proto.set_user_id(&user_id.to_string());
        proto.set_ip(&ip);
        proto.set_check_security(true);
        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message).unwrap();
        return Ok(buf_slice);
    }
}


#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use super::*;

    fn init() {
        let _ = env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("info")).try_init();
    }

    #[actix_web::main]
    #[test]
    async fn test_check_ip_event_mapper_to_proto() {
        init();
        let data = CheckIpEventMapper::proto(&Uuid::new_v4(), &String::from("127.0.0.1")).await.expect("Error getting proto");
        info!("Check All Email {:?}", data);
        assert_eq!(false, data.is_empty());
    }
}