use crate::db::core::handle_input;
use crate::todos::todos::Todo;
use std::process;

#[derive(Debug)]
enum InputType {
    Number(usize),
    Text(String),
}

 fn print_static_options () {
    println!("1. Add Todo");
    println!("2. Edit Todo");
    println!("3. Delete Todo");
    println!("4. Mard Todo as completed");
    println!("5. Exit");
}

pub fn print_options() -> () {
    print_static_options();
    loop {
        print!("Enter: ");

        let input = match handle_input() {
            Ok(value) => value.trim().to_string(),
            _ => {
                println!("Invalid input");
                continue;
            }
        };

        let value = parse_input(input);

        match value {
            InputType::Number(value) => {
                handle_options(value);
            }
            _ => {
                println!("Invalid Option");
            }
        }
    }
}

fn parse_input(input: String) -> InputType {
    if let Ok(num) = input.parse::<usize>() {
        InputType::Number(num)
    } else {
        InputType::Text(input)
    }
}

pub fn handle_options(option: usize) {
    match option {
        option if option == 1 => {
            handle_add_todo();
            print_static_options();
        }
        option if option == 2 => {
            println!("Enter the task id: ");
            println!("Enter the task");
        }
        option if option == 3 => {
            println!("Are you sure to delete the task?");
            println!("Enter task id which you want to delete: ")
        }
        option if option == 4 => {
            println!("Marked todo as completed!")
        }
        option if option == 5 => {
            println!("Bye!");
            process::exit(1)
        }
        _ => {
            println!("Invalid Option");
        }
    }
}

fn handle_add_todo() {
    println!("Enter Task: ");
    let input = match handle_input() {
        Ok(value) => value.trim().to_string(),
        _ => {
            println!("Invalid input");
            return;
        }
    };

    let todo = Todo::new(input);

    println!("{:#?}", todo);
}
