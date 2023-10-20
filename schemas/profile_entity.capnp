@0x828fdc923497fd7a;

using Rust = import "rust.capnp";
$Rust.parentModule("domain::protos::schema");

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
    portrait @5 :Text;
    portraitHash @6 :Text;
    description @7 :Text;
    verified @8 :Bool;
    privacy @9 :Bool;
}

struct ProfileDetail {
  userId @0 :Text;
  lat @1 :Float32;
  lng @2 :Float32;
  birthday @3 :Date;
}

