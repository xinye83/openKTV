use std::collections::HashMap;
use actix_web::web::{Json, Query};
use aws_sdk_dynamodb::Client;
use aws_config::SdkConfig;
use log::{debug, error, info};
use aws_sdk_dynamodb::error::{GetItemError, PutItemError, QueryError, ScanError};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::output::PutItemOutput;
use aws_sdk_dynamodb::types::SdkError;
use serde_dynamo::aws_sdk_dynamodb_0_19::{from_item, from_items};
use serde_dynamo::{to_attribute_value, to_item};
use crate::ApiError::DbError;
use crate::model::song::{Song};
use crate::SongApi;

pub struct DDBRepository {
    client: Client,
    table_name: String,
}


#[derive(thiserror::Error, Debug)]
pub enum DdbError {
    #[error("DB error")]
    DynamoError(#[from] aws_sdk_dynamodb::Error),
    #[error("DB scan error")]
    ScanError(#[from] SdkError<ScanError>),
    #[error("DB put item error")]
    PutItemError(#[from] SdkError<PutItemError>),
    #[error("DB get item error")]
    GetItemError(#[from] SdkError<GetItemError>),
    #[error("DB get item error")]
    QueryError(#[from] SdkError<QueryError>),
    #[error("Object parsing error.")]
    ParsingError(#[from] serde_dynamo::Error),
    #[error("Item not found in DB")]
    ItemNotFoundItem(String)
}


impl DDBRepository {
    pub fn init(table_name: String, config: SdkConfig) -> DDBRepository {
        let client = Client::new(&config);
        DDBRepository {
            table_name,
            client,
        }
    }

    pub async fn get_songs(&self) -> Result<Vec<Song>, DdbError> {
        let request = self.client.scan()
            .table_name(&self.table_name);

        let response = request.send().await?;

        let songs:Vec<Song> = from_items(response.items.unwrap())?;
        Ok(songs)
    }

    pub async fn put_song(&self, song: &Song) -> Result<PutItemOutput, DdbError> {
        let item = to_item(song)?;
        let request = self.client.put_item()
            .table_name(&self.table_name)
            .set_item(Some(item));

        let response = request.send()
            .await?;

        Ok(response)
    }

    pub async fn query_songs(&self, query: Json<SongApi>) -> Result<Vec<Song>, DdbError> {
        // if query has uuid, run query by ID and return
        if let Some(uuid) = query.uuid.clone() {
            let item = self.client
                .get_item()
                .table_name(&self.table_name)
                .key("uuid", AttributeValue::S(uuid.clone()))
                .send()
                .await?;

            return match item.item {
                None => Err(DdbError::ItemNotFoundItem(uuid)),
                Some(it) => {
                    let song: Song = from_item(it)?;
                    Ok(vec![song])
                },
            }
        }

        let mut condition: Vec<String> = Vec::new();

        // Declare all of the expression inputs for a query call
        let mut expression_attribute_values = HashMap::new();

        if let Some(name) = query.name.clone() {
            condition.push("name = :name".to_string());
            expression_attribute_values.insert(":name".to_string(), to_attribute_value(name)?);
        }

        if let Some(artist) = query.artist.clone() {
            condition.push("artist = :artist".to_string());
            expression_attribute_values.insert(":artist".to_string(), to_attribute_value(artist)?);
        }

        let key_condition_exp = condition.join(" AND ");

        info!("Query filter_expression={}", &key_condition_exp);

        let request = self.client.
            scan()
            .table_name(&self.table_name)
            .filter_expression(key_condition_exp)
            .set_expression_attribute_values(Some(expression_attribute_values))
            .send()
            .await?;

        let songs:Vec<Song> = from_items(request.items.unwrap())?;
        Ok(songs)
    }


}