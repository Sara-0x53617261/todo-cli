// Diesel imports - for Database / SQL
use diesel::prelude::*;
use diesel::SqliteConnection;

// Dotenv for Database location from our .env file
use dotenvy::dotenv;
use std::env;

// Local imports
use crate::models::{TodoItem, NewTodoItem};
use crate::schema::todo_items;

// Add new todo item to the database
pub fn create_todo_item(conn: &mut SqliteConnection, message: String) {
    let new_item = NewTodoItem {
        completed: false,
        message: String::from(message),
    };

    diesel::insert_into(todo_items::table)
        .values(&new_item)
        .execute(conn)
        .expect("Error inserting new item into database");
}

// Set item with todo_id as completed
pub fn set_todo_complete(conn: &mut SqliteConnection, todo_id: i32) {
    diesel::update(todo_items::table.find(todo_id))
        .set(todo_items::completed.eq(true))
        .execute(conn)
        .expect("Error updating item in database");
}

// Get todo items -> returns Vector
pub fn get_todo_items(conn: &mut SqliteConnection) -> Vec<TodoItem>{
    todo_items::table
        .limit(20)
        .load::<TodoItem>(conn)
        .expect("Error loading items")
}

// Removes all items that are set as 'completed == true' from the database
pub fn remove_completed(conn: &mut SqliteConnection) {
    diesel::delete(
        todo_items::table.filter(todo_items::completed.eq(true)))
        .execute(conn)
        .expect("Error deleting completed items");
}



/* DATABASE SETUP  */
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("No database url");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(
            |_| panic!("Error connecting to database {}", database_url)
        )
}
