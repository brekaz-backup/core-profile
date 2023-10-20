# check_all.capnp
@0x934efea77054ffd3;


using Rust = import "rust.capnp";
$Rust.parentModule("domain::protos::schema");

struct IpLookup {
  userId @0 :Text;
  ip @1 :Text;
  checkSecurity @2 :Bool;
}