use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("group")
        .required(false)
        .args(&["add", "list", "remove", "complete"])
))]
pub struct CliArgs {

    /// The message for your new todo item to display
    #[arg(short, long, required = false)]
    pub add: Option<String>,

    /// Toggle - lists your current todo items
    #[arg(short, long, default_value_t = false, required = false)]
    pub list: bool,

    /// Remove all completed todo items from your list
    #[arg(short, long, default_value_t = false, required = false)]
    pub remove: bool,

    /// Sets the todo item linked to the given ID as completed
    #[arg(short, long, required = false)]
    pub complete: Option<i32>,
}