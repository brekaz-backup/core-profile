mod profile_event_consumer;


pub use profile_event_consumer::ProfileEventConsumer;

pub const REDUCER_CREATE_PROFILE_TOPIC: &str = "ms-profile-create";
pub const REDUCER_REACTION_TOPIC: &str = "ms-post-reaction-post_reaction";
pub const REDUCER_PROFILE_PHOTO_TOPIC: &str = "ms-profile-photo";
pub const REDUCER_VERIFY_TOPIC: &str = "ms-admin-api-verify-user";


