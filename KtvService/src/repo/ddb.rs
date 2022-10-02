
use aws_sdk_dynamodb::Client;
use aws_config::SdkConfig;
use log::{debug, error};
use aws_sdk_dynamodb::error::{PutItemError, ScanError};
use aws_sdk_dynamodb::output::PutItemOutput;
use aws_sdk_dynamodb::types::SdkError;
use serde_dynamo::aws_sdk_dynamodb_0_19::from_items;
use serde_dynamo::to_item;
use crate::model::song::{Song};

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
    PutError(#[from] SdkError<PutItemError>),
    #[error("Object parsing error.")]
    ParsingError(#[from] serde_dynamo::Error),

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

    pub async fn put_song(&self, song: Song) -> Result<PutItemOutput, DdbError> {
        let item = to_item(song)?;
        let request = self.client.put_item()
            .table_name(&self.table_name)
            .set_item(Some(item));

        let response = request.send()
            .await?;

        Ok(response)
    }


}