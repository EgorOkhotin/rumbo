pub mod prelude {
    pub(super) use mongodb::{
        options::ClientOptions, 
        Client, 
        Collection,
        bson::{doc, Document, oid::ObjectId}
    };

    // use from lib.rs
    pub use super::super::prelude::*;

    pub use super::DbAdapter;
    pub use super::{get_id_filter_from_object, get_id_filter_from_str};
}
use prelude::*;


#[derive(Clone)]
pub struct DbAdapter {
    client: Client
}

const DB_NAME: &'static str = "rumbo_app";

impl DbAdapter {
    pub async fn new(host: &str, app_name: &str) -> Result<Self> {
        let mut client_options = ClientOptions::parse(host).await?;
        
        // Manually set an option.
        client_options.app_name = Some(app_name.to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options)?;

        let result = DbAdapter {
            client: client
        };

        Ok(result)
    }

    pub fn get_collection<T>(&self, collection_name: &str) -> Collection<T> {
        self.client.database(DB_NAME).collection::<T>(collection_name)
    }
}

pub fn get_id_filter_from_str(id: &str) -> Document {
    let object_id = ObjectId::parse_str(id).unwrap();
    get_id_filter_from_object(&object_id)
}

pub fn get_id_filter_from_object(id: &ObjectId) -> Document {
    doc! {"_id": id }
}