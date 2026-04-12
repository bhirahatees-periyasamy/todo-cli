use dotenvy;
use crate::cli::ui::print_options;

mod todos;
mod cli; 
mod db;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    dotenvy::dotenv()?;



    print_options();


    Ok(())
}
