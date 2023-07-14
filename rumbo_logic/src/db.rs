use super::prelude::*;
use mongodb::{options::ClientOptions, Client, Collection};

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