//! Application BlockChain Interface ([ABCI]) is the interface between Tendermint
//! (a consensus engine for Byzantine-fault-tolerant replication of a state
//! machine) and an application (the state machine to be replicated).
//!
//! Using ABCI involves writing an application driven by ABCI methods, exposing
//! that application as an ABCI server, and having Tendermint connect to the
//! server as an ABCI client.
//!
//! This module does not include an ABCI server implementation itself. Instead,
//! it provides a common set of Rust domain types that model the ABCI protocol,
//! which can be used by both ABCI applications and ABCI server implementations.
//!
//! One ABCI server implementation is provided by the [`tendermint_abci`][tmabci]
//! crate.
//!
//! Each ABCI method corresponds to a request/response pair. ABCI requests are
//! modeled by the [`Request`] enum, and responses are modeled by the
//! [`Response`] enum.  As described in the [methods and types][mat] page, ABCI
//! methods are split into four categories. Tendermint opens one ABCI connection
//! for each category of messages. These categories are modeled by the
//! [`MethodKind`] enum and by per-category request and response enums:
//!
//! * [`ConsensusRequest`] /  [`ConsensusResponse`] for [`MethodKind::Consensus`] methods;
//! * [`MempoolRequest`] /  [`MempoolResponse`] for [`MethodKind::Mempool`] methods;
//! * [`InfoRequest`] /  [`InfoResponse`] for [`MethodKind::Info`] methods;
//! * [`SnapshotRequest`] /  [`SnapshotResponse`] for [`MethodKind::Snapshot`] methods.
//!
//! The domain types in this module have conversions to and from the Protobuf
//! types defined in the [`tendermint_proto`] crate. These conversions are
//! required for ABCI server implementations, which use the protobufs to
//! communicate with Tendermint, but should not be required for ABCI
//! applications, which should use the domain types in an interface defined by
//! their choice of ABCI server implementation.
//!
//! [ABCI]: https://docs.tendermint.com/master/spec/abci/
//! [mat]: https://docs.tendermint.com/master/spec/abci/abci.html
//! [tmabci]: https://github.com/informalsystems/tendermint-rs/tree/master/abci

mod code;
mod data;
mod gas;
mod info;
mod kind;
mod log;
mod path;

/// Events. Hide this later once types are merged.
pub mod event;
//pub use event::{Event, EventAttribute};

pub mod params;
pub mod request;
pub mod response;
pub mod types;

#[doc(hidden)]
pub mod responses;
#[doc(hidden)]
pub mod tag;
#[doc(hidden)]
pub mod transaction;

#[doc(hidden)]
pub use self::{
    code::Code,
    data::Data,
    gas::Gas,
    info::Info,
    log::Log,
    path::Path,
    responses::{DeliverTx, Event, Responses},
    transaction::Transaction,
};

#[doc(inline)]
pub use self::{
    kind::MethodKind,
    request::{ConsensusRequest, InfoRequest, MempoolRequest, Request, SnapshotRequest},
    response::{ConsensusResponse, InfoResponse, MempoolResponse, Response, SnapshotResponse},
};
