use serde::Serialize;

use crate::client::Client;
use crate::Result;

pub struct ReplicateRequest {
    client: Client,
    payload: ReplicatePayload,
}

impl ReplicateRequest {
    pub(crate) fn new(client: impl Into<Client>, target: impl Into<String>) -> Self {
        let client = client.into();
        let source = client.url().path();
        let payload = ReplicatePayload::new(source, target);
        ReplicateRequest {
            client: client.into(),
            payload,
        }
    }

    pub async fn send() -> Result<Replication> {
        unimplemented!()
    }
}

pub struct Replication {
    client: Client,
    payload: ReplicatePayload,
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
    fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        ReplicatePayload {
            cancel: false,
            continuous: false,
            create_target: false,
            doc_ids: Vec::default(),
            filter: None,
            proxy: None,
            source: source.into(),
            target: target.into(),
        }
    }
}
