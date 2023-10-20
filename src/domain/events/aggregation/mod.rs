mod check_ip_event_producer;
mod profile_edit_event_producer;

pub use {
    check_ip_event_producer::CheckIpEventProducer,
    profile_edit_event_producer::ProfileEditEventProducer
};

pub const AGGREGATE_PROFILE_EDIT_TOPIC: &str = "ms-profile-edit";
pub const AGGREGATE_CHECK_IP_TOPIC: &str = "ms-ip-lookup";