use common_infra::dynamodb_client::get_dynamodb_client;

use crate::util::dynamodb::{create_table, delete_table};

#[tokio::test]
#[ignore]
async fn example() {
    let client = get_dynamodb_client().await;
    create_table(client, "test-table").await.unwrap();
    delete_table(client, "test-table").await.unwrap();
}
