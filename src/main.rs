use std::io::{self, Write};
use chrono::prelude::*;
use serde_json::to_string;


#[derive(Debug)]

//TO-DO
//Allow Created Notes to automatically record date


struct Note{
    id: u16,
    //date: String,
    title : String,
    author : String,
    body : String,
}

// enum Notes{
//     Id(Note),
// }



fn main() {
//[L] List all notes
//[C] Create new note
//[D] Delete Note
//[U] Update an old note
//[R] Read a note via a TUI.

    println!("+----------------------------+\n|  WELCOME TO THE NOTES APP  |\n+----------------------------+");
    
    let now: DateTime<Local> = Local::now();
    println!("Current date and time: {}\n", now);

    //The Name would be assigned as an author of a note, at any particular time
    let mut name = String::new();
    print!("Name of User in this current session ---> ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut name).expect("Something happened while creating name");

    println!("
    \t[C] Create new note \t[U] Update an old note
    \t[R] Read a note. \t[L] List all notes
    \t[D] Delete Note
    ");
    
    //Choose a Command based on the above options
    let mut choice = String::new();

    print!("type here ---> ");
    //Allows Us to Receive inputs on the same line
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut choice).expect("Something is Wrong");
    let option = choice.to_lowercase().to_string();

    match option {
        "name" => {
            println!("Creating New Note...");
            
            let from_ymd_opt = NaiveDate::from_ymd_opt;
            assert!(from_ymd_opt(2015, 3, 14).is_some());
    
            print!("TITLE:   ");
            let mut t = String::new();
            let _ = io::stdout().flush();
            io::stdin().read_line(&mut t).expect("Check the title");
    
            print!("Make Note:   ");
            let mut b = String::new();
            let _ = io::stdout().flush();
            io::stdin().read_line(&mut b).expect("Check the title");
            
            create_note(t, name, b);
        },

    };
    
    //CREATE NOTE
    
    //LIST NOTES

    //READ NOTE - OUPUT TO TERMINAL - TUI

    //UPDATE NOTE

    //DELETE NOTE

    
}

//CREATE NOTE
fn create_note(title: String, name : String, body: String){

    let new_note = Note{
        id: 1,
        title : title,
        author : name,
        body : body,
    };
}