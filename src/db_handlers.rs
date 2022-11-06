use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenvy::dotenv;
use std::env;

use crate::models::{TodoItem, NewTodoItem};
use crate::schema::todo_items;

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

pub fn set_todo_complete(conn: &mut SqliteConnection, todo_id: i32) {
    diesel::update(todo_items::table.find(todo_id))
        .set(todo_items::completed.eq(true))
        .execute(conn)
        .expect("Error updating item in database");
}

pub fn get_todo_items(conn: &mut SqliteConnection) -> Vec<TodoItem>{
    todo_items::table
        .limit(20)
        .load::<TodoItem>(conn)
        .expect("Error loading items")
}

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
