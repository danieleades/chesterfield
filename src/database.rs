mod get;
mod insert;
pub use self::get::GetResponse;
pub use self::insert::InsertResponse;

pub mod sync {
    pub use super::{get::sync::GetRequest, insert::sync::InsertRequest};
    use crate::inner_client::sync::InnerClient;
    use serde::Serialize;

    pub struct Database {
        client: InnerClient,
    }

    impl Database {
        pub(crate) fn new(client: InnerClient) -> Self {
            Database { client }
        }

        pub fn get(&self, id: impl Into<String>) -> GetRequest {
            GetRequest::new(&self.client, id)
        }

        pub fn insert<T: Serialize>(
            &self,
            document: T,
            id: impl Into<Option<String>>,
        ) -> InsertRequest<T> {
            InsertRequest::new(&self.client, document, id)
        }
    }
}

pub mod r#async {
    pub use super::{get::r#async::GetRequest, insert::r#async::InsertRequest};
    use crate::inner_client::r#async::InnerClient;
    use serde::Serialize;

    pub struct Database {
        client: InnerClient,
    }

    impl Database {
        pub(crate) fn new(client: InnerClient) -> Self {
            Database { client }
        }

        pub fn get(&self, id: impl Into<String>) -> GetRequest {
            GetRequest::new(&self.client, id)
        }

        pub fn insert<T: Serialize>(
            &self,
            document: T,
            id: impl Into<Option<String>>,
        ) -> InsertRequest<T> {
            InsertRequest::new(&self.client, document, id)
        }
    }
}
