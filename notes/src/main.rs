use axum::{extract::Json, routing::{post,get}, Router};
use mongodb::{results::DeleteResult,options::ClientOptions, Client, bson::doc};
use serde::{Serialize,Deserialize};
use futures::stream::TryStreamExt;
//use oid::prelude::*;
use uuid::Uuid;
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Note {
    nid:String,
    title: String,
    content: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Id{
    uid:String,
}
async fn create_note(Json(note): Json<Note>) {
    let title = note.title;
    let content = note.content;
    let client_options = ClientOptions::parse("mongodb+srv://manichand:root@cluster0.oll6q.mongodb.net/?retryWrites=true&w=majority").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("Rust_notes");
    let collection = db.collection("Notes");
    let id = Uuid::new_v4();
    print!("{:?}",id);
    let document = doc! {
        "nid":id.to_string(),
        "title": title,
        "content": content,
    };
    print!("{:?}",document);
    let s = collection.insert_one(document, None).await.unwrap();
    print!("{:?}",s);

}


async fn get_all_notes()->Json<Vec<Note>>{
    let client_options = ClientOptions::parse("mongodb+srv://manichand:root@cluster0.oll6q.mongodb.net/?retryWrites=true&w=majority").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("Rust_notes");
    let collection = db.collection::<Note>("Notes");
    let mut cursor = collection.find(None, None).await.expect("Failed to execute query");
    let mut models:Vec<Note> = Vec::new();
    // Iterate over the cursor and deserialize documents into MyModel
    while let Some(book) = cursor.try_next().await.unwrap(){
        models.push(book);
    }
    Json(models)
}
async fn get_note_by_id(Json(id): Json<Id>)->Json<Option<Note>>{
    let client_options = ClientOptions::parse("mongodb+srv://manichand:root@cluster0.oll6q.mongodb.net/?retryWrites=true&w=majority").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("Rust_notes");
    let collection = db.collection::<Note>("Notes");
    let result = collection.find_one(
        doc! { "nid":id.uid},
        None
    ).await.expect("Failed to execute query");
    println!("{:#?}", result);
    Json(result)
}


async fn delete_note(Json(id): Json<Id>)->Json<DeleteResult>{
    let client_options = ClientOptions::parse("mongodb+srv://manichand:root@cluster0.oll6q.mongodb.net/?retryWrites=true&w=majority").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("Rust_notes");
    let collection = db.collection::<Note>("Notes");
    let result = collection.delete_one(
        doc! { "nid":id.uid},
        None
    ).await.expect("Failed to execute query");
    println!("{:#?}", result);
    Json(result)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/create_note", post(create_note))
    .route("/note", post(delete_note))
    .route("/",get(get_all_notes))
    .route("/id",get(get_note_by_id));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}