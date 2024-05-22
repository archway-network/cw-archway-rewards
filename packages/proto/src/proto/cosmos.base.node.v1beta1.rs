// @generated
/// ConfigRequest defines the request structure for the Config gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRequest {}
impl ::prost::Name for ConfigRequest {
    const NAME: &'static str = "ConfigRequest";
    const PACKAGE: &'static str = "cosmos.base.node.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.base.node.v1beta1.{}", Self::NAME)
    }
}
/// ConfigResponse defines the response structure for the Config gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigResponse {
    #[prost(string, tag = "1")]
    pub minimum_gas_price: ::prost::alloc::string::String,
}
impl ::prost::Name for ConfigResponse {
    const NAME: &'static str = "ConfigResponse";
    const PACKAGE: &'static str = "cosmos.base.node.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.base.node.v1beta1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `cosmos.base.node.v1beta1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x9e, 0x07, 0x0a, 0x24, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x62, 0x61, 0x73, 0x65,
    0x2f, 0x6e, 0x6f, 0x64, 0x65, 0x2f, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2f, 0x71, 0x75,
    0x65, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x18, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
    0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x76, 0x31, 0x62, 0x65,
    0x74, 0x61, 0x31, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f,
    0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0x0f, 0x0a, 0x0d, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x22, 0x3c, 0x0a, 0x0e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2a, 0x0a, 0x11, 0x6d, 0x69, 0x6e, 0x69, 0x6d, 0x75, 0x6d, 0x5f,
    0x67, 0x61, 0x73, 0x5f, 0x70, 0x72, 0x69, 0x63, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0f, 0x6d, 0x69, 0x6e, 0x69, 0x6d, 0x75, 0x6d, 0x47, 0x61, 0x73, 0x50, 0x72, 0x69, 0x63, 0x65,
    0x32, 0x91, 0x01, 0x0a, 0x07, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x85, 0x01, 0x0a,
    0x06, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x27, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
    0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74,
    0x61, 0x31, 0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x1a, 0x28, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x6e,
    0x6f, 0x64, 0x65, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x28, 0x82, 0xd3, 0xe4, 0x93,
    0x02, 0x22, 0x12, 0x20, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x62, 0x61, 0x73, 0x65,
    0x2f, 0x6e, 0x6f, 0x64, 0x65, 0x2f, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2f, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x42, 0x2f, 0x5a, 0x2d, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63,
    0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
    0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2f, 0x67, 0x72, 0x70, 0x63,
    0x2f, 0x6e, 0x6f, 0x64, 0x65, 0x4a, 0xa1, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x15, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x01, 0x00, 0x21, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x00, 0x26, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05, 0x00, 0x44, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12,
    0x03, 0x05, 0x00, 0x44, 0x0a, 0x50, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x08, 0x00, 0x0d, 0x01,
    0x1a, 0x44, 0x20, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e,
    0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x71, 0x75, 0x65, 0x72,
    0x69, 0x65, 0x72, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x6e, 0x6f, 0x64, 0x65, 0x20, 0x72, 0x65, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x20, 0x71, 0x75, 0x65,
    0x72, 0x69, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x08,
    0x08, 0x0f, 0x0a, 0x3e, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x04, 0x0a, 0x02, 0x0c, 0x03,
    0x1a, 0x30, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x20, 0x71, 0x75, 0x65, 0x72, 0x69, 0x65,
    0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74,
    0x6f, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x06, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0a, 0x0d, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x25, 0x33, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x04, 0x46, 0x0a, 0x11, 0x0a, 0x0a, 0x06, 0x00,
    0x02, 0x00, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x02, 0x12, 0x03, 0x0b, 0x04, 0x46, 0x0a, 0x53, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x03, 0x10, 0x00, 0x18, 0x1a, 0x48, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x73, 0x74, 0x72,
    0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79,
    0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x10, 0x08, 0x15, 0x0a, 0x56,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x13, 0x00, 0x15, 0x01, 0x1a, 0x4a, 0x20, 0x43, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69,
    0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x71,
    0x75, 0x65, 0x72, 0x79, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x13,
    0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x14, 0x02, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x14, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x09, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x14, 0x1d, 0x1e, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x33,
];
include!("cosmos.base.node.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
