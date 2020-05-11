use chesterfield::Client;
use couchdb_container::CouchDbContainer;

#[tokio::test]
async fn run() {
    // Create the couchdb instance
    let couchdb = CouchDbContainer::new().await.unwrap();
    let url = format!("http://localhost:{}", couchdb.port());

    let client = Client::from_url_str(url).unwrap();

    // create a client for a specific database
    let database = client.database("items").unwrap();

    // create the database in the remote CouchDB instance
    database.create().await.expect("unable to create database!");

    // Clean up CouchDB instance
    couchdb.delete().await.unwrap();
}
