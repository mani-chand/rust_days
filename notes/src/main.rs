use axum::{extract::Json, routing::{post,get}, Router};
use mongodb::{options::ClientOptions, Client, bson::doc};

#[derive(serde::Deserialize,Debug)]
struct Note {
    title: String,
    content: String,
}

async fn create_note(Json(note): Json<Note>) {
    let title = note.title;
    let content = note.content;
    
    // Assuming you have a MongoDB collection named "notes"
    let client_options = ClientOptions::parse("mongodb+srv://manichand:root@cluster0.oll6q.mongodb.net/?retryWrites=true&w=majority").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("Rust_notes");
    let collection = db.collection("Notes");

    // Create a BSON document to insert into the MongoDB collection
    let document = doc! {
        "title": title,
        "content": content,
    };

    // Insert the document into the collection
    collection.insert_one(document, None).await.unwrap();

    // Optionally, you might want to return a response indicating success
    // For simplicity, this example returns a plain string
}


async fn get_all_notes()->Json<Note>{
    let client_options = ClientOptions::parse("mongodb+srv://manichand:root@cluster0.oll6q.mongodb.net/?retryWrites=true&w=majority").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("Rust_notes");
    let collection = db.collection::<Note>("Notes");
    let cursor = collection.find(None, None).await.unwrap();
    // println!("{:?}",cursor);
    // Json(cursor)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/create_note", post(create_note)).route("/",get(get_all_notes));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
