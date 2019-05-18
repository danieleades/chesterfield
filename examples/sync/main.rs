use chesterfield::sync::Client;
use serde_json::json;

mod docker;
use self::docker::{create_couchdb_container, get_unused_port, remove_couchdb_container};

// This example assumes you have docker installed!

fn main() {
    let couchdb_port = get_unused_port();
    println!("using host port {}", couchdb_port);

    let couchdb_url = format!("https://localhost:{}", couchdb_port);

    let container_id = create_couchdb_container(couchdb_port);

    let client = Client::new(couchdb_url).unwrap();
    let database = client.database("examples").unwrap();

    // TODO add methods for creating a database if it doesn't exist!

    let document = json!(
        {
            "field1": 5,
            "field2": "some string",
            "object" : {
                "values": [
                    "value1",
                    "value2",
                    "value3",
                ]
            }
        }
    );

    // finally, delete the couchdb container
    remove_couchdb_container(&container_id);
}
