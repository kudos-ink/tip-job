use std::env;

use reqwest;

use dotenv::dotenv;
use notion_client::endpoints::{
    databases::query::request::{
        Filter, FilterType, PropertyCondition, QueryDatabaseRequest, SelectCondition, Sort,
        SortDirection, Timestamp,
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let notion_api_key =
        env::var("NOTION_API_KEY").expect("NOTION_API_KEY not set in the environment");
    let notion_database_id =
        env::var("NOTION_DATABASE_ID").expect("NOTION_DATABASE_ID not set in the environment");

    let client = Client::new(notion_api_key.to_string());
    let Ok(client) = client else {
        panic!("error");
    };
    // TODO:
    // 1 - loop all the issues
    // 2 - call issues-api to filter them
    // 3 - check one by one if they have our commands in the comments //TODO: e-tag?
    // 4 - if it has it, make a POST to our backend

    let request = QueryDatabaseRequest {
        ..Default::default()
    };

    // Send request
    let res = client
        .databases
        .query_a_database(&notion_database_id, request)
        .await;

    // See result
    print!("{:#?}", res);

    Ok(())
}
