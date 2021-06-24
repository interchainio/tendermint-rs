//! ABCI requests and request data.
//!
//! The [`Request`] enum records all possible ABCI requests. Requests that
//! contain data are modeled as a separate struct, to avoid duplication of field
//! definitions.

// IMPORTANT NOTE ON DOCUMENTATION:
//
// The documentation for each request type is adapted from the ABCI Methods and
// Types spec document. However, the same logical request may appear three
// times, as a struct with the request data, as a Request variant, and as a
// CategoryRequest variant. Ideally, the documentation would be copied between
// these automatically, but doing this requires using #[doc = include_str!],
// which is unstable. For now, the Request enum is the source of truth; please
// change the docs there and copy as required.

use std::convert::{TryFrom, TryInto};

use bytes::Bytes;
use chrono::{DateTime, Utc};

use crate::block;

use super::{
    params::ConsensusParams,
    types::{Evidence, LastCommitInfo, Snapshot, ValidatorUpdate},
    MethodKind,
};

/// Echoes a string to test an ABCI implementation.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#echo)
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Echo {
    /// The message to send back.
    pub message: String,
}

/// Requests information about the application state.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#info)
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Info {
    /// The Tendermint software semantic version.
    pub version: String,
    /// The Tendermint block protocol version.
    pub block_version: u64,
    /// The Tendermint p2p protocol version.
    pub p2p_version: u64,
    /// The Tendermint ABCI semantic version.
    pub abci_version: String,
}

/// Called on genesis to initialize chain state.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#initchain)
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InitChain {
    /// The genesis time.
    pub time: DateTime<Utc>,
    /// The ID of the blockchain.
    pub chain_id: String,
    /// Initial consensus-critical parameters.
    pub consensus_params: ConsensusParams,
    /// Initial genesis validators, sorted by voting power.
    pub validators: Vec<ValidatorUpdate>,
    /// Serialized JSON bytes containing the initial application state.
    pub app_state_bytes: Bytes,
    /// Height of the initial block (typically `1`).
    pub initial_height: i64,
}

/// Queries for data from the application at current or past height.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#query)
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Query {
    /// Raw query bytes.
    ///
    /// Can be used with or in lieu of `path`.
    pub data: Bytes,
    /// Path of the request, like an HTTP `GET` path.
    ///
    /// Can be used with or in lieu of `data`.
    ///
    /// Applications MUST interpret `/store` as a query by key on the underlying
    /// store. The key SHOULD be specified in the Data field. Applications SHOULD
    /// allow queries over specific types like `/accounts/...` or `/votes/...`.
    pub path: String,
    /// The block height for which the query should be executed.
    ///
    /// The default `0` returns data for the latest committed block. Note that
    /// this is the height of the block containing the application's Merkle root
    /// hash, which represents the state as it was after committing the block at
    /// `height - 1`.
    pub height: i64,
    /// Whether to return a Merkle proof with the response, if possible.
    pub prove: bool,
}

/// Signals the beginning of a new block.
///
/// Called prior to any [`DeliverTx`]s. The `header` contains the height,
/// timestamp, and more -- it exactly matches the Tendermint block header.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#beginblock)
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BeginBlock {
    /// The block's hash.
    ///
    /// This can be derived from the block header.
    pub hash: Bytes,
    /// The block header.
    pub header: block::Header,
    /// Information about the last commit.
    ///
    /// This includes the round, the list of validators, and which validators
    /// signed the last block.
    pub last_commit_info: LastCommitInfo,
    /// Evidence of validator misbehavior.
    pub byzantine_validators: Vec<Evidence>,
}

/// Execute a transaction against the application state.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#delivertx)
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DeliverTx {
    /// The bytes of the transaction to execute.
    pub tx: Bytes,
}

/// Signals the end of a block.
///
/// Called after all transactions, and prior to each `Commit`.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#endblock)
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EndBlock {
    /// The height of the block just executed.
    pub height: i64,
}

/// Check whether a transaction should be included in the mempool.
///
/// `CheckTx` is not involved in processing blocks, only in deciding whether a
/// transaction should be included in the mempool. Every node runs `CheckTx`
/// before adding a transaction to its local mempool. The transaction may come
/// from an external user or another node. `CheckTx` need not execute the
/// transaction in full, but can instead perform lightweight or statateful
/// validation (e.g., checking signatures or account balances) instead of more
/// expensive checks (like running code in a virtual machine).
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#checktx)
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CheckTx {
    /// The transaction bytes.
    pub tx: Bytes,
    /// The kind of check to perform.
    ///
    /// Note: this field is called `type` in the protobuf, but we call it `kind`
    /// to avoid the Rust keyword.
    pub kind: CheckTxKind,
}

/// The possible kinds of [`CheckTx`] checks.
///
/// Note: the
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#checktx)
/// calls this `CheckTxType`, but we follow the Rust convention and name it `CheckTxKind`
/// to avoid confusion with Rust types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CheckTxKind {
    /// A full check is required (the default).
    New = 0,
    /// Indicates that the mempool is initiating a recheck of the transaction.
    Recheck = 1,
}

impl Default for CheckTxKind {
    fn default() -> Self {
        CheckTxKind::New
    }
}

/// Offers a list of snapshots to the application.
///
/// `OfferSnapshot` is called when bootstrapping a node using state sync. The
/// application may accept or reject snapshots as appropriate. Upon accepting,
/// Tendermint will retrieve and apply snapshot chunks via
/// [`ApplySnapshotChunk`]. The application may also choose to reject a snapshot
/// in the chunk response, in which case it should be prepared to accept further
/// `OfferSnapshot` calls.
///
/// Only `app_hash` can be trusted, as it has been verified by the light client.
/// Any other data can be spoofed by adversaries, so applications should employ
/// additional verification schemes to avoid denial-of-service attacks. The
/// verified `app_hash` is automatically checked against the restored application
/// at the end of snapshot restoration.
///
/// See also the [`Snapshot`] data type and the [ABCI state sync documentation][ssd].
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#offersnapshot)
///
/// [ssd]: https://docs.tendermint.com/master/spec/abci/apps.html#state-sync
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OfferSnapshot {
    /// The snapshot offered for restoration.
    pub snapshot: Snapshot,
    /// The light client verified app hash for this height.
    pub app_hash: Bytes,
}

/// Requests a snapshot chunk from the application to send to a peer.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#loadsnapshotchunk)
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LoadSnapshotChunk {
    /// The height of the snapshot the chunks belong to.
    pub height: u64,
    /// An application-specific identifier of the format of the snapshot chunk.
    pub format: u32,
    /// The chunk index, starting from `0` for the initial chunk.
    pub chunk: u32,
}

/// Applies a snapshot chunk.
///
/// The application can choose to refetch chunks and/or ban P2P peers as
/// appropriate. Tendermint will not do this unless instructed by the
/// application.
///
/// The application may want to verify each chunk, e.g., by attaching chunk
/// hashes in [`Snapshot::metadata`] and/or incrementally verifying contents
/// against `app_hash`.
///
/// When all chunks have been accepted, Tendermint will make an ABCI [`Info`]
/// request to verify that `last_block_app_hash` and `last_block_height` match
/// the expected values, and record the `app_version` in the node state. It then
/// switches to fast sync or consensus and joins the network.
///
/// If Tendermint is unable to retrieve the next chunk after some time (e.g.,
/// because no suitable peers are available), it will reject the snapshot and try
/// a different one via `OfferSnapshot`. The application should be prepared to
/// reset and accept it or abort as appropriate.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#applysnapshotchunk)
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ApplySnapshotChunk {
    /// The chunk index, starting from `0`.  Tendermint applies chunks sequentially.
    pub index: u32,
    /// The binary chunk contents, as returned by [`LoadSnapshotChunk`].
    pub chunk: Bytes,
    /// The P2P ID of the node who sent this chunk.
    pub sender: String,
}

/// All possible ABCI requests.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Request {
    /// Echoes a string to test an ABCI implementation.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#echo)
    Echo(Echo),
    /// Indicates that any pending requests should be completed and their responses flushed.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#flush)
    Flush,
    /// Requests information about the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#info)
    Info(Info),
    /// Called on genesis to initialize chain state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#initchain)
    InitChain(InitChain),
    /// Queries for data from the application at current or past height.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#query)
    Query(Query),
    /// Signals the beginning of a new block.
    ///
    /// Called prior to any [`DeliverTx`]s. The `header` contains the height,
    /// timestamp, and more -- it exactly matches the Tendermint block header.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#beginblock)
    BeginBlock(BeginBlock),
    /// Check whether a transaction should be included in the mempool.
    ///
    /// `CheckTx` is not involved in processing blocks, only in deciding whether a
    /// transaction should be included in the mempool. Every node runs `CheckTx`
    /// before adding a transaction to its local mempool. The transaction may come
    /// from an external user or another node. `CheckTx` need not execute the
    /// transaction in full, but can instead perform lightweight or statateful
    /// validation (e.g., checking signatures or account balances) instead of more
    /// expensive checks (like running code in a virtual machine).
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#checktx)
    CheckTx(CheckTx),
    /// Execute a transaction against the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#delivertx)
    DeliverTx(DeliverTx),
    /// Signals the end of a block.
    ///
    /// Called after all transactions, and prior to each `Commit`.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#endblock)
    EndBlock(EndBlock),
    /// Signals the application that it can write the queued state transitions
    /// from the block to its state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#commit)
    Commit,
    /// Asks the application for a list of snapshots.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#listsnapshots)
    ListSnapshots,
    /// Offers a list of snapshots to the application.
    ///
    /// `OfferSnapshot` is called when bootstrapping a node using state sync. The
    /// application may accept or reject snapshots as appropriate. Upon accepting,
    /// Tendermint will retrieve and apply snapshot chunks via
    /// [`ApplySnapshotChunk`]. The application may also choose to reject a snapshot
    /// in the chunk response, in which case it should be prepared to accept further
    /// `OfferSnapshot` calls.
    ///
    /// Only `app_hash` can be trusted, as it has been verified by the light client.
    /// Any other data can be spoofed by adversaries, so applications should employ
    /// additional verification schemes to avoid denial-of-service attacks. The
    /// verified `app_hash` is automatically checked against the restored application
    /// at the end of snapshot restoration.
    ///
    /// See also the [`Snapshot`] data type and the [ABCI state sync documentation][ssd].
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#offersnapshot)
    ///
    /// [ssd]: https://docs.tendermint.com/master/spec/abci/apps.html#state-sync
    OfferSnapshot(OfferSnapshot),
    /// Used during state sync to retrieve snapshot chunks from peers.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#loadsnapshotchunk)
    LoadSnapshotChunk(LoadSnapshotChunk),
    /// Applies a snapshot chunk.
    ///
    /// The application can choose to refetch chunks and/or ban P2P peers as
    /// appropriate. Tendermint will not do this unless instructed by the
    /// application.
    ///
    /// The application may want to verify each chunk, e.g., by attaching chunk
    /// hashes in [`Snapshot::metadata`] and/or incrementally verifying contents
    /// against `app_hash`.
    ///
    /// When all chunks have been accepted, Tendermint will make an ABCI [`Info`]
    /// request to verify that `last_block_app_hash` and `last_block_height` match
    /// the expected values, and record the `app_version` in the node state. It then
    /// switches to fast sync or consensus and joins the network.
    ///
    /// If Tendermint is unable to retrieve the next chunk after some time (e.g.,
    /// because no suitable peers are available), it will reject the snapshot and try
    /// a different one via `OfferSnapshot`. The application should be prepared to
    /// reset and accept it or abort as appropriate.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#applysnapshotchunk)
    ApplySnapshotChunk(ApplySnapshotChunk),
}

impl Request {
    /// Get the method kind for this request.
    pub fn kind(&self) -> MethodKind {
        use Request::*;
        match self {
            Flush => MethodKind::Flush,
            InitChain(_) => MethodKind::Consensus,
            BeginBlock(_) => MethodKind::Consensus,
            DeliverTx(_) => MethodKind::Consensus,
            EndBlock(_) => MethodKind::Consensus,
            Commit => MethodKind::Consensus,
            CheckTx(_) => MethodKind::Mempool,
            ListSnapshots => MethodKind::Snapshot,
            OfferSnapshot(_) => MethodKind::Snapshot,
            LoadSnapshotChunk(_) => MethodKind::Snapshot,
            ApplySnapshotChunk(_) => MethodKind::Snapshot,
            Info(_) => MethodKind::Info,
            Query(_) => MethodKind::Info,
            Echo(_) => MethodKind::Info,
        }
    }
}

/// The consensus category of ABCI requests.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ConsensusRequest {
    /// Called on genesis to initialize chain state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#initchain)
    InitChain(InitChain),
    /// Signals the beginning of a new block.
    ///
    /// Called prior to any [`DeliverTx`]s. The `header` contains the height,
    /// timestamp, and more -- it exactly matches the Tendermint block header.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#beginblock)
    BeginBlock(BeginBlock),
    /// Execute a transaction against the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#delivertx)
    DeliverTx(DeliverTx),
    /// Signals the end of a block.
    ///
    /// Called after all transactions, and prior to each `Commit`.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#endblock)
    EndBlock(EndBlock),
    /// Signals the application that it can write the queued state transitions
    /// from the block to its state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#commit)
    Commit,
}

impl From<ConsensusRequest> for Request {
    fn from(req: ConsensusRequest) -> Self {
        match req {
            ConsensusRequest::InitChain(x) => Self::InitChain(x),
            ConsensusRequest::BeginBlock(x) => Self::BeginBlock(x),
            ConsensusRequest::DeliverTx(x) => Self::DeliverTx(x),
            ConsensusRequest::EndBlock(x) => Self::EndBlock(x),
            ConsensusRequest::Commit => Self::Commit,
        }
    }
}

impl TryFrom<Request> for ConsensusRequest {
    type Error = &'static str;
    fn try_from(req: Request) -> Result<Self, Self::Error> {
        match req {
            Request::InitChain(x) => Ok(Self::InitChain(x)),
            Request::BeginBlock(x) => Ok(Self::BeginBlock(x)),
            Request::DeliverTx(x) => Ok(Self::DeliverTx(x)),
            Request::EndBlock(x) => Ok(Self::EndBlock(x)),
            Request::Commit => Ok(Self::Commit),
            _ => Err("wrong request type"),
        }
    }
}

/// The mempool category of ABCI requests.
#[derive(Clone, PartialEq, Debug)]
pub enum MempoolRequest {
    /// Check whether a transaction should be included in the mempool.
    ///
    /// `CheckTx` is not involved in processing blocks, only in deciding whether a
    /// transaction should be included in the mempool. Every node runs `CheckTx`
    /// before adding a transaction to its local mempool. The transaction may come
    /// from an external user or another node. `CheckTx` need not execute the
    /// transaction in full, but can instead perform lightweight or statateful
    /// validation (e.g., checking signatures or account balances) instead of more
    /// expensive checks (like running code in a virtual machine).
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#checktx)
    CheckTx(CheckTx),
}

impl From<MempoolRequest> for Request {
    fn from(req: MempoolRequest) -> Self {
        match req {
            MempoolRequest::CheckTx(x) => Self::CheckTx(x),
        }
    }
}

impl TryFrom<Request> for MempoolRequest {
    type Error = &'static str;
    fn try_from(req: Request) -> Result<Self, Self::Error> {
        match req {
            Request::CheckTx(x) => Ok(Self::CheckTx(x)),
            _ => Err("wrong request type"),
        }
    }
}

/// The info category of ABCI requests.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum InfoRequest {
    /// Requests information about the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#info)
    Info(Info),
    /// Queries for data from the application at current or past height.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#query)
    Query(Query),
    /// Echoes a string to test an ABCI implementation.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#echo)
    Echo(Echo),
}

impl From<InfoRequest> for Request {
    fn from(req: InfoRequest) -> Self {
        match req {
            InfoRequest::Info(x) => Self::Info(x),
            InfoRequest::Query(x) => Self::Query(x),
            InfoRequest::Echo(x) => Self::Echo(x),
        }
    }
}

impl TryFrom<Request> for InfoRequest {
    type Error = &'static str;
    fn try_from(req: Request) -> Result<Self, Self::Error> {
        match req {
            Request::Info(x) => Ok(Self::Info(x)),
            Request::Query(x) => Ok(Self::Query(x)),
            Request::Echo(x) => Ok(Self::Echo(x)),
            _ => Err("wrong request type"),
        }
    }
}

/// The snapshot category of ABCI requests.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SnapshotRequest {
    /// Asks the application for a list of snapshots.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#listsnapshots)
    ListSnapshots,
    /// Offers a list of snapshots to the application.
    ///
    /// `OfferSnapshot` is called when bootstrapping a node using state sync. The
    /// application may accept or reject snapshots as appropriate. Upon accepting,
    /// Tendermint will retrieve and apply snapshot chunks via
    /// [`ApplySnapshotChunk`]. The application may also choose to reject a snapshot
    /// in the chunk response, in which case it should be prepared to accept further
    /// `OfferSnapshot` calls.
    ///
    /// Only `app_hash` can be trusted, as it has been verified by the light client.
    /// Any other data can be spoofed by adversaries, so applications should employ
    /// additional verification schemes to avoid denial-of-service attacks. The
    /// verified `app_hash` is automatically checked against the restored application
    /// at the end of snapshot restoration.
    ///
    /// See also the [`Snapshot`] data type and the [ABCI state sync documentation][ssd].
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#offersnapshot)
    ///
    /// [ssd]: https://docs.tendermint.com/master/spec/abci/apps.html#state-sync
    OfferSnapshot(OfferSnapshot),
    /// Used during state sync to retrieve snapshot chunks from peers.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#loadsnapshotchunk)
    LoadSnapshotChunk(LoadSnapshotChunk),
    /// Applies a snapshot chunk.
    ///
    /// The application can choose to refetch chunks and/or ban P2P peers as
    /// appropriate. Tendermint will not do this unless instructed by the
    /// application.
    ///
    /// The application may want to verify each chunk, e.g., by attaching chunk
    /// hashes in [`Snapshot::metadata`] and/or incrementally verifying contents
    /// against `app_hash`.
    ///
    /// When all chunks have been accepted, Tendermint will make an ABCI [`Info`]
    /// request to verify that `last_block_app_hash` and `last_block_height` match
    /// the expected values, and record the `app_version` in the node state. It then
    /// switches to fast sync or consensus and joins the network.
    ///
    /// If Tendermint is unable to retrieve the next chunk after some time (e.g.,
    /// because no suitable peers are available), it will reject the snapshot and try
    /// a different one via `OfferSnapshot`. The application should be prepared to
    /// reset and accept it or abort as appropriate.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#applysnapshotchunk)
    ApplySnapshotChunk(ApplySnapshotChunk),
}

impl From<SnapshotRequest> for Request {
    fn from(req: SnapshotRequest) -> Self {
        match req {
            SnapshotRequest::ListSnapshots => Self::ListSnapshots,
            SnapshotRequest::OfferSnapshot(x) => Self::OfferSnapshot(x),
            SnapshotRequest::LoadSnapshotChunk(x) => Self::LoadSnapshotChunk(x),
            SnapshotRequest::ApplySnapshotChunk(x) => Self::ApplySnapshotChunk(x),
        }
    }
}

impl TryFrom<Request> for SnapshotRequest {
    type Error = &'static str;
    fn try_from(req: Request) -> Result<Self, Self::Error> {
        match req {
            Request::ListSnapshots => Ok(Self::ListSnapshots),
            Request::OfferSnapshot(x) => Ok(Self::OfferSnapshot(x)),
            Request::LoadSnapshotChunk(x) => Ok(Self::LoadSnapshotChunk(x)),
            Request::ApplySnapshotChunk(x) => Ok(Self::ApplySnapshotChunk(x)),
            _ => Err("wrong request type"),
        }
    }
}

// =============================================================================
// Protobuf conversions
// =============================================================================

// XXX(hdevalence): these all use &'static str for now, this should be fixed
// to align with the crate's error-handling strategy.

use tendermint_proto::abci as pb;
use tendermint_proto::Protobuf;

impl From<Echo> for pb::RequestEcho {
    fn from(echo: Echo) -> Self {
        Self {
            message: echo.message,
        }
    }
}

impl TryFrom<pb::RequestEcho> for Echo {
    type Error = &'static str;

    fn try_from(echo: pb::RequestEcho) -> Result<Self, Self::Error> {
        Ok(Self {
            message: echo.message,
        })
    }
}

impl Protobuf<pb::RequestEcho> for Echo {}

impl From<Info> for pb::RequestInfo {
    fn from(info: Info) -> Self {
        Self {
            version: info.version,
            block_version: info.block_version,
            p2p_version: info.p2p_version,
            abci_version: info.abci_version,
        }
    }
}

impl TryFrom<pb::RequestInfo> for Info {
    type Error = &'static str;

    fn try_from(info: pb::RequestInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            version: info.version,
            block_version: info.block_version,
            p2p_version: info.p2p_version,
            abci_version: info.abci_version,
        })
    }
}

impl Protobuf<pb::RequestInfo> for Info {}

impl From<InitChain> for pb::RequestInitChain {
    fn from(init_chain: InitChain) -> Self {
        Self {
            time: Some(init_chain.time.into()),
            chain_id: init_chain.chain_id,
            consensus_params: Some(init_chain.consensus_params.into()),
            validators: init_chain.validators.into_iter().map(Into::into).collect(),
            app_state_bytes: init_chain.app_state_bytes,
            initial_height: init_chain.initial_height,
        }
    }
}

impl TryFrom<pb::RequestInitChain> for InitChain {
    type Error = crate::Error;

    fn try_from(init_chain: pb::RequestInitChain) -> Result<Self, Self::Error> {
        Ok(Self {
            time: init_chain.time.ok_or("missing genesis time")?.try_into()?,
            chain_id: init_chain.chain_id,
            consensus_params: init_chain
                .consensus_params
                .ok_or("missing consensus params")?
                .try_into()?,
            validators: init_chain
                .validators
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            app_state_bytes: init_chain.app_state_bytes,
            initial_height: init_chain.initial_height,
        })
    }
}

impl Protobuf<pb::RequestInitChain> for InitChain {}

impl From<Query> for pb::RequestQuery {
    fn from(query: Query) -> Self {
        Self {
            data: query.data,
            path: query.path,
            height: query.height,
            prove: query.prove,
        }
    }
}

impl TryFrom<pb::RequestQuery> for Query {
    type Error = crate::Error;

    fn try_from(query: pb::RequestQuery) -> Result<Self, Self::Error> {
        Ok(Self {
            data: query.data,
            path: query.path,
            height: query.height,
            prove: query.prove,
        })
    }
}

impl Protobuf<pb::RequestQuery> for Query {}

impl From<BeginBlock> for pb::RequestBeginBlock {
    fn from(begin_block: BeginBlock) -> Self {
        Self {
            hash: begin_block.hash,
            header: Some(begin_block.header.into()),
            last_commit_info: Some(begin_block.last_commit_info.into()),
            byzantine_validators: begin_block
                .byzantine_validators
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl TryFrom<pb::RequestBeginBlock> for BeginBlock {
    type Error = crate::Error;

    fn try_from(begin_block: pb::RequestBeginBlock) -> Result<Self, Self::Error> {
        Ok(Self {
            hash: begin_block.hash,
            header: begin_block.header.ok_or("missing header")?.try_into()?,
            last_commit_info: begin_block
                .last_commit_info
                .ok_or("missing last commit info")?
                .try_into()?,
            byzantine_validators: begin_block
                .byzantine_validators
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Protobuf<pb::RequestBeginBlock> for BeginBlock {}

impl From<DeliverTx> for pb::RequestDeliverTx {
    fn from(deliver_tx: DeliverTx) -> Self {
        Self { tx: deliver_tx.tx }
    }
}

impl TryFrom<pb::RequestDeliverTx> for DeliverTx {
    type Error = crate::Error;

    fn try_from(deliver_tx: pb::RequestDeliverTx) -> Result<Self, Self::Error> {
        Ok(Self { tx: deliver_tx.tx })
    }
}

impl Protobuf<pb::RequestDeliverTx> for DeliverTx {}

impl From<EndBlock> for pb::RequestEndBlock {
    fn from(end_block: EndBlock) -> Self {
        Self {
            height: end_block.height,
        }
    }
}

impl TryFrom<pb::RequestEndBlock> for EndBlock {
    type Error = crate::Error;

    fn try_from(end_block: pb::RequestEndBlock) -> Result<Self, Self::Error> {
        Ok(Self {
            height: end_block.height,
        })
    }
}

impl Protobuf<pb::RequestEndBlock> for EndBlock {}

impl From<CheckTx> for pb::RequestCheckTx {
    fn from(check_tx: CheckTx) -> Self {
        Self {
            tx: check_tx.tx,
            r#type: check_tx.kind as i32,
        }
    }
}

impl TryFrom<pb::RequestCheckTx> for CheckTx {
    type Error = crate::Error;

    fn try_from(check_tx: pb::RequestCheckTx) -> Result<Self, Self::Error> {
        let kind = match check_tx.r#type {
            0 => CheckTxKind::New,
            1 => CheckTxKind::Recheck,
            _ => Err("unknown checktx type")?,
        };
        Ok(Self {
            tx: check_tx.tx,
            kind,
        })
    }
}

impl Protobuf<pb::RequestCheckTx> for CheckTx {}

impl From<OfferSnapshot> for pb::RequestOfferSnapshot {
    fn from(offer_snapshot: OfferSnapshot) -> Self {
        Self {
            snapshot: Some(offer_snapshot.snapshot.into()),
            app_hash: offer_snapshot.app_hash,
        }
    }
}

impl TryFrom<pb::RequestOfferSnapshot> for OfferSnapshot {
    type Error = crate::Error;

    fn try_from(offer_snapshot: pb::RequestOfferSnapshot) -> Result<Self, Self::Error> {
        Ok(Self {
            snapshot: offer_snapshot
                .snapshot
                .ok_or("missing snapshot")?
                .try_into()?,
            app_hash: offer_snapshot.app_hash,
        })
    }
}

impl Protobuf<pb::RequestOfferSnapshot> for OfferSnapshot {}

impl From<LoadSnapshotChunk> for pb::RequestLoadSnapshotChunk {
    fn from(load_snapshot_chunk: LoadSnapshotChunk) -> Self {
        Self {
            height: load_snapshot_chunk.height,
            format: load_snapshot_chunk.format,
            chunk: load_snapshot_chunk.chunk,
        }
    }
}

impl TryFrom<pb::RequestLoadSnapshotChunk> for LoadSnapshotChunk {
    type Error = crate::Error;

    fn try_from(load_snapshot_chunk: pb::RequestLoadSnapshotChunk) -> Result<Self, Self::Error> {
        Ok(Self {
            height: load_snapshot_chunk.height,
            format: load_snapshot_chunk.format,
            chunk: load_snapshot_chunk.chunk,
        })
    }
}

impl Protobuf<pb::RequestLoadSnapshotChunk> for LoadSnapshotChunk {}

impl From<ApplySnapshotChunk> for pb::RequestApplySnapshotChunk {
    fn from(apply_snapshot_chunk: ApplySnapshotChunk) -> Self {
        Self {
            index: apply_snapshot_chunk.index,
            chunk: apply_snapshot_chunk.chunk,
            sender: apply_snapshot_chunk.sender,
        }
    }
}

impl TryFrom<pb::RequestApplySnapshotChunk> for ApplySnapshotChunk {
    type Error = crate::Error;

    fn try_from(apply_snapshot_chunk: pb::RequestApplySnapshotChunk) -> Result<Self, Self::Error> {
        Ok(Self {
            index: apply_snapshot_chunk.index,
            chunk: apply_snapshot_chunk.chunk,
            sender: apply_snapshot_chunk.sender,
        })
    }
}

impl Protobuf<pb::RequestApplySnapshotChunk> for ApplySnapshotChunk {}

impl From<Request> for pb::Request {
    fn from(request: Request) -> pb::Request {
        use pb::request::Value;
        let value = match request {
            Request::Echo(x) => Some(Value::Echo(x.into())),
            Request::Flush => Some(Value::Flush(Default::default())),
            Request::Info(x) => Some(Value::Info(x.into())),
            Request::InitChain(x) => Some(Value::InitChain(x.into())),
            Request::Query(x) => Some(Value::Query(x.into())),
            Request::BeginBlock(x) => Some(Value::BeginBlock(x.into())),
            Request::CheckTx(x) => Some(Value::CheckTx(x.into())),
            Request::DeliverTx(x) => Some(Value::DeliverTx(x.into())),
            Request::EndBlock(x) => Some(Value::EndBlock(x.into())),
            Request::Commit => Some(Value::Commit(Default::default())),
            Request::ListSnapshots => Some(Value::ListSnapshots(Default::default())),
            Request::OfferSnapshot(x) => Some(Value::OfferSnapshot(x.into())),
            Request::LoadSnapshotChunk(x) => Some(Value::LoadSnapshotChunk(x.into())),
            Request::ApplySnapshotChunk(x) => Some(Value::ApplySnapshotChunk(x.into())),
        };
        pb::Request { value }
    }
}

impl TryFrom<pb::Request> for Request {
    type Error = crate::Error;

    fn try_from(request: pb::Request) -> Result<Self, Self::Error> {
        use pb::request::Value;
        match request.value {
            Some(Value::Echo(x)) => Ok(Request::Echo(x.try_into()?)),
            Some(Value::Flush(pb::RequestFlush {})) => Ok(Request::Flush),
            Some(Value::Info(x)) => Ok(Request::Info(x.try_into()?)),
            Some(Value::InitChain(x)) => Ok(Request::InitChain(x.try_into()?)),
            Some(Value::Query(x)) => Ok(Request::Query(x.try_into()?)),
            Some(Value::BeginBlock(x)) => Ok(Request::BeginBlock(x.try_into()?)),
            Some(Value::CheckTx(x)) => Ok(Request::CheckTx(x.try_into()?)),
            Some(Value::DeliverTx(x)) => Ok(Request::DeliverTx(x.try_into()?)),
            Some(Value::EndBlock(x)) => Ok(Request::EndBlock(x.try_into()?)),
            Some(Value::Commit(pb::RequestCommit {})) => Ok(Request::Commit),
            Some(Value::ListSnapshots(pb::RequestListSnapshots {})) => Ok(Request::ListSnapshots),
            Some(Value::OfferSnapshot(x)) => Ok(Request::OfferSnapshot(x.try_into()?)),
            Some(Value::LoadSnapshotChunk(x)) => Ok(Request::LoadSnapshotChunk(x.try_into()?)),
            Some(Value::ApplySnapshotChunk(x)) => Ok(Request::ApplySnapshotChunk(x.try_into()?)),
            None => Err("no request in proto".into()),
        }
    }
}

impl Protobuf<pb::Request> for Request {}
