pub mod quotes;

use crate::protos::quotes::{quotes_service_client::QuotesServiceClient, QuoteRequest};
use juniper::{graphql_object, FieldResult};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Channel, Code};

#[derive(Clone, Debug)]
pub struct Context {
    pub quotes_service: Arc<Mutex<QuotesServiceClient<Channel>>>,
}
impl juniper::Context for Context {}

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    pub async fn quotes(id: Option<String>, context: &Context) -> FieldResult<Vec<quotes::Quote>> {
        let mut client = context.quotes_service.lock().await;
        let request = tonic::Request::new(QuoteRequest {
            id: id.unwrap_or_else(|| "627e007bab1ab6462c21a5d6".to_string()),
        });

        let response = client.get_quote(request).await;
        match response {
            Ok(response) => {
                let quotes = response.into_inner().quotes;
                Ok(quotes
                    .into_iter()
                    .map(|q| quotes::Quote { inner: q })
                    .collect())
            }
            Err(status) => match status.code() {
                Code::NotFound => Err(juniper::FieldError::new(
                    "Quote not found",
                    juniper::Value::null(),
                )),
                Code::InvalidArgument => Err(juniper::FieldError::new(
                    "Invalid argument",
                    juniper::Value::null(),
                )),
                _ => Err(juniper::FieldError::new(
                    "Internal server error",
                    juniper::Value::null(),
                )),
            },
        }
    }
}
