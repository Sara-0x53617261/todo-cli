pub mod models;
pub mod schema;
pub mod ascii;

use diesel::prelude::*;
use diesel::SqliteConnection;

use models::NewTodoItem;
use schema::todo_items;

pub fn create_todo_item(conn: &mut SqliteConnection, message: String) {
    use crate::schema::todo_items;

    let new_item = NewTodoItem {
        completed: false,
        message: String::from(message)
    };

    diesel::insert_into(todo_items::table)
        .values(&new_item)
        .execute(conn)
        .expect("Error savine new todo item");

        //.get_result(conn)
        //.expect("Error saving new todo item!")
}

pub fn set_todo_complete(conn: &mut SqliteConnection, todo_id: i32) {
    diesel::update(todo_items::table.find(todo_id))
        .set(todo_items::completed.eq(true))
        .execute(conn)
        .expect("Item with id: '{todo_id}' not found!");
}
