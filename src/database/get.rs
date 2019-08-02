use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::inner_client::InnerClient;
use crate::Error;
use futures::compat::Future01CompatExt;
use serde::de::DeserializeOwned;

/// A request to retrieve a document from a CouchDB database.
///
/// The request is lazy- it doesn't do a thing until you call its '[send](GetRequest::send)'
/// method.
///
/// see [CouchDB API docs](https://docs.couchdb.org/en/stable/api/document/common.html#get--db-docid)
/// for details.
pub struct GetRequest {
    id: String,
    client: InnerClient,
    query: GetRequestQuery,
}

impl GetRequest {
    pub(crate) fn new(client: &InnerClient, id: impl Into<String>) -> Self {
        GetRequest {
            id: id.into(),
            client: client.duplicate(),
            query: GetRequestQuery::default(),
        }
    }

    /// Includes attachments bodies in response.
    ///
    /// Default is false.
    pub fn attachments(mut self, value: bool) -> Self {
        self.query.attachments = value;
        self
    }

    /// Includes attachment encoding information in response.
    ///
    /// Default is false.
    pub fn attachment_encoding_info(mut self, value: bool) -> Self {
        self.query.att_encoding_info = value;
        self
    }

    /// Includes only the attachments since the specified revisions.
    /// Doesn’t include attachments for specified revisions
    ///
    /// Default is false.
    pub fn attachments_since(mut self, revisions: impl Into<Vec<String>>) -> Self {
        self.query.atts_since = revisions.into();
        self
    }

    /// Includes information about conflicts in document.
    ///
    /// Default is false.
    pub fn conflicts(mut self, value: bool) -> Self {
        self.query.conflicts = value;
        self
    }

    /// Includes information about deleted conflict revisions.
    ///
    /// Default is false
    pub fn deleted_conflicts(mut self, value: bool) -> Self {
        self.query.deleted_conflicts = value;
        self
    }

    /// Forces retrieving latest 'leaf' revision, no matter which revision
    /// was requested.
    ///
    /// Default is false.
    pub fn latest(mut self, value: bool) -> Self {
        self.query.latest = value;
        self
    }

    /// Includes last 'update sequence' for this document.
    ///
    /// The update sequence is specific to this node (in the case
    /// of a cluster of CouchDB nodes).
    ///
    /// Default is false.
    pub fn local_sequence(mut self, value: bool) -> Self {
        self.query.local_seq = value;
        self
    }

    /// This is the same as setting all of 'conflicts',
    /// 'deleted_conflicts', and 'revisions_info' to true.
    ///
    /// Default is false.
    pub fn meta(mut self, value: bool) -> Self {
        self.query.meta = value;
        self
    }

    /// retrieve documents of specified leaf revisions.
    pub fn open_revisions(mut self, revisions: impl Into<Vec<String>>) -> Self {
        self.query.open_revs = Some(OpenRevs::Revisions(revisions.into()));
        self
    }

    /// retrieve documents of all leaf revisions.
    ///
    /// Default is false.
    pub fn all_open_revisions(mut self, value: bool) -> Self {
        if value {
            self.query.open_revs = Some(OpenRevs::All("all"));
        } else {
            self.query.open_revs = None
        }
        self
    }

    /// retrieve document of specified revision.
    pub fn revision(mut self, revision: impl Into<Option<String>>) -> Self {
        self.query.rev = revision.into();
        self
    }

    /// Retrieve list of known document revisions.
    ///
    /// Default is false.
    pub fn revisions(mut self, value: bool) -> Self {
        self.query.revs = value;
        self
    }

    /// included detailed information for all know document revisions.
    ///
    /// Default is false.
    pub fn revisions_info(mut self, value: bool) -> Self {
        self.query.revs_info = value;
        self
    }

    /// Send the request.
    ///
    /// This will consume the 'get' request and return a [GetResponse](GetResponse).
    /// The response is generic, so occasionally you might need type annotations.
    pub async fn send<T: DeserializeOwned>(self) -> Result<GetResponse<T>, Error> {
        let response = self
            .client
            .join(&self.id)?
            .get()
            .query(&self.query)
            .send()
            .compat()
            .await?
            .json()
            .compat()
            .await?;

        Ok(response)
    }
}

#[derive(Serialize, Clone)]
pub struct GetRequestQuery {
    attachments: bool,
    att_encoding_info: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    atts_since: Vec<String>,
    conflicts: bool,
    deleted_conflicts: bool,
    latest: bool,
    local_seq: bool,
    meta: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_revs: Option<OpenRevs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rev: Option<String>,
    revs: bool,
    revs_info: bool,
}

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
enum OpenRevs {
    Revisions(Vec<String>),
    All(&'static str),
}

impl Default for GetRequestQuery {
    fn default() -> Self {
        GetRequestQuery {
            attachments: false,
            att_encoding_info: false,
            atts_since: Vec::default(),
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

#[derive(Debug, Deserialize)]
pub struct GetResponseMeta {
    pub _id: String,
    pub _rev: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub _deleted: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub _attachments: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub _conflicts: Option<Vec<Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub _deleted_conflicts: Option<Vec<Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub _local_seq: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub _revs_info: Option<Vec<Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub _revisions: Option<Value>,
}

/// A response from a GetRequest.
///
/// The response is generic over a type parameter T. You can use this
/// to strongly type the response. Alternatively you can use the
/// default generic parameter of serde_json::Value, which can represent
/// any valid response from the database.
///
/// The GetResponse implements Deref with respect to the returned document.
/// You can also consume the response and retrieve the document with the [into_inner](GetResponse::into_inner)
/// method.
#[derive(Debug, Deserialize)]
pub struct GetResponse<T = Value> {
    #[serde(flatten)]
    document: Option<T>,

    #[serde(flatten)]
    meta_data: GetResponseMeta,
}

impl<T> GetResponse<T> {
    /// Return metadata about the response.
    ///
    /// Which metadata is available will depend on the options set
    /// in the request.
    pub fn meta_data(&self) -> &GetResponseMeta {
        &self.meta_data
    }

    /// Consume the response and return the contained document
    pub fn into_inner(self) -> Option<T> {
        self.document
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
