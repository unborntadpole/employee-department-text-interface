use std::collections::HashMap;
use std::io;
use std::collections::hash_map::Entry;

fn main() {
    let mut input = String::new();
    let mut emp_dep: HashMap<String, String> = HashMap::new();
    loop {
        input.clear();
        println!("Enter command: (start with 'add' to add employee to department, 'get' to get employee details, 'exit' to stop)");
        io::stdin().read_line(&mut input).expect("Incorrect input");

        let words:Vec<&str> = input.split_whitespace().collect();

        let command: String = words[0].to_string().to_lowercase();

        match command.as_str() {
            "add" => {
                if words.len() < 3 {
                    println!("Invalid command. 'add' requires 2 arguments: name, department. you can also use 'to' in between name and department'");
                    continue;
                }
                add_employee(&mut emp_dep, words[1], words[2..].to_vec());
            }
            "get" => {
                if words.len() < 2 {
                    println!("Invalid command. Include more words in your command");
                    continue;
                }                
                print_details(&mut emp_dep, words[1])
            }
            "exit" => break,
            _ => println!("Invalid command try again"),
        }

    }
}


fn add_employee( map: &mut HashMap<String,String>, name: &str, department: Vec<&str>) {
    println!("Adding employee..");
    let mut value = String::from(department[..].join(" ").to_string());
    if department[0].to_lowercase() == "to" {
        if department.len() < 2 {
            println!("Include department name.");
            return;
        }
        value = department[1..].join(" ").to_string();
    } 
    match map.entry(name.to_lowercase().to_string()) {
        Entry::Vacant(entry) => {
            entry.insert(value);
        }
        Entry::Occupied(mut entry) => {
            *entry.get_mut() = value;
        }
    }
}

fn print_details(map: &mut HashMap<String,String>, name: &str) {
    match map.entry(name.to_lowercase().to_string()) {
        Entry::Vacant(_) => {
            if name.to_lowercase() == "all" {
                println!("{:#?}",map);
            } else {
                println!("Employee {} not found", name);
            }
        },
        Entry::Occupied(entry) => {
            println!("Employee {} belongs to department: {}", name, entry.get());
        }
    }
}