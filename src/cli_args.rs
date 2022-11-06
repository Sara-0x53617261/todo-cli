use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("group")
        .required(false)
        .args(&["add", "list", "remove", "complete"])
))]
pub struct CliArgs {

    #[arg(short, long, required = false)]
    pub add: Option<String>,

    #[arg(short, long, default_value_t = false, required = false)]
    pub list: bool,

    #[arg(short, long, default_value_t = false, required = false)]
    pub remove: bool,

    #[arg(short, long, required = false)]
    pub complete: Option<i32>,
}