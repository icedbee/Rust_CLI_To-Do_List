/// Concepts Covered: Vectors, strings, enums, basic file I/O (std::fs)
///
/// What You'll Build: A command-line application that lets users add 
/// tasks, list all tasks, and mark them as complete. Tasks persist 
/// between sessions using a simple text file.
///
/// This introduces you to Rust's owned types (String vs &str),
/// working with collections, and basic file operations. You'll define
/// your first custom types and learn how data ownership works when
/// passing values between functions.
///
/// Tip: Start with an in-memory implementation, then add file persistence.

use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
};

enum List {
    Complete(String),
    Todo(String),
}

fn read_file(list: &mut Vec<List>) -> io::Result<()> {
    let file = File::open("to-do_list.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if let Some(done) = line.strip_prefix("COMPLETE- ") {
            list.push(List::Complete(done.to_string()));
        } else {
            list.push(List::Todo(line));
        }
    }
    Ok(())
}

fn main() -> Result<(), String> {
    println!("Start your to-do list!\n");
    let mut list: Vec<List> = Vec::new();

    match read_file(&mut list) {
        Ok(()) => println!("Your to-do list has been loaded!"),
        Err(_) => println!("You haven't created a to-do list before :("),
    }

    loop {
        let mut input = String::new();

        println!("What would you like to do with your to-do list? (Enter the corresponding number)");
        println!("1. Add Task");
        println!("2. List All Tasks");
        println!("3. Mark a Task as Complete");
        println!("4. Quit");

        if io::stdin().read_line(&mut input).is_err() {
            return Err(String::from("Could not read input."));
        }

        input = input.trim().to_string();

        if input == String::from("1") {
            println!("Enter the task you want to add:");
            let mut task = String::new();
            if io::stdin().read_line(&mut task).is_err() {
                return Err(String::from("Could not read input."));
            }

            task = task.replace('\n', "");
            list.insert(0, List::Todo(task));
            println!("Your task has been added!\n");
        } else if input == String::from("2") {
            println!("\nYour To-Do List:");
            for (i, task) in list.iter().enumerate() {
                match task {
                    List::Todo(todo) => println!("{}: {todo}", i+1),
                    List::Complete(done) => println!("{}: {done} - COMPLETE", i+1),
                }
            }
            println!();
        } else if input == String::from("3") {
            if list.len() == 0 {
                eprintln!("\nThere are no items in your to-do list!\n");
                continue;
            }

            println!("Which task do you want to mark as complete? (Enter the corresponding number)");
            for (i, task) in list.iter().enumerate() {
                match task {
                    List::Todo(todo) => println!("{}: {todo}", i+1),
                    List::Complete(done) => println!("{}: {done} - COMPLETE", i+1),
                }
            }

            loop {
                let mut num = String::new();
                if io::stdin().read_line(&mut num).is_err() {
                    return Err(String::from("Could not read input."));
                }
    
                num = num.trim().to_string();
    
                if let Ok(n) = num.parse::<usize>() {
                    if n == 0 || n > list.len() {
                        eprintln!("Please enter a valid number.");
                        continue;
                    }
                    let complete_task = list.remove(n - 1);
                    match complete_task {
                        List::Complete(_) => eprintln!("Task already completed!"),
                        List::Todo(task) => {
                            list.push(List::Complete(task));
                        }
                    }
                    break;
                } else {
                    eprintln!("Please enter a valid number.");
                }
            }
        } else if input == String::from("4") {
            if list.len() != 0 {
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open("to-do_list.txt")
                    .unwrap();
                for task in list {
                    match task {
                        List::Todo(todo) => writeln!(file, "{todo}").unwrap(),
                        List::Complete(done) => writeln!(file, "COMPLETE- {done}").unwrap(),
                    }
                }
            }
            break;
        } else {
            eprintln!("\nYou didn't enter a valid option, try again\n");
        }
    }

    Ok(())
}
