use mysql::*;
use mysql::prelude::*;
use std::io::{self, Write};

fn get_text_input(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn get_integer_input(prompt: &str) -> Option<i32> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        match trimmed.parse::<i32>() {
            Ok(val) => Some(val),
            Err(_) => {
                println!("Invalid integer. NULL will be stored.");
                None
            }
        }
    }
}

fn display_crushes(conn: &mut PooledConn) -> Vec<Row> {
    println!("\nFetching list of crushes...");
    let query = "SELECT id, first_name, last_name FROM crushes";
    let result: Vec<Row> = conn.query(query).unwrap();

    if result.is_empty() {
        println!("No crushes found in the database.");
        return result;
    }

    println!("\n--- List of Your Crushes ---");
    for row in &result {
        let id: i32 = row.get("id").unwrap();
        let first_name: Option<String> = row.get("first_name").unwrap();
        let last_name: Option<String> = row.get("last_name").unwrap();
        println!("ID: {} - {} {}", id, first_name.unwrap_or("-".into()), last_name.unwrap_or("-".into()));
    }

    result
}

fn display_crush(conn: &mut PooledConn) {
    if let Some(id) = get_integer_input("Enter the ID of the crush to view: ") {
        let query = format!("SELECT * FROM crushes WHERE id = {}", id);
        let result: Option<Row> = conn.query_first(query).unwrap();

        match result {
            Some(row) => {
                println!("Crush details:");
                println!("First Name: {:?}", row.get::<Option<String>, _>("first_name").unwrap());
                println!("Last Name: {:?}", row.get::<Option<String>, _>("last_name").unwrap());
                println!("Gender: {:?}", row.get::<Option<String>, _>("gender").unwrap());
                println!("Age: {:?}", row.get::<Option<i32>, _>("age").unwrap());
                println!("Phone: {:?}", row.get::<Option<String>, _>("phone_number").unwrap());
                println!("Instagram: {:?}", row.get::<Option<String>, _>("instagram_id").unwrap());
                println!("Relationship Status: {:?}", row.get::<Option<String>, _>("relationship_status").unwrap());
                println!("Interaction Level: {:?}", row.get::<Option<i32>, _>("interaction_level").unwrap());
                println!("Feelings Level: {:?}", row.get::<Option<i32>, _>("feelings_level").unwrap());
                println!("Future Plan: {:?}", row.get::<Option<String>, _>("future_plan").unwrap());
                println!("Notes: {:?}", row.get::<Option<String>, _>("notes").unwrap());
            },
            None => println!("No crush found with that ID."),
        }
    }
}

fn add_new_crush(conn: &mut PooledConn) {
    println!("\n--- Add New Crush ---");

    let first_name = get_text_input("First name: ");
    let last_name = get_text_input("Last name: ");
    let gender = get_text_input("Gender (male/female): ");
    let age = get_integer_input("Age: ");
    let phone_number = get_text_input("Phone number: ");
    let instagram_id = get_text_input("Instagram ID: ");
    let relationship_status = get_text_input("Relationship status: ");
    let interaction_level = get_integer_input("Interaction level: ");
    let feelings_level = get_integer_input("Feelings level: ");
    let future_plan = get_text_input("Future plan: ");
    let notes = get_text_input("Notes: ");

    conn.exec_drop(
        r"INSERT INTO crushes 
        (first_name, last_name, gender, age, phone_number, instagram_id, relationship_status, interaction_level, feelings_level, future_plan, notes) 
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        (
            first_name, last_name, gender, age, phone_number, instagram_id,
            relationship_status, interaction_level, feelings_level, future_plan, notes
        )
    ).unwrap();

    println!("New record inserted successfully.");
}

fn edit_crush(conn: &mut PooledConn) {
    if let Some(id) = get_integer_input("Enter the ID of the crush to edit: ") {
        println!("Leave the field empty if you don't want to change it.");

        let first_name = get_text_input("First name: ");
        let last_name = get_text_input("Last name: ");
        let gender = get_text_input("Gender (male/female): ");
        let age = get_integer_input("Age: ");
        let phone_number = get_text_input("Phone number: ");
        let instagram_id = get_text_input("Instagram ID: ");
        let relationship_status = get_text_input("Relationship status: ");
        let interaction_level = get_integer_input("Interaction level: ");
        let feelings_level = get_integer_input("Feelings level: ");
        let future_plan = get_text_input("Future plan: ");
        let notes = get_text_input("Notes: ");

        conn.exec_drop(
            r"UPDATE crushes SET 
            first_name = COALESCE(?, first_name),
            last_name = COALESCE(?, last_name),
            gender = COALESCE(?, gender),
            age = COALESCE(?, age),
            phone_number = COALESCE(?, phone_number),
            instagram_id = COALESCE(?, instagram_id),
            relationship_status = COALESCE(?, relationship_status),
            interaction_level = COALESCE(?, interaction_level),
            feelings_level = COALESCE(?, feelings_level),
            future_plan = COALESCE(?, future_plan),
            notes = COALESCE(?, notes)
            WHERE id = ?",
            (
                first_name, last_name, gender, age, phone_number, instagram_id,
                relationship_status, interaction_level, feelings_level, future_plan, notes, id
            )
        ).unwrap();

        println!("Record updated successfully.");
    }
}

fn delete_crush(conn: &mut PooledConn) {
    if let Some(id) = get_integer_input("Enter the ID of the crush to delete: ") {
        conn.exec_drop(
            "DELETE FROM crushes WHERE id = ?",
            (id,)
        ).unwrap();
        println!("Record deleted successfully.");
    }
}

fn main() {
    let url = "mysql://alirezza:@localhost:3306/learningdb";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    loop {
        println!("\n--- Crush Manager Menu ---");
        println!("1. Add New Crush");
        println!("2. View All Crushes");
        println!("3. View Single Crush");
        println!("4. Edit Crush");
        println!("5. Delete Crush");
        println!("6. Exit");

        print!("Enter your choice (1-6): ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => add_new_crush(&mut conn),
            "2" => { display_crushes(&mut conn); },
            "3" => display_crush(&mut conn),
            "4" => edit_crush(&mut conn),
            "5" => delete_crush(&mut conn),
            "6" => {
                println!("Exiting application.");
                break;
            }
            _ => println!("Invalid choice. Please enter 1-6."),
        }
    }
}
