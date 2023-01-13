pub mod quotes;

use crate::protos::quotes::{
    quotes_service_client::QuotesServiceClient, LimitFilter, QuoteRequest, SearchQuotesRequest,
};
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
    pub async fn quotes(
        context: &Context,
        limit: Option<i32>,
        skip: Option<i32>,
        random: Option<bool>,
    ) -> FieldResult<Vec<quotes::Quote>> {
        let mut client = context.quotes_service.lock().await;
        let request = SearchQuotesRequest {
            limit: Some(LimitFilter {
                limit: limit.unwrap_or(20),
                skip: skip.unwrap_or(0),
            }),
            random,
            ..Default::default()
        };
        let request = tonic::Request::new(request);

        let response = client.search_quotes(request).await;
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
