// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Infos {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Info>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Info {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub decimal: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
