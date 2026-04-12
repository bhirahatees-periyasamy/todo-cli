use dotenvy;
use crate::cli::ui::print_options;

mod todos;
mod cli; 
mod db;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    dotenvy::dotenv()?;

    let path = dotenvy::var("JSON_PATH")?;

    println!("Path: {:?}", path);

    print_options();


    Ok(())
}
