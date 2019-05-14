use crate::error::Error;
use crate::inner_client::InnerClient;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

pub struct GetRequest<'a> {
    id: String,
    client: &'a InnerClient,
    query: GetRequestQuery,
}

impl<'a> GetRequest<'a> {
    pub(crate) fn new(client: &'a InnerClient, id: impl Into<String>) -> Self {
        GetRequest {
            id: id.into(),
            client,
            query: GetRequestQuery::default(),
        }
    }

    pub fn send<T: DeserializeOwned>(&self) -> Result<GetResponse<T>, Error> {
        let response = self
            .client
            .join(&self.id)?
            .get()
            .query(&self.query)
            .send()?
            .json()?;
        Ok(response)
    }
}

#[derive(Serialize)]
struct GetRequestQuery {
    attachments: bool,
    att_encoding_info: bool,
    atts_since: Option<Vec<String>>,
    conflicts: bool,
    deleted_conflicts: bool,
    latest: bool,
    local_seq: bool,
    meta: bool,
    open_revs: Option<Vec<String>>,
    rev: Option<String>,
    revs: bool,
    revs_info: bool,
}

impl Default for GetRequestQuery {
    fn default() -> Self {
        GetRequestQuery {
            attachments: false,
            att_encoding_info: false,
            atts_since: None,
            conflicts: false,
            deleted_conflicts: false,
            latest: false,
            local_seq: false,
            meta: false,
            open_revs: None,
            rev: None,
            revs: false,
            revs_info: false,
        }
    }
}

#[derive(Deserialize)]
pub struct GetResponseMeta {
    pub _id: String,
    pub _rev: String,
    pub _deleted: Option<bool>,
    pub _attachments: Option<Value>,
    pub _conflicts: Option<Vec<Value>>,
    pub _deleted_conflicts: Option<Vec<Value>>,
    pub _local_seq: Option<String>,
    pub _revs_info: Option<Vec<Value>>,
    pub _revisions: Option<Value>,
}

#[derive(Deserialize)]
pub struct GetResponse<T> {
    #[serde(flatten)]
    pub document: T,

    #[serde(flatten)]
    pub metadata: GetResponseMeta,
}
