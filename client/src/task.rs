use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Task {
    pub id: String,
    pub account_id: Uuid,
    pub name: String,
    pub vdaf: Vdaf,
    pub min_batch_size: u64,
    pub max_batch_size: Option<u64>,
    pub batch_time_window_size_seconds: Option<u64>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
    pub time_precision_seconds: u32,
    #[deprecated = "Not populated. Will be removed in a future release."]
    pub report_count: u32,
    #[deprecated = "Not populated. Will be removed in a future release."]
    pub aggregate_collection_count: u32,
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub expiration: Option<OffsetDateTime>,
    pub leader_aggregator_id: Uuid,
    pub helper_aggregator_id: Uuid,
    pub collector_credential_id: Uuid,
    pub report_counter_interval_collected: i64,
    pub report_counter_decode_failure: i64,
    pub report_counter_decrypt_failure: i64,
    pub report_counter_expired: i64,
    pub report_counter_outdated_key: i64,
    pub report_counter_success: i64,
    pub report_counter_too_early: i64,
    pub report_counter_task_expired: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct NewTask {
    pub name: String,
    pub leader_aggregator_id: Uuid,
    pub helper_aggregator_id: Uuid,
    pub vdaf: Vdaf,
    pub min_batch_size: u64,
    pub max_batch_size: Option<u64>,
    pub batch_time_window_size_seconds: Option<u64>,
    pub time_precision_seconds: u64,
    pub collector_credential_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Vdaf {
    #[serde(rename = "count")]
    Count,

    #[serde(rename = "histogram")]
    Histogram(Histogram),

    #[serde(rename = "sum")]
    Sum { bits: u8 },

    #[serde(rename = "count_vec")]
    CountVec {
        length: u64,
        chunk_length: Option<u64>,
    },

    #[serde(rename = "sum_vec")]
    SumVec {
        bits: u8,
        length: u64,
        chunk_length: Option<u64>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum Histogram {
    Categorical {
        buckets: Vec<String>,
        chunk_length: Option<u64>,
    },
    Continuous {
        buckets: Vec<u64>,
        chunk_length: Option<u64>,
    },
    Length {
        length: u64,
        chunk_length: Option<u64>,
    },
}
