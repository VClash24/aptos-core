// Copyright © Aptos Foundation

pub mod models;
pub mod schema;
pub mod utils;
pub mod worker;

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use reqwest::{header, Client};

/// Establishes a connection pool to Postgres
pub fn establish_connection_pool(database_url: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

/// HEAD request to get MIME type and size of content
pub async fn get_uri_metadata(url: String) -> anyhow::Result<(String, u32)> {
    let client = Client::new();
    let request = client.head(&url);
    let response = request.send().await?;
    let headers = response.headers();

    let mime_type = headers
        .get(header::CONTENT_TYPE)
        .map(|value| value.to_str().unwrap_or("text/plain"))
        .unwrap_or("text/plain")
        .to_string();
    let size = headers
        .get(header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);

    Ok((mime_type, size))
}
