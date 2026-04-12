use std::fs::{self, File};
use std::io::Write;
use std::process;

use crate::todos::todos::Todo;
use serde::{Deserialize,Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoList {
    pub todos: Vec<Todo>
}

impl TodoList {
    pub fn new() -> Self {
        let path = match dotenvy::var("JSON_PATH") {
            Ok(path) => path,
            Err(_) => {
                println!("ENV Not found");
                let fallback = "../tasks.json".to_string();

                let mut file = match File::create(&fallback) {
                    Ok(file) => file,
                    Err(_) => {
                        println!("Failed to create DB file");
                        process::exit(1);
                    }
                };

                let _ = file.write_all(r#"{"todos": []}"#.as_bytes());

                fallback
            }
        };

        let data = match fs::read(&path) {
            Ok(v) => v,
            Err(_) => {
                println!("Failed to read a file!");
                process::exit(1);
            }
        };

        let todos: TodoList = match serde_json::from_slice(&data) {
            Ok(t) => t,
            Err(_) => {
                println!("Invalid JSON format!");
                process::exit(1);
            }
        };

        todos
    }

    pub fn write(&self) {
        let path = dotenvy::var("JSON_PATH")
            .unwrap_or("../tasks.json".to_string());

        let json = match serde_json::to_string_pretty(self) {
            Ok(j) => j,
            Err(e) => {
                println!("Serialization error: {:?}", e);
                process::exit(1);
            }
        };

        if let Err(e) = fs::write(&path, json) {
            println!("Failed to write file: {:?}", e);
            process::exit(1);
        }
    }
}