use serde::{Deserialize, Serialize};



        use crate::inner_client::InnerClient;

pub struct ReplicateRequest {
    client: InnerClient,
    payload: ReplicatePayload
}

impl ReplicateRequest {
    pub fn new(client: &InnerClient) {
        
    }
}

    pub struct Replication {
    client: InnerClient,
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

    #[serde(skip_serializing_if = "is_true")]
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
