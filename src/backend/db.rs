use crate::backend::model::Teacher;
use leptos::prelude::ServerFnError;
use leptos::server;
use std::sync::OnceLock;
use surrealdb::engine::any::{connect, Any};
use surrealdb::Surreal;

pub static DB: OnceLock<Surreal<Any>> = OnceLock::new();

// Initialize once at startup
pub async fn init_db() -> Result<(), ServerFnError> {
    let db = connect("ws://localhost:8000").await?;

    db.use_ns("older").use_db("school").await?;

    // Sign in with credentials
    db.signin(surrealdb::opt::auth::Root {
        username: "root".to_string(),
        password: "root".to_string(),
    })
    .await?;

    DB.set(db).expect("Failed to set database");

    Ok(())
}

// Use anywhere in your app
pub fn get_db() -> &'static Surreal<Any> {
    DB.get().expect("Database not initialized")
}

#[server(GetTeachers, "/api")]
pub async fn get_all_teachers() -> Result<Vec<Teacher>, ServerFnError> {
    let db = get_db();
    let teachers: Vec<Teacher> = db.select("teacher").await?;
    println!("Fetched teachers: {:?}", teachers);
    Ok(teachers)
}
