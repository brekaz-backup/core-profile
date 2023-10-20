@0xeefdf1ff4054ffd3;


using Rust = import "rust.capnp";
$Rust.parentModule("domain::protos::schema");

enum PhotoType {
  profile @0;
  portrait @1;
}

struct ProfilePhoto {
  userId @0 :Text;
  photoType @1 :PhotoType;
  photo @2 :Text;
  photoHash @3 :Text;
}