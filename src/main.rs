mod context;
mod models;
mod schema;
//mod mongodb;

//use crate::mongodb::mongodb;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};

use crate::schema::{build_schema, ToDosSchema};

async fn graphql_handler(schema: Extension<ToDosSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

async fn schema_main() {
    let schema = build_schema();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .layer(Extension(schema));

    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    //mongodb().await;
    schema_main().await;
}

//-> Result<(), Box<dyn Error>>  {
/*

    let collection: mongodb::Collection<ToDo> = client.database("rust").collection("todos");
    let coll = client.database("rust").collection::<ToDo>("in_stock");

    for i in 0..5 {
    let coll_ref = coll.clone();

    // Spawn several tasks that operate on the same collection concurrently.
    todo::spawn(async move {
        // Perform operations with `coll_ref` that work with directly our model.
        coll_ref.insert_one(ToDo { id: i, descrition: "", completed: false, }, None).await?;
    });
*/
