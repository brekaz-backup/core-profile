#[allow(dead_code, unused)]
extern crate capnp;

#[allow(dead_code, unused)]
pub mod profile_capnp {
    include!(concat!(env!("OUT_DIR"), "/schemas/profile_capnp.rs"));
}

#[allow(dead_code, unused)]
pub mod ip_lookup_capnp {
    include!(concat!(env!("OUT_DIR"), "/schemas/ip_lookup_capnp.rs"));
}

#[allow(dead_code, unused)]
pub mod delete_profile_capnp {
    include!(concat!(env!("OUT_DIR"), "/schemas/delete_profile_capnp.rs"));
}

#[allow(dead_code, unused)]
pub mod profile_entity_capnp {
    include!(concat!(env!("OUT_DIR"), "/schemas/profile_entity_capnp.rs"));
}


#[allow(dead_code, unused)]
pub mod profile_edit_capnp {
    include!(concat!(env!("OUT_DIR"), "/schemas/profile_edit_capnp.rs"));
}

#[allow(dead_code, unused)]
pub mod profile_photo_capnp {
    include!(concat!(env!("OUT_DIR"), "/schemas/profile_photo_capnp.rs"));
}