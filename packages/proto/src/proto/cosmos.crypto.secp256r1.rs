// @generated
/// PubKey defines a secp256r1 ECDSA public key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKey {
    /// Point on secp256r1 curve in a compressed representation as specified in section
    /// 4.3.6 of ANSI X9.62: <https://webstore.ansi.org/standards/ascx9/ansix9621998>
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for PubKey {
    const NAME: &'static str = "PubKey";
    const PACKAGE: &'static str = "cosmos.crypto.secp256r1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.crypto.secp256r1.{}", Self::NAME)
    }
}
/// PrivKey defines a secp256r1 ECDSA private key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivKey {
    /// secret number serialized using big-endian encoding
    #[prost(bytes = "vec", tag = "1")]
    pub secret: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for PrivKey {
    const NAME: &'static str = "PrivKey";
    const PACKAGE: &'static str = "cosmos.crypto.secp256r1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.crypto.secp256r1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `cosmos.crypto.secp256r1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xaa, 0x07, 0x0a, 0x22, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x72, 0x79, 0x70,
    0x74, 0x6f, 0x2f, 0x73, 0x65, 0x63, 0x70, 0x32, 0x35, 0x36, 0x72, 0x31, 0x2f, 0x6b, 0x65, 0x79,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x17, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e,
    0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x2e, 0x73, 0x65, 0x63, 0x70, 0x32, 0x35, 0x36, 0x72, 0x31,
    0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x6f, 0x67, 0x6f,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x27, 0x0a, 0x06, 0x50, 0x75, 0x62, 0x4b, 0x65, 0x79,
    0x12, 0x1d, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x42, 0x0b, 0xda,
    0xde, 0x1f, 0x07, 0x65, 0x63, 0x64, 0x73, 0x61, 0x50, 0x4b, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x22,
    0x2e, 0x0a, 0x07, 0x50, 0x72, 0x69, 0x76, 0x4b, 0x65, 0x79, 0x12, 0x23, 0x0a, 0x06, 0x73, 0x65,
    0x63, 0x72, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x42, 0x0b, 0xda, 0xde, 0x1f, 0x07,
    0x65, 0x63, 0x64, 0x73, 0x61, 0x53, 0x4b, 0x52, 0x06, 0x73, 0x65, 0x63, 0x72, 0x65, 0x74, 0x42,
    0x40, 0xc8, 0xe1, 0x1e, 0x00, 0xd8, 0xe1, 0x1e, 0x00, 0xc8, 0xe3, 0x1e, 0x01, 0x5a, 0x32, 0x67,
    0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
    0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x63, 0x72, 0x79, 0x70,
    0x74, 0x6f, 0x2f, 0x6b, 0x65, 0x79, 0x73, 0x2f, 0x73, 0x65, 0x63, 0x70, 0x32, 0x35, 0x36, 0x72,
    0x31, 0x4a, 0xb1, 0x05, 0x0a, 0x06, 0x12, 0x04, 0x01, 0x00, 0x16, 0x01, 0x0a, 0x22, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x01, 0x00, 0x12, 0x1a, 0x18, 0x20, 0x53, 0x69, 0x6e, 0x63, 0x65, 0x3a, 0x20,
    0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x20, 0x30, 0x2e, 0x34, 0x33, 0x0a,
    0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x20, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00,
    0x12, 0x03, 0x04, 0x00, 0x1e, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x5f, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x06, 0x00, 0x5f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x07, 0x00, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xb9, 0xec, 0x03, 0x12, 0x03, 0x07, 0x00,
    0x2f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x08,
    0x9b, 0xec, 0x03, 0x12, 0x03, 0x08, 0x00, 0x30, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09,
    0x00, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0x99, 0xec, 0x03, 0x12, 0x03, 0x09, 0x00, 0x30, 0x0a,
    0x3a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0c, 0x00, 0x10, 0x01, 0x1a, 0x2e, 0x20, 0x50, 0x75,
    0x62, 0x4b, 0x65, 0x79, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x61, 0x20, 0x73,
    0x65, 0x63, 0x70, 0x32, 0x35, 0x36, 0x72, 0x31, 0x20, 0x45, 0x43, 0x44, 0x53, 0x41, 0x20, 0x70,
    0x75, 0x62, 0x6c, 0x69, 0x63, 0x20, 0x6b, 0x65, 0x79, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x0e, 0x0a, 0xac, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x0f, 0x02, 0x35, 0x1a, 0x9e, 0x01, 0x20, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x6f,
    0x6e, 0x20, 0x73, 0x65, 0x63, 0x70, 0x32, 0x35, 0x36, 0x72, 0x31, 0x20, 0x63, 0x75, 0x72, 0x76,
    0x65, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65,
    0x64, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x61, 0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x69, 0x6e,
    0x20, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x34, 0x2e, 0x33, 0x2e, 0x36, 0x20,
    0x6f, 0x66, 0x20, 0x41, 0x4e, 0x53, 0x49, 0x20, 0x58, 0x39, 0x2e, 0x36, 0x32, 0x3a, 0x20, 0x68,
    0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x77, 0x65, 0x62, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e,
    0x61, 0x6e, 0x73, 0x69, 0x2e, 0x6f, 0x72, 0x67, 0x2f, 0x73, 0x74, 0x61, 0x6e, 0x64, 0x61, 0x72,
    0x64, 0x73, 0x2f, 0x61, 0x73, 0x63, 0x78, 0x39, 0x2f, 0x61, 0x6e, 0x73, 0x69, 0x78, 0x39, 0x36,
    0x32, 0x31, 0x39, 0x39, 0x38, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x0f, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f,
    0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x0e, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x0f, 0x10, 0x34, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xeb, 0xfb, 0x03, 0x12, 0x03, 0x0f, 0x11, 0x33, 0x0a,
    0x3c, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x13, 0x00, 0x16, 0x01, 0x1a, 0x30, 0x20, 0x50, 0x72,
    0x69, 0x76, 0x4b, 0x65, 0x79, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x61, 0x20,
    0x73, 0x65, 0x63, 0x70, 0x32, 0x35, 0x36, 0x72, 0x31, 0x20, 0x45, 0x43, 0x44, 0x53, 0x41, 0x20,
    0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x13, 0x08, 0x0f, 0x0a, 0x41, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x15, 0x02, 0x38, 0x1a, 0x34, 0x20, 0x73, 0x65, 0x63, 0x72, 0x65, 0x74, 0x20,
    0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65,
    0x64, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x62, 0x69, 0x67, 0x2d, 0x65, 0x6e, 0x64, 0x69,
    0x61, 0x6e, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x15, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x15, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x08, 0x12,
    0x03, 0x15, 0x13, 0x37, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x00, 0x08, 0xeb, 0xfb, 0x03,
    0x12, 0x03, 0x15, 0x14, 0x36, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
