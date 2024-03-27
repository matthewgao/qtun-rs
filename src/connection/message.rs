#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Envelope {
    #[prost(oneof = "envelope::Type", tags = "1, 2")]
    pub r#type: ::core::option::Option<envelope::Type>,
}
/// Nested message and enum types in `Envelope`.
pub mod envelope {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag = "1")]
        Ping(super::MessagePing),
        #[prost(message, tag = "2")]
        Packet(super::MessagePacket),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagePing {
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    #[prost(string, tag = "2")]
    pub local_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub local_private_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub ip: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub dc: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagePacket {
    #[prost(bytes = "vec", tag = "1")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
