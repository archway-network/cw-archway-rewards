// @generated
/// CommitInfo defines commit information used by the multi-store when committing
/// a version/height.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitInfo {
    #[prost(int64, tag = "1")]
    pub version: i64,
    #[prost(message, repeated, tag = "2")]
    pub store_infos: ::prost::alloc::vec::Vec<StoreInfo>,
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
impl ::prost::Name for CommitInfo {
    const NAME: &'static str = "CommitInfo";
    const PACKAGE: &'static str = "cosmos.base.store.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.base.store.v1beta1.{}", Self::NAME)
    }
}
/// StoreInfo defines store-specific commit information. It contains a reference
/// between a store name and the commit ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub commit_id: ::core::option::Option<CommitId>,
}
impl ::prost::Name for StoreInfo {
    const NAME: &'static str = "StoreInfo";
    const PACKAGE: &'static str = "cosmos.base.store.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.base.store.v1beta1.{}", Self::NAME)
    }
}
/// CommitID defines the commitment information when a specific store is
/// committed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitId {
    #[prost(int64, tag = "1")]
    pub version: i64,
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for CommitId {
    const NAME: &'static str = "CommitID";
    const PACKAGE: &'static str = "cosmos.base.store.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.base.store.v1beta1.{}", Self::NAME)
    }
}
/// StoreKVPair is a KVStore KVPair used for listening to state changes (Sets and Deletes)
/// It optionally includes the StoreKey for the originating KVStore and a Boolean flag to distinguish between Sets and
/// Deletes
///
/// Since: cosmos-sdk 0.43
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreKvPair {
    /// the store key for the KVStore this pair originates from
    #[prost(string, tag = "1")]
    pub store_key: ::prost::alloc::string::String,
    /// true indicates a delete operation, false indicates a set operation
    #[prost(bool, tag = "2")]
    pub delete: bool,
    #[prost(bytes = "vec", tag = "3")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for StoreKvPair {
    const NAME: &'static str = "StoreKVPair";
    const PACKAGE: &'static str = "cosmos.base.store.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.base.store.v1beta1.{}", Self::NAME)
    }
}
/// BlockMetadata contains all the abci event data of a block
/// the file streamer dump them into files together with the state changes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMetadata {
    #[prost(message, optional, tag = "1")]
    pub request_begin_block:
        ::core::option::Option<::tendermint_proto::v0_37::abci::RequestBeginBlock>,
    #[prost(message, optional, tag = "2")]
    pub response_begin_block:
        ::core::option::Option<::tendermint_proto::v0_37::abci::ResponseBeginBlock>,
    #[prost(message, repeated, tag = "3")]
    pub deliver_txs: ::prost::alloc::vec::Vec<block_metadata::DeliverTx>,
    #[prost(message, optional, tag = "4")]
    pub request_end_block: ::core::option::Option<::tendermint_proto::v0_37::abci::RequestEndBlock>,
    #[prost(message, optional, tag = "5")]
    pub response_end_block:
        ::core::option::Option<::tendermint_proto::v0_37::abci::ResponseEndBlock>,
    #[prost(message, optional, tag = "6")]
    pub response_commit: ::core::option::Option<::tendermint_proto::v0_37::abci::ResponseCommit>,
}
/// Nested message and enum types in `BlockMetadata`.
pub mod block_metadata {
    /// DeliverTx encapulate deliver tx request and response.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeliverTx {
        #[prost(message, optional, tag = "1")]
        pub request: ::core::option::Option<::tendermint_proto::v0_37::abci::RequestDeliverTx>,
        #[prost(message, optional, tag = "2")]
        pub response: ::core::option::Option<::tendermint_proto::v0_37::abci::ResponseDeliverTx>,
    }
    impl ::prost::Name for DeliverTx {
        const NAME: &'static str = "DeliverTx";
        const PACKAGE: &'static str = "cosmos.base.store.v1beta1";
        fn full_name() -> ::prost::alloc::string::String {
            ::prost::alloc::format!("cosmos.base.store.v1beta1.BlockMetadata.{}", Self::NAME)
        }
    }
}
impl ::prost::Name for BlockMetadata {
    const NAME: &'static str = "BlockMetadata";
    const PACKAGE: &'static str = "cosmos.base.store.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.base.store.v1beta1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `cosmos.base.store.v1beta1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xf3, 0x0b, 0x0a, 0x2b, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x62, 0x61, 0x73, 0x65,
    0x2f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2f, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2f, 0x63,
    0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x19, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x73, 0x74,
    0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x1a, 0x14, 0x67, 0x6f, 0x67,
    0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x2f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0xb7, 0x01, 0x0a, 0x0a, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x49, 0x6e, 0x66,
    0x6f, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x03, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x4b, 0x0a, 0x0b, 0x73,
    0x74, 0x6f, 0x72, 0x65, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x24, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x73,
    0x74, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x53, 0x74, 0x6f,
    0x72, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x0a, 0x73, 0x74,
    0x6f, 0x72, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x73, 0x12, 0x42, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x42, 0x08, 0xc8, 0xde, 0x1f, 0x00, 0x90, 0xdf, 0x1f,
    0x01, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x22, 0x67, 0x0a, 0x09,
    0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x46, 0x0a,
    0x09, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x23, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x73,
    0x74, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x43, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x49, 0x44, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x08, 0x63, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x49, 0x64, 0x22, 0x3e, 0x0a, 0x08, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x49,
    0x44, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x03, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x68,
    0x61, 0x73, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x68, 0x61, 0x73, 0x68, 0x3a,
    0x04, 0x98, 0xa0, 0x1f, 0x00, 0x42, 0x2a, 0x5a, 0x28, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e,
    0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
    0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2f, 0x74, 0x79, 0x70, 0x65,
    0x73, 0x4a, 0xda, 0x07, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1f, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x00, 0x22,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x00, 0x1e, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x3f,
    0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x06, 0x00, 0x3f, 0x0a, 0x6e, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x0a, 0x00, 0x0f, 0x01, 0x1a, 0x62, 0x20, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74,
    0x49, 0x6e, 0x66, 0x6f, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x63, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x75, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x75, 0x6c, 0x74,
    0x69, 0x2d, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x63, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x0a, 0x20, 0x61, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x2f, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x0b, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b,
    0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x1c, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x2a, 0x2b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x4b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x0c, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0c, 0x1c, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0c, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x2c,
    0x4a, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe9, 0xfb, 0x03, 0x12, 0x03, 0x0c,
    0x2d, 0x49, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x0d, 0x02, 0x0e, 0x41,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0d, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x1c, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x08, 0x12, 0x03, 0x0e, 0x06, 0x40, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x02,
    0x08, 0xe9, 0xfb, 0x03, 0x12, 0x03, 0x0e, 0x07, 0x23, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02,
    0x02, 0x08, 0xf2, 0xfb, 0x03, 0x12, 0x03, 0x0e, 0x25, 0x3f, 0x0a, 0x83, 0x01, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x13, 0x00, 0x16, 0x01, 0x1a, 0x77, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49,
    0x6e, 0x66, 0x6f, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72,
    0x65, 0x2d, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69,
    0x74, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x49,
    0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x61, 0x20, 0x72, 0x65, 0x66,
    0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x0a, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20,
    0x61, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x20, 0x49, 0x44, 0x2e, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x13, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x14, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x14, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x14, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x14, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x15, 0x02, 0x38,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x08, 0x12, 0x03, 0x15, 0x19, 0x37, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x01,
    0x08, 0xe9, 0xfb, 0x03, 0x12, 0x03, 0x15, 0x1a, 0x36, 0x0a, 0x5e, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x1a, 0x00, 0x1f, 0x01, 0x1a, 0x52, 0x20, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x49, 0x44,
    0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x61, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69,
    0x66, 0x69, 0x63, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x63, 0x6f,
    0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x1a, 0x08, 0x10, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x07, 0x12, 0x03, 0x1b, 0x02,
    0x2e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x07, 0x83, 0xf4, 0x03, 0x12, 0x03, 0x1b, 0x02, 0x2e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x02, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1d, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x1d, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x1e, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1e,
    0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1e, 0x12, 0x13, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33, 0x0a, 0xfa, 0x11, 0x0a, 0x29, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
    0x73, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2f, 0x76, 0x31, 0x62,
    0x65, 0x74, 0x61, 0x31, 0x2f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x19, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73,
    0x65, 0x2e, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x1a,
    0x1b, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x74, 0x2f, 0x61, 0x62, 0x63, 0x69,
    0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x6a, 0x0a, 0x0b,
    0x53, 0x74, 0x6f, 0x72, 0x65, 0x4b, 0x56, 0x50, 0x61, 0x69, 0x72, 0x12, 0x1b, 0x0a, 0x09, 0x73,
    0x74, 0x6f, 0x72, 0x65, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08,
    0x73, 0x74, 0x6f, 0x72, 0x65, 0x4b, 0x65, 0x79, 0x12, 0x16, 0x0a, 0x06, 0x64, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65,
    0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b,
    0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x83, 0x05, 0x0a, 0x0d, 0x42, 0x6c, 0x6f,
    0x63, 0x6b, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12, 0x52, 0x0a, 0x13, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x62, 0x65, 0x67, 0x69, 0x6e, 0x5f, 0x62, 0x6c, 0x6f, 0x63,
    0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x72,
    0x6d, 0x69, 0x6e, 0x74, 0x2e, 0x61, 0x62, 0x63, 0x69, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x42, 0x65, 0x67, 0x69, 0x6e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x52, 0x11, 0x72, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x42, 0x65, 0x67, 0x69, 0x6e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x55,
    0x0a, 0x14, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5f, 0x62, 0x65, 0x67, 0x69, 0x6e,
    0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x74,
    0x65, 0x6e, 0x64, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x74, 0x2e, 0x61, 0x62, 0x63, 0x69, 0x2e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x65, 0x67, 0x69, 0x6e, 0x42, 0x6c, 0x6f, 0x63,
    0x6b, 0x52, 0x12, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x65, 0x67, 0x69, 0x6e,
    0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x53, 0x0a, 0x0b, 0x64, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72,
    0x5f, 0x74, 0x78, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x63, 0x6f, 0x73,
    0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x76,
    0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4d, 0x65, 0x74, 0x61,
    0x64, 0x61, 0x74, 0x61, 0x2e, 0x44, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x54, 0x78, 0x52, 0x0a,
    0x64, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x54, 0x78, 0x73, 0x12, 0x4c, 0x0a, 0x11, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x65, 0x6e, 0x64, 0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x6d, 0x69,
    0x6e, 0x74, 0x2e, 0x61, 0x62, 0x63, 0x69, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x45,
    0x6e, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x52, 0x0f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x45, 0x6e, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x4f, 0x0a, 0x12, 0x72, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x5f, 0x65, 0x6e, 0x64, 0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x6d, 0x69, 0x6e,
    0x74, 0x2e, 0x61, 0x62, 0x63, 0x69, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x45,
    0x6e, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x52, 0x10, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x45, 0x6e, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x48, 0x0a, 0x0f, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x74, 0x2e,
    0x61, 0x62, 0x63, 0x69, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x43, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x52, 0x0e, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x43, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x1a, 0x88, 0x01, 0x0a, 0x09, 0x44, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x54,
    0x78, 0x12, 0x3b, 0x0a, 0x07, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x21, 0x2e, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x74, 0x2e,
    0x61, 0x62, 0x63, 0x69, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x44, 0x65, 0x6c, 0x69,
    0x76, 0x65, 0x72, 0x54, 0x78, 0x52, 0x07, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x3e,
    0x0a, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x22, 0x2e, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x74, 0x2e, 0x61, 0x62,
    0x63, 0x69, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x44, 0x65, 0x6c, 0x69, 0x76,
    0x65, 0x72, 0x54, 0x78, 0x52, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x2a,
    0x5a, 0x28, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73,
    0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x73,
    0x74, 0x6f, 0x72, 0x65, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x4a, 0xee, 0x0a, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x21, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x03, 0x00, 0x25, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05, 0x00, 0x3f, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x05, 0x00, 0x3f, 0x0a, 0xfb, 0x01, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x0c, 0x00, 0x11, 0x01, 0x1a, 0xee, 0x01, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x4b,
    0x56, 0x50, 0x61, 0x69, 0x72, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x4b, 0x56, 0x53, 0x74, 0x6f,
    0x72, 0x65, 0x20, 0x4b, 0x56, 0x50, 0x61, 0x69, 0x72, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20,
    0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x28, 0x53,
    0x65, 0x74, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x73, 0x29,
    0x0a, 0x20, 0x49, 0x74, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x6c, 0x79, 0x20,
    0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x74, 0x6f,
    0x72, 0x65, 0x4b, 0x65, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72,
    0x69, 0x67, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x4b, 0x56, 0x53, 0x74, 0x6f, 0x72,
    0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x42, 0x6f, 0x6f, 0x6c, 0x65, 0x61, 0x6e, 0x20,
    0x66, 0x6c, 0x61, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x75,
    0x69, 0x73, 0x68, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x53, 0x65, 0x74, 0x73,
    0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x73, 0x0a, 0x0a, 0x20,
    0x53, 0x69, 0x6e, 0x63, 0x65, 0x3a, 0x20, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64,
    0x6b, 0x20, 0x30, 0x2e, 0x34, 0x33, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x0c, 0x08, 0x13, 0x0a, 0x46, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x02, 0x17,
    0x22, 0x39, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x6b, 0x65, 0x79,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4b, 0x56, 0x53, 0x74, 0x6f, 0x72, 0x65,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x70, 0x61, 0x69, 0x72, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69,
    0x6e, 0x61, 0x74, 0x65, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x0d, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x0d, 0x15, 0x16, 0x0a, 0x51, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0e,
    0x02, 0x17, 0x22, 0x44, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61,
    0x74, 0x65, 0x73, 0x20, 0x61, 0x20, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x20, 0x6f, 0x70, 0x65,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2c, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x20, 0x69, 0x6e,
    0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x61, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x70,
    0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x0e, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0e, 0x07, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e,
    0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x02, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x02, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x10, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x10, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x10,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x10, 0x15, 0x16,
    0x0a, 0x91, 0x01, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x15, 0x00, 0x21, 0x01, 0x1a, 0x84, 0x01,
    0x20, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x63,
    0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x61, 0x62, 0x63, 0x69, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20,
    0x6f, 0x66, 0x20, 0x61, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x66, 0x69, 0x6c, 0x65, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x65, 0x72, 0x20, 0x64, 0x75,
    0x6d, 0x70, 0x20, 0x74, 0x68, 0x65, 0x6d, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x66, 0x69, 0x6c,
    0x65, 0x73, 0x20, 0x74, 0x6f, 0x67, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x15, 0x08, 0x15,
    0x0a, 0x45, 0x0a, 0x04, 0x04, 0x01, 0x03, 0x00, 0x12, 0x04, 0x17, 0x04, 0x1a, 0x05, 0x1a, 0x37,
    0x20, 0x44, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x54, 0x78, 0x20, 0x65, 0x6e, 0x63, 0x61, 0x70,
    0x75, 0x6c, 0x61, 0x74, 0x65, 0x20, 0x64, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x20, 0x74, 0x78,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01,
    0x12, 0x03, 0x17, 0x0c, 0x15, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x18, 0x08, 0x35, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x18, 0x08, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x18, 0x29, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x18, 0x33, 0x34, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x19, 0x08, 0x37, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x19, 0x08, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x19, 0x2a, 0x32, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x19, 0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x04, 0x3e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1b, 0x04, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x26, 0x39, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x3c, 0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x1c, 0x04, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x1c, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x1c, 0x27, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1c, 0x3e,
    0x3f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x1d, 0x04, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1d, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x17, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x1d, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03,
    0x1e, 0x04, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x06, 0x12, 0x03, 0x1e, 0x04,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1e, 0x24, 0x35, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1e, 0x38, 0x39, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x1f, 0x04, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x04, 0x06, 0x12, 0x03, 0x1f, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x1f, 0x25, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x1f, 0x3a, 0x3b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x20, 0x04,
    0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x06, 0x12, 0x03, 0x20, 0x04, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x20, 0x23, 0x32, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x20, 0x35, 0x36, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
