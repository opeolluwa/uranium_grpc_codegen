#[path = "./client/mod.rs"]
#[cfg(feature="client")]
pub mod client_stub;

#[cfg(feature="server")]
#[path = "./server/mod.rs"]
pub mod server_stub;

