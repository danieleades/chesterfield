use serde::{Deserialize, Serialize};

use crate::client::Client;

pub struct ReplicateRequest {
    client: Client,
    payload: ReplicatePayload,
}

impl ReplicateRequest {
    pub fn new(client: &Client) {}
}

pub struct Replication {
    client: Client,
    payload: ReplicatePayload,
}

fn is_true(value: &bool) -> bool {
    *value
}

fn is_false(value: &bool) -> bool {
    !*value
}

#[derive(Serialize)]
pub struct ReplicatePayload {
    #[serde(skip_serializing_if = "is_false")]
    cancel: bool,

    #[serde(skip_serializing_if = "is_false")]
    continuous: bool,

    #[serde(skip_serializing_if = "is_false")]
    create_target: bool,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    doc_ids: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    proxy: Option<String>,

    source: String,
    target: String,
}

impl ReplicatePayload {
    fn new(source: String, target: String) -> Self {
        ReplicatePayload {
            cancel: false,
            continuous: false,
            create_target: false,
            doc_ids: Vec::default(),
            filter: None,
            proxy: None,
            source,
            target,
        }
    }
}