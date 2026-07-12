// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then, let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io::{self, Write};

fn add_employees(depts: &mut HashMap<String, Vec<String>>) {
    let mut input = String::new();
    print!("Enter employee info (Add <employee> to <department>): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline.");

    let input = input.trim();
    if let Some(rest) = input.strip_prefix("Add ") {
        if let Some((employee, department)) = rest.split_once(" to ") {
            depts
                .entry(department.trim().to_string())
                .or_default()
                .push(employee.trim().to_string());
            println!("\n\nSuccessfully added {employee} to {department}.");
        } else {
            println!("Expected format: Add <employee> to <department>")
        }
    } else {
        println!("Expected format: Add <employee> to <department>")
    }
}

fn list_employees_in_department(depts: &HashMap<String, Vec<String>>) {
    let mut dept_name = String::new();
    print!("Enter Department name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut dept_name)
        .expect("Failed to readline.");
    let dept_name = dept_name.trim();

    match depts.get(dept_name) {
        Some(employees) => {
            let mut new_employees = employees.clone();
            new_employees.sort_by_key(|name| name.to_lowercase());
            println!("\nEmployees in {dept_name}: ");
            for employee in new_employees {
                println!("- {employee}");
            }
        }
        None => {
            println!("Department '{dept_name}' not found.");
        }
    }
}

fn list_employees_in_all_department(depts: &HashMap<String, Vec<String>>) {
    let mut dept_names: Vec<&String> = depts.keys().collect();
    dept_names.sort_by_key(|name| name.to_lowercase());

    for dept_name in dept_names {
        match depts.get(dept_name) {
            Some(employees) => {
                let mut new_employees = employees.clone();
                new_employees.sort_by_key(|name| name.to_lowercase());
                println!("\nEmployees in {dept_name}: ");
                for employee in new_employees {
                    println!("- {employee}");
                }
            }
            None => {
                println!("Department '{dept_name}' not found.");
            }
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn pause() {
    println!();
    println!("Press Enter to return to the main menu...");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn main_menu() -> u8 {
    println!("***************************************");
    println!(" Welcome to Employee Management System");
    println!("***************************************");
    println!();
    println!("1. Add Employee to a Department");
    println!("2. List Employees in a Department");
    println!("3. List all Employees by Department");
    println!("4. Exit");

    loop {
        let mut choice = String::new();

        print!("\nEnter your choice: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim().parse::<u8>() {
            Ok(num) if (1..=4).contains(&num) => return num,
            _ => println!("Please enter a number between 1 and 4."),
        }
    }
}

fn main() {
    let mut depts: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        clear_screen();
        match main_menu() {
            1 => {
                clear_screen();
                add_employees(&mut depts);
                pause();
            }
            2 => {
                clear_screen();
                list_employees_in_department(&depts);
                pause();
            }
            3 => {
                clear_screen();
                list_employees_in_all_department(&depts);
                pause();
            }
            4 => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice.");
                pause();
            }
        }
    }
}
