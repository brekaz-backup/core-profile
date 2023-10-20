extern crate capnpc;

fn main() {
    ::capnpc::CompilerCommand::new()
        .file("schemas/rust.capnp")
        .file("schemas/profile.capnp")
        .file("schemas/delete_profile.capnp")
        .file("schemas/profile_entity.capnp")
        .file("schemas/profile_edit.capnp")
        .file("schemas/ip_lookup.capnp")
        .file("schemas/profile_photo.capnp")
        .run()
        .expect("Error compiling schema");
}
