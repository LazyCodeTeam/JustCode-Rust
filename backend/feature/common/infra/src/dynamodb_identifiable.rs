pub trait DynamoDbIdentifiable {
    fn pk(&self) -> String;

    fn sk(&self) -> String;
}
