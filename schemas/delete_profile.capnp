@0x934ef1345854ffd3;


using Rust = import "rust.capnp";
$Rust.parentModule("domain::protos::schema");


struct DeleteProfile {
  userId @0 :Text;
  reason @1 :Text;
}