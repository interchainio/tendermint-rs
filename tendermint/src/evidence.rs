//! Evidence of malfeasance by validators (i.e. signing conflicting votes).

use crate::error::{self, KindError as Error};
use crate::{block::signed_header::SignedHeader, serializers, vote::Power, Time, Vote};
use serde::{Deserialize, Serialize};
use std::{
    boxed::Box,
    convert::{TryFrom, TryInto},
    slice,
    vec::Vec,
};
use tendermint_proto::google::protobuf::Duration as RawDuration;
use tendermint_proto::types::evidence::Sum as RawSum;
use tendermint_proto::types::evidence::Sum;
use tendermint_proto::types::DuplicateVoteEvidence as RawDuplicateVoteEvidence;
use tendermint_proto::types::Evidence as RawEvidence;
use tendermint_proto::types::EvidenceList as RawEvidenceList;
use tendermint_proto::types::EvidenceParams as RawEvidenceParams;
use tendermint_proto::Protobuf;

mod time {
    #[cfg(not(feature = "std"))]
    pub use core::time::Duration;

    #[cfg(feature = "std")]
    pub use std::time::Duration;
}

/// Evidence of malfeasance by validators (i.e. signing conflicting votes).
/// encoded using an Amino prefix. There is currently only a single type of
/// evidence: `DuplicateVoteEvidence`.
///
/// <https://github.com/tendermint/spec/blob/d46cd7f573a2c6a2399fcab2cde981330aa63f37/spec/core/data_structures.md#evidence>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
//#[serde(tag = "type", content = "value")]
#[serde(try_from = "RawEvidence", into = "RawEvidence")] // Used by RPC /broadcast_evidence endpoint
pub enum Evidence {
    /// Duplicate vote evidence
    //#[serde(rename = "tendermint/DuplicateVoteEvidence")]
    DuplicateVote(DuplicateVoteEvidence),

    /// Conflicting headers evidence - Todo: this is not implemented in protobuf, it's ignored now
    //#[serde(rename = "tendermint/ConflictingHeadersEvidence")]
    ConflictingHeaders(Box<ConflictingHeadersEvidence>),

    /// LightClient attack evidence - Todo: Implement details
    LightClientAttackEvidence,
}

impl TryFrom<RawEvidence> for Evidence {
    type Error = Error;

    fn try_from(value: RawEvidence) -> Result<Self, Self::Error> {
        match value
            .sum
            .ok_or(error::invalid_evidence_error(anyhow::anyhow!(
                "invalid evidence error"
            )))? {
            Sum::DuplicateVoteEvidence(ev) => Ok(Evidence::DuplicateVote(ev.try_into()?)),
            Sum::LightClientAttackEvidence(_ev) => Ok(Evidence::LightClientAttackEvidence),
        }
    }
}

impl From<Evidence> for RawEvidence {
    fn from(value: Evidence) -> Self {
        match value {
            Evidence::DuplicateVote(ev) => RawEvidence {
                sum: Some(RawSum::DuplicateVoteEvidence(ev.into())),
            },
            Evidence::ConflictingHeaders(_ev) => RawEvidence { sum: None }, // Todo: implement
            Evidence::LightClientAttackEvidence => RawEvidence { sum: None }, // Todo: implement
        }
    }
}

/// Duplicate vote evidence
#[derive(Clone, Debug, PartialEq)]
pub struct DuplicateVoteEvidence {
    vote_a: Vote,
    vote_b: Vote,
    total_voting_power: Power,
    validator_power: Power,
    timestamp: Time,
}

impl TryFrom<RawDuplicateVoteEvidence> for DuplicateVoteEvidence {
    type Error = Error;

    fn try_from(value: RawDuplicateVoteEvidence) -> Result<Self, Self::Error> {
        Ok(Self {
            vote_a: value
                .vote_a
                .ok_or(error::missing_evidence_error(anyhow::anyhow!(
                    "missing evidence error"
                )))?
                .try_into()?,
            vote_b: value
                .vote_b
                .ok_or(error::missing_evidence_error(anyhow::anyhow!(
                    "missing evidence error"
                )))?
                .try_into()?,
            total_voting_power: value.total_voting_power.try_into()?,
            validator_power: value.validator_power.try_into()?,
            timestamp: value
                .timestamp
                .ok_or(error::missing_timestamp_error(anyhow::anyhow!(
                    "missing timestamp error"
                )))?
                .try_into()
                .map_err(|e: std::convert::Infallible| {
                    error::in_fallible_error(anyhow::anyhow!(e))
                })?,
        })
    }
}

impl From<DuplicateVoteEvidence> for RawDuplicateVoteEvidence {
    fn from(value: DuplicateVoteEvidence) -> Self {
        RawDuplicateVoteEvidence {
            vote_a: Some(value.vote_a.into()),
            vote_b: Some(value.vote_b.into()),
            total_voting_power: value.total_voting_power.into(),
            validator_power: value.total_voting_power.into(),
            timestamp: Some(value.timestamp.into()),
        }
    }
}

impl DuplicateVoteEvidence {
    /// constructor
    pub fn new(vote_a: Vote, vote_b: Vote) -> Result<Self, Error> {
        if vote_a.height != vote_b.height {
            return Err(error::invalid_evidence_error(anyhow::anyhow!(
                "invalid evidence error"
            )));
        }
        // Todo: make more assumptions about what is considered a valid evidence for duplicate vote
        Ok(Self {
            vote_a,
            vote_b,
            total_voting_power: Default::default(),
            validator_power: Default::default(),
            timestamp: Time::now(),
        })
    }
    /// Get votes
    pub fn votes(&self) -> (&Vote, &Vote) {
        (&self.vote_a, &self.vote_b)
    }
}

/// Conflicting headers evidence.
// Todo: This struct doesn't seem to have a protobuf definition.
#[derive(Clone, Debug, PartialEq)]
pub struct ConflictingHeadersEvidence {
    //#[serde(rename = "H1")]
    h1: SignedHeader,
    //#[serde(rename = "H2")]
    h2: SignedHeader,
}

impl ConflictingHeadersEvidence {
    /// Create a new evidence of conflicting headers
    pub fn new(h1: SignedHeader, h2: SignedHeader) -> Self {
        Self { h1, h2 }
    }
}

/// Evidence data is a wrapper for a list of `Evidence`.
///
/// <https://github.com/tendermint/spec/blob/d46cd7f573a2c6a2399fcab2cde981330aa63f37/spec/core/data_structures.md#evidencedata>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "RawEvidenceList", into = "RawEvidenceList")]
pub struct Data {
    evidence: Option<Vec<Evidence>>,
}

impl TryFrom<RawEvidenceList> for Data {
    type Error = Error;
    fn try_from(value: RawEvidenceList) -> Result<Self, Self::Error> {
        if value.evidence.is_empty() {
            return Ok(Self { evidence: None });
        }
        let evidence: Result<Vec<Evidence>, Error> =
            value.evidence.into_iter().map(TryInto::try_into).collect();
        Ok(Self {
            evidence: Some(evidence?),
        })
    }
}

impl From<Data> for RawEvidenceList {
    fn from(value: Data) -> Self {
        RawEvidenceList {
            evidence: value
                .evidence
                .unwrap_or_default()
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl Data {
    /// Create a new evidence data collection
    pub fn new<I>(into_evidence: I) -> Data
    where
        I: Into<Vec<Evidence>>,
    {
        Data {
            evidence: Some(into_evidence.into()),
        }
    }

    /// Convert this evidence data into a vector
    pub fn into_vec(self) -> Vec<Evidence> {
        self.iter().cloned().collect()
    }

    /// Iterate over the evidence data
    pub fn iter(&self) -> slice::Iter<'_, Evidence> {
        self.as_ref().iter()
    }
}

impl AsRef<[Evidence]> for Data {
    fn as_ref(&self) -> &[Evidence] {
        self.evidence.as_deref().unwrap_or_else(|| &[])
    }
}

/// Evidence collection parameters
#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
// Todo: This struct is ready to be converted through tendermint_proto::types::EvidenceParams.
// https://github.com/informalsystems/tendermint-rs/issues/741
pub struct Params {
    /// Maximum allowed age for evidence to be collected
    #[serde(with = "serializers::from_str")]
    pub max_age_num_blocks: u64,

    /// Max age duration
    pub max_age_duration: Duration,

    /// Max bytes
    #[serde(with = "serializers::from_str", default)]
    pub max_bytes: i64,
}

impl Protobuf<RawEvidenceParams> for Params {}

impl TryFrom<RawEvidenceParams> for Params {
    type Error = Error;

    fn try_from(value: RawEvidenceParams) -> Result<Self, Self::Error> {
        Ok(Self {
            max_age_num_blocks: value.max_age_num_blocks.try_into().map_err(|_| {
                error::negative_max_age_num_error(anyhow::anyhow!("negative max age num error"))
            })?,
            max_age_duration: value
                .max_age_duration
                .ok_or(error::missing_max_age_duration_error(anyhow::anyhow!(
                    "missing max afe duration error"
                )))?
                .try_into()?,
            max_bytes: value.max_bytes,
        })
    }
}

impl From<Params> for RawEvidenceParams {
    fn from(value: Params) -> Self {
        Self {
            // Todo: Implement proper domain types so this becomes infallible
            max_age_num_blocks: value.max_age_num_blocks.try_into().unwrap(),
            max_age_duration: Some(value.max_age_duration.into()),
            max_bytes: value.max_bytes,
        }
    }
}

#[cfg(feature = "std")]
use std::time::Duration as NativeDuration;

#[cfg(not(feature = "std"))]
use core::time::Duration as NativeDuration;

/// Duration is a wrapper around std::time::Duration
/// essentially, to keep the usages look cleaner
/// i.e. you can avoid using serde annotations everywhere
/// Todo: harmonize google::protobuf::Duration, std::time::Duration and this. Too many structs.
/// <https://github.com/informalsystems/tendermint-rs/issues/741>
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Duration(#[serde(with = "serializers::time_duration")] pub crate::primitives::Duration);

impl From<Duration> for NativeDuration {
    fn from(d: Duration) -> NativeDuration {
        d.0
    }
}

impl Protobuf<RawDuration> for Duration {}

impl TryFrom<RawDuration> for Duration {
    type Error = Error;

    fn try_from(value: RawDuration) -> Result<Self, Self::Error> {
        Ok(Self(NativeDuration::new(
            value.seconds.try_into().map_err(|_| {
                error::integer_overflow_error(anyhow::anyhow!("integer overflow error"))
            })?,
            value.nanos.try_into().map_err(|_| {
                error::integer_overflow_error(anyhow::anyhow!("integer overflow error"))
            })?,
        )))
    }
}

impl From<Duration> for RawDuration {
    fn from(value: Duration) -> Self {
        // Todo: make the struct into a proper domaintype so this becomes infallible.
        Self {
            seconds: value.0.as_secs() as i64,
            nanos: value.0.subsec_nanos() as i32,
        }
    }
}
