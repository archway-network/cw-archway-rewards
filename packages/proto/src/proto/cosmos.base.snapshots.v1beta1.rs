// @generated
/// Snapshot contains Tendermint state sync snapshot info.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(uint32, tag = "2")]
    pub format: u32,
    #[prost(uint32, tag = "3")]
    pub chunks: u32,
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub metadata: ::core::option::Option<Metadata>,
}
/// Metadata contains SDK-specific snapshot metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// SHA-256 chunk hashes
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub chunk_hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// SnapshotItem is an item contained in a rootmulti.Store snapshot.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotItem {
    /// item is the specific type of snapshot item.
    #[prost(oneof = "snapshot_item::Item", tags = "1, 2, 3, 4, 5, 6")]
    pub item: ::core::option::Option<snapshot_item::Item>,
}
/// Nested message and enum types in `SnapshotItem`.
pub mod snapshot_item {
    /// item is the specific type of snapshot item.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        #[prost(message, tag = "1")]
        Store(super::SnapshotStoreItem),
        #[prost(message, tag = "2")]
        Iavl(super::SnapshotIavlItem),
        #[prost(message, tag = "3")]
        Extension(super::SnapshotExtensionMeta),
        #[prost(message, tag = "4")]
        ExtensionPayload(super::SnapshotExtensionPayload),
        #[prost(message, tag = "5")]
        Kv(super::SnapshotKvItem),
        #[prost(message, tag = "6")]
        Schema(super::SnapshotSchema),
    }
}
/// SnapshotStoreItem contains metadata about a snapshotted store.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotStoreItem {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// SnapshotIAVLItem is an exported IAVL node.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotIavlItem {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// version is block height
    #[prost(int64, tag = "3")]
    pub version: i64,
    /// height is depth of the tree.
    #[prost(int32, tag = "4")]
    pub height: i32,
}
/// SnapshotExtensionMeta contains metadata about an external snapshotter.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotExtensionMeta {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub format: u32,
}
/// SnapshotExtensionPayload contains payloads of an external snapshotter.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotExtensionPayload {
    #[prost(bytes = "vec", tag = "1")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// SnapshotKVItem is an exported Key/Value Pair
///
/// Since: cosmos-sdk 0.46
/// Deprecated: This message was part of store/v2alpha1 which has been deleted from v0.47.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotKvItem {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// SnapshotSchema is an exported schema of smt store
///
/// Since: cosmos-sdk 0.46
/// Deprecated: This message was part of store/v2alpha1 which has been deleted from v0.47.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotSchema {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Encoded file descriptor set for the `cosmos.base.snapshots.v1beta1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xfe, 0x1f, 0x0a, 0x2c, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x62, 0x61, 0x73, 0x65,
    0x2f, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x73, 0x2f, 0x76, 0x31, 0x62, 0x65, 0x74,
    0x61, 0x31, 0x2f, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x1d, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x73,
    0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31,
    0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x6f, 0x67, 0x6f,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xb1, 0x01, 0x0a, 0x08, 0x53, 0x6e, 0x61, 0x70, 0x73,
    0x68, 0x6f, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x66, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x73, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x06, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x68,
    0x61, 0x73, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x68, 0x61, 0x73, 0x68, 0x12,
    0x49, 0x0a, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x27, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e,
    0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61,
    0x31, 0x2e, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00,
    0x52, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x22, 0x2d, 0x0a, 0x08, 0x4d, 0x65,
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x5f,
    0x68, 0x61, 0x73, 0x68, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x0b, 0x63, 0x68,
    0x75, 0x6e, 0x6b, 0x48, 0x61, 0x73, 0x68, 0x65, 0x73, 0x22, 0x87, 0x04, 0x0a, 0x0c, 0x53, 0x6e,
    0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x49, 0x74, 0x65, 0x6d, 0x12, 0x48, 0x0a, 0x05, 0x73, 0x74,
    0x6f, 0x72, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x30, 0x2e, 0x63, 0x6f, 0x73, 0x6d,
    0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74,
    0x73, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68,
    0x6f, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x48, 0x00, 0x52, 0x05, 0x73,
    0x74, 0x6f, 0x72, 0x65, 0x12, 0x4f, 0x0a, 0x04, 0x69, 0x61, 0x76, 0x6c, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x2f, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65,
    0x2e, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74,
    0x61, 0x31, 0x2e, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x49, 0x41, 0x56, 0x4c, 0x49,
    0x74, 0x65, 0x6d, 0x42, 0x08, 0xe2, 0xde, 0x1f, 0x04, 0x49, 0x41, 0x56, 0x4c, 0x48, 0x00, 0x52,
    0x04, 0x69, 0x61, 0x76, 0x6c, 0x12, 0x54, 0x0a, 0x09, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x34, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
    0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x73,
    0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f,
    0x74, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x4d, 0x65, 0x74, 0x61, 0x48, 0x00,
    0x52, 0x09, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x66, 0x0a, 0x11, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x37, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e,
    0x62, 0x61, 0x73, 0x65, 0x2e, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x73, 0x2e, 0x76,
    0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x45,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x50, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x48,
    0x00, 0x52, 0x10, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x50, 0x61, 0x79, 0x6c,
    0x6f, 0x61, 0x64, 0x12, 0x49, 0x0a, 0x02, 0x6b, 0x76, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x2d, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x73, 0x6e,
    0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e,
    0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x4b, 0x56, 0x49, 0x74, 0x65, 0x6d, 0x42, 0x08,
    0xe2, 0xde, 0x1f, 0x02, 0x4b, 0x56, 0x18, 0x01, 0x48, 0x00, 0x52, 0x02, 0x6b, 0x76, 0x12, 0x4b,
    0x0a, 0x06, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2d,
    0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x73, 0x6e, 0x61,
    0x70, 0x73, 0x68, 0x6f, 0x74, 0x73, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x53,
    0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x42, 0x02, 0x18,
    0x01, 0x48, 0x00, 0x52, 0x06, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x42, 0x06, 0x0a, 0x04, 0x69,
    0x74, 0x65, 0x6d, 0x22, 0x27, 0x0a, 0x11, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x53,
    0x74, 0x6f, 0x72, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x6c, 0x0a, 0x10,
    0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x49, 0x41, 0x56, 0x4c, 0x49, 0x74, 0x65, 0x6d,
    0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b,
    0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x12, 0x16, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x22, 0x43, 0x0a, 0x15, 0x53, 0x6e,
    0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x4d,
    0x65, 0x74, 0x61, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x66, 0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x22,
    0x34, 0x0a, 0x18, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x45, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x50, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x70,
    0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x70, 0x61,
    0x79, 0x6c, 0x6f, 0x61, 0x64, 0x22, 0x3c, 0x0a, 0x0e, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f,
    0x74, 0x4b, 0x56, 0x49, 0x74, 0x65, 0x6d, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a,
    0x02, 0x18, 0x01, 0x22, 0x28, 0x0a, 0x0e, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x53,
    0x63, 0x68, 0x65, 0x6d, 0x61, 0x12, 0x12, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0c, 0x52, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x3a, 0x02, 0x18, 0x01, 0x42, 0x2e, 0x5a,
    0x2c, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d,
    0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x73, 0x6e,
    0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x73, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x4a, 0xf9, 0x15,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x59, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x00, 0x26, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x03, 0x00, 0x1e, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05, 0x00,
    0x43, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x05, 0x00, 0x43, 0x0a, 0x44, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x0e, 0x01, 0x1a, 0x38, 0x20, 0x53, 0x6e, 0x61, 0x70, 0x73,
    0x68, 0x6f, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x54, 0x65, 0x6e,
    0x64, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x74, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x73, 0x79,
    0x6e, 0x63, 0x20, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x20, 0x69, 0x6e, 0x66, 0x6f,
    0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x10, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x09, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x09, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a,
    0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x0b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x0b, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x02, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x02, 0x07, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0c, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06,
    0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x0d, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0d, 0x16,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x08, 0x12, 0x03, 0x0d, 0x18, 0x36, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x04, 0x08, 0xe9, 0xfb, 0x03, 0x12, 0x03, 0x0d, 0x19, 0x35,
    0x0a, 0x3f, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x11, 0x00, 0x13, 0x01, 0x1a, 0x33, 0x20, 0x4d,
    0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73,
    0x20, 0x53, 0x44, 0x4b, 0x2d, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x73, 0x6e,
    0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x11, 0x08, 0x10, 0x0a, 0x23, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x12, 0x02, 0x22, 0x22, 0x16, 0x20, 0x53, 0x48, 0x41,
    0x2d, 0x32, 0x35, 0x36, 0x20, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x20, 0x68, 0x61, 0x73, 0x68, 0x65,
    0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x11, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x20, 0x21, 0x0a, 0x67, 0x0a, 0x02, 0x04, 0x02,
    0x12, 0x04, 0x18, 0x00, 0x22, 0x01, 0x1a, 0x5b, 0x20, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f,
    0x74, 0x49, 0x74, 0x65, 0x6d, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x74, 0x65, 0x6d,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20,
    0x72, 0x6f, 0x6f, 0x74, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x20,
    0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x2e, 0x0a, 0x0a, 0x20, 0x53, 0x69, 0x6e, 0x63,
    0x65, 0x3a, 0x20, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x20, 0x30, 0x2e,
    0x34, 0x36, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x18, 0x08, 0x14, 0x0a,
    0x3b, 0x0a, 0x04, 0x04, 0x02, 0x08, 0x00, 0x12, 0x04, 0x1a, 0x02, 0x21, 0x03, 0x1a, 0x2d, 0x20,
    0x69, 0x74, 0x65, 0x6d, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63,
    0x69, 0x66, 0x69, 0x63, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x6e, 0x61,
    0x70, 0x73, 0x68, 0x6f, 0x74, 0x20, 0x69, 0x74, 0x65, 0x6d, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x08, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x1b, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x1b, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x1b, 0x1d, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x31,
    0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x04, 0x55, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x1c, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1c, 0x1d, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x1c, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x08, 0x12, 0x03, 0x1c, 0x33, 0x54, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x01, 0x08, 0xec,
    0xfb, 0x03, 0x12, 0x03, 0x1c, 0x34, 0x53, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x1d, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1d,
    0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x1d, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1d, 0x31, 0x32, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1e, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x06, 0x12, 0x03, 0x1e, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x1e, 0x1d, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x1e, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1f,
    0x04, 0x66, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x06, 0x12, 0x03, 0x1f, 0x04, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1f, 0x1d, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1f, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x04, 0x08, 0x12, 0x03, 0x1f, 0x33, 0x65, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02,
    0x02, 0x04, 0x08, 0x03, 0x12, 0x03, 0x1f, 0x34, 0x45, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02,
    0x04, 0x08, 0xec, 0xfb, 0x03, 0x12, 0x03, 0x1f, 0x47, 0x64, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x20, 0x04, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x06,
    0x12, 0x03, 0x20, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x20, 0x1d, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x20, 0x31,
    0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x08, 0x12, 0x03, 0x20, 0x33, 0x46, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x02, 0x02, 0x05, 0x08, 0x03, 0x12, 0x03, 0x20, 0x34, 0x45, 0x0a, 0x65,
    0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x27, 0x00, 0x29, 0x01, 0x1a, 0x59, 0x20, 0x53, 0x6e, 0x61,
    0x70, 0x73, 0x68, 0x6f, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x20, 0x63,
    0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61,
    0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x61, 0x20, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f,
    0x74, 0x74, 0x65, 0x64, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x0a, 0x0a, 0x20, 0x53, 0x69,
    0x6e, 0x63, 0x65, 0x3a, 0x20, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x20,
    0x30, 0x2e, 0x34, 0x36, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x27, 0x08,
    0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x28, 0x02, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x28, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x28, 0x10, 0x11, 0x0a, 0x51, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04,
    0x2e, 0x00, 0x35, 0x01, 0x1a, 0x45, 0x20, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x49,
    0x41, 0x56, 0x4c, 0x49, 0x74, 0x65, 0x6d, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x78,
    0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x49, 0x41, 0x56, 0x4c, 0x20, 0x6e, 0x6f, 0x64, 0x65,
    0x2e, 0x0a, 0x0a, 0x20, 0x53, 0x69, 0x6e, 0x63, 0x65, 0x3a, 0x20, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
    0x73, 0x2d, 0x73, 0x64, 0x6b, 0x20, 0x30, 0x2e, 0x34, 0x36, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x04, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12,
    0x03, 0x2f, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2f,
    0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x0b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2f, 0x10, 0x11, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x30, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x30, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x30, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x30, 0x10, 0x11, 0x0a, 0x26, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x32,
    0x02, 0x14, 0x1a, 0x19, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20,
    0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x32, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x32, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x32, 0x12, 0x13, 0x0a, 0x2b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12,
    0x03, 0x34, 0x02, 0x13, 0x1a, 0x1e, 0x20, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x20, 0x69, 0x73,
    0x20, 0x64, 0x65, 0x70, 0x74, 0x68, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72,
    0x65, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05, 0x12, 0x03, 0x34,
    0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x34, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x34, 0x11, 0x12, 0x0a, 0x6d,
    0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x3a, 0x00, 0x3d, 0x01, 0x1a, 0x61, 0x20, 0x53, 0x6e, 0x61,
    0x70, 0x73, 0x68, 0x6f, 0x74, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x4d, 0x65,
    0x74, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x6d, 0x65, 0x74, 0x61,
    0x64, 0x61, 0x74, 0x61, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x78,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x74,
    0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x20, 0x53, 0x69, 0x6e, 0x63, 0x65, 0x3a, 0x20, 0x63, 0x6f, 0x73,
    0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x20, 0x30, 0x2e, 0x34, 0x36, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x3b, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x3b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3b,
    0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3b, 0x12, 0x13,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x3c, 0x02, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3c, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3c, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x3c, 0x12, 0x13, 0x0a, 0x6d, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x42,
    0x00, 0x44, 0x01, 0x1a, 0x61, 0x20, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x45, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x50, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x63,
    0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x73,
    0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20,
    0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x20, 0x53,
    0x69, 0x6e, 0x63, 0x65, 0x3a, 0x20, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b,
    0x20, 0x30, 0x2e, 0x34, 0x36, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x42,
    0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x43, 0x02, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x43, 0x02, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x43, 0x12, 0x13, 0x0a, 0xac, 0x01, 0x0a, 0x02, 0x04, 0x07,
    0x12, 0x04, 0x4a, 0x00, 0x4f, 0x01, 0x1a, 0x9f, 0x01, 0x20, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68,
    0x6f, 0x74, 0x4b, 0x56, 0x49, 0x74, 0x65, 0x6d, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x65,
    0x78, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x4b, 0x65, 0x79, 0x2f, 0x56, 0x61, 0x6c, 0x75,
    0x65, 0x20, 0x50, 0x61, 0x69, 0x72, 0x0a, 0x0a, 0x20, 0x53, 0x69, 0x6e, 0x63, 0x65, 0x3a, 0x20,
    0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x20, 0x30, 0x2e, 0x34, 0x36, 0x0a,
    0x20, 0x44, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x3a, 0x20, 0x54, 0x68, 0x69,
    0x73, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x77, 0x61, 0x73, 0x20, 0x70, 0x61,
    0x72, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2f, 0x76, 0x32, 0x61, 0x6c,
    0x70, 0x68, 0x61, 0x31, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x68, 0x61, 0x73, 0x20, 0x62,
    0x65, 0x65, 0x6e, 0x20, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d,
    0x20, 0x76, 0x30, 0x2e, 0x34, 0x37, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12,
    0x03, 0x4a, 0x08, 0x16, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x07, 0x12, 0x03, 0x4b, 0x02, 0x1b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x07, 0x03, 0x12, 0x03, 0x4b, 0x02, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x4d, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x4d, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x4d, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x4d, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x4e, 0x02,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4e, 0x02, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4e, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4e, 0x10, 0x11, 0x0a, 0xb1, 0x01, 0x0a, 0x02,
    0x04, 0x08, 0x12, 0x04, 0x55, 0x00, 0x59, 0x01, 0x1a, 0xa4, 0x01, 0x20, 0x53, 0x6e, 0x61, 0x70,
    0x73, 0x68, 0x6f, 0x74, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e,
    0x20, 0x65, 0x78, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61,
    0x20, 0x6f, 0x66, 0x20, 0x73, 0x6d, 0x74, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x0a, 0x0a, 0x20,
    0x53, 0x69, 0x6e, 0x63, 0x65, 0x3a, 0x20, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64,
    0x6b, 0x20, 0x30, 0x2e, 0x34, 0x36, 0x0a, 0x20, 0x44, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74,
    0x65, 0x64, 0x3a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x20, 0x77, 0x61, 0x73, 0x20, 0x70, 0x61, 0x72, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x74, 0x6f,
    0x72, 0x65, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x31, 0x20, 0x77, 0x68, 0x69, 0x63,
    0x68, 0x20, 0x68, 0x61, 0x73, 0x20, 0x62, 0x65, 0x65, 0x6e, 0x20, 0x64, 0x65, 0x6c, 0x65, 0x74,
    0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x76, 0x30, 0x2e, 0x34, 0x37, 0x2e, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x55, 0x08, 0x16, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x08, 0x07, 0x12, 0x03, 0x56, 0x02, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x07, 0x03, 0x12,
    0x03, 0x56, 0x02, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x58, 0x02,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x58, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x58, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x58, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x58, 0x18, 0x19, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x33,
];
// @@protoc_insertion_point(module)
