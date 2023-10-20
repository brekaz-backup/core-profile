@0x828fdc923497447a;

using Rust = import "rust.capnp";
$Rust.parentModule("domain::protos::schema");

struct Geo {
  lat @0 :Float32;
  lng @1 :Float32;
}

struct Date {
  year @0 :Int16;
  month @1 :UInt8;
  day @2 :UInt8;
}

struct Profile {
  userId @0 :Text;
  username @1 :Text;
  names @2 :Text;
  photo @3 :Text;
  photoHash @4 :Text;
  birthday @5 :Date;
  verified @6 :Bool;
  privacy @7 :Bool;
  geo @8 :Geo;
}

