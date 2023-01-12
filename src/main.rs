pub mod protos;
pub mod schema;

use std::sync::Arc;

use juniper::{EmptyMutation, EmptySubscription, RootNode};
use protos::quotes::quotes_service_client::QuotesServiceClient;
use rocket::{response::content, State};
use tokio::sync::Mutex;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let quotes_service = QuotesServiceClient::connect("http://[::1]:50051").await?;

    let cors = rocket_cors::CorsOptions::default()
        .to_cors()
        .expect("error while building CORS");

    let _ = rocket::build()
        .manage(schema::Context {
            quotes_service: Arc::new(Mutex::new(quotes_service)),
        })
        .manage(schema())
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .mount("/", rocket_cors::catch_all_options_routes())
        .attach(cors.clone())
        .manage(cors)
        .launch()
        .await
        .expect("server to launch");
    Ok(())
}

type Schema = RootNode<
    'static,
    schema::Query,
    EmptyMutation<schema::Context>,
    EmptySubscription<schema::Context>,
>;

fn schema() -> Schema {
    Schema::new(
        schema::Query,
        EmptyMutation::<schema::Context>::new(),
        EmptySubscription::<schema::Context>::new(),
    )
}

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
async fn get_graphql_handler(
    context: &State<schema::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql_handler(
    context: &State<schema::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
}
