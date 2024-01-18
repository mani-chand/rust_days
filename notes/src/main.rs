use axum::{routing::post,Router, Json};
use serde::Deserialize;

#[derive(Deserialize)]
struct Note{
    title:String,
    content:String
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/",post(create_note));
    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
}

async fn create_note(Json(note):Json<Note>)->String{
    let mut notes = vec![];
    let title = note.title;
    let content = note.content;
    notes.push(Note{title,content});
    format!("Received title: {} and content: {}", notes[0].title, notes[0].content)
}
