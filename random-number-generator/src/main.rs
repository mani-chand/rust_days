use axum::{extract::Query, response::Html, routing::get, Router};
use rand::{thread_rng, Rng};
use serde::Deserialize;

#[tokio::main]
async fn main(){
    let app = Router::new().route("/",get(handler));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
}
#[derive(Deserialize)]
struct RangeParameter{
    start:i32,
    end: i32
}
async fn handler(Query(range): Query<RangeParameter>) -> Html<String>{
    let random_number = thread_rng().gen_range(range.start..range.end);
    Html(format!("<h1>Random number : {} <h1>",random_number))
}
