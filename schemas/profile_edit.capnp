@0x828ffd223497fd7a;

using Rust = import "rust.capnp";
$Rust.parentModule("domain::protos::schema");

struct ProfileEdit {
  userId @0 :Text;
  username @1 :Text;
  names @2 :Text;
  photo @3 :Text;
  photoHash @4 :Text;
  verified @5 :Bool;
  privacy @6 :Bool;
}

struct VerifyUser {
  userId @0 :Text;
  status @1 :Bool;
}