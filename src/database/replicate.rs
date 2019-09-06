use serde::Serialize;

use crate::client::Client;
use crate::Result;

pub struct ReplicateRequest {
    client: Client,
    payload: ReplicatePayload,
}

impl ReplicateRequest {
    pub(crate) fn new_push_replication(
        client: impl Into<Client>,
        target: impl Into<String>,
    ) -> Self {
        let client = client.into();

        let source: String = client.url().path().into();
        let sink = target;

        ReplicateRequest::new(client, source, sink)
    }

    pub(crate) fn new_pull_replication(
        client: impl Into<Client>,
        target: impl Into<String>,
    ) -> Self {
        let client = client.into();

        let source = target;
        let sink: String = client.url().path().into();

        ReplicateRequest::new(client, source, sink)
    }

    fn new(client: impl Into<Client>, source: impl Into<String>, sink: impl Into<String>) -> Self {
        let client = client.into();

        let payload = ReplicatePayload::new(source, sink);

        ReplicateRequest { client, payload }
    }

    pub async fn send() -> Result<Replication> {
        unimplemented!()
    }
}

pub struct Replication {
    client: Client,
    payload: ReplicatePayload,
}

impl Replication {
    pub async fn cancel(&self) -> Result<()> {
        unimplemented!()
    }
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
    sink: String,
}

impl ReplicatePayload {
    fn new(source: impl Into<String>, sink: impl Into<String>) -> Self {
        ReplicatePayload {
            cancel: false,
            continuous: false,
            create_target: false,
            doc_ids: Vec::default(),
            filter: None,
            proxy: None,
            source: source.into(),
            sink: sink.into(),
        }
    }
}
