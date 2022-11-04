use diesel::prelude::*;
use crate::schema::todo_items;


#[derive(Queryable)]
pub struct TodoItem {
    pub id: i32,
    pub completed: bool,
    pub message: String,
}

#[derive(Insertable)]
#[diesel(table_name = todo_items)]
pub struct NewTodoItem {
    pub completed: bool,
    pub message: String,
}