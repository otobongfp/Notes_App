use std::io::{self, Write};
use chrono::prelude::*;
//use serde_json::to_string;


#[derive(Debug)]

//TO-DO
//Allow Created Notes to automatically record date


struct Note{
    id: u16,
    date: DateTime<Local>,
    title : String,
    author : String,
    body : String,
}

// enum Notes{
//     Id(Note),
// }

//To hold the structs on notes being pushed in.
static  mut ALL_NOTES : Vec<Note> = Vec::new();

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
    
    loop{

        //Choose a Command based on the above options
        let mut choice = String::new();

        print!("type here ---> ");
        //Allows Us to Receive inputs on the same line
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut choice).expect("Something is Wrong");

        let choice = choice.trim().parse().unwrap();
        match choice {
            1 => {
                println!("Creating New Note...");
                
                // let from_ymd_opt = NaiveDate::from_ymd_opt;
                // assert!(from_ymd_opt(2015, 3, 14).is_some());
        
                print!("TITLE:   ");
                let mut title = String::new();
                let _ = io::stdout().flush();
                io::stdin().read_line(&mut title).expect("Check the title");
        
                print!("Make Note:   ");
                let mut body = String::new();
                let _ = io::stdout().flush();
                io::stdin().read_line(&mut body).expect("Check the title");

                //I cloned the name as the author in the function. This way we avoid
                //Any possible memory pointer issues
                create_note(now, title, name.clone(), body);
            },
            2 => {
                //Reading the note
                read_note();
            },
            _ => {
                println!("Something is wrong");
            },
    
    
        };
    }
    
    //CREATE NOTE - working
    
    //LIST NOTES

    //READ NOTE - OUPUT TO TERMINAL - TUI

    //UPDATE NOTE

    //DELETE NOTE


    
}


pub fn create_note(date: DateTime<Local>, title: String, author: String, body: String) {

    //You can use this but since the function names is similar
    //to the names of the field on the struct we use single names
    // let new_note = Note{
    //     title: title,
    //     author: author,
    //     body: body,
    // };
    
    unsafe{
        let this_id = ALL_NOTES.len() as u16 + 1;

        let new_note = Note{
            id:this_id,
            date,
            title,
            author,
            body,
        };
    
        ALL_NOTES.push(new_note);
    }

}

pub fn read_note(){
    //I used unsafe here to easily access the values in All notes
    //There are options to avoid using unsafe but for simplicity
    //I made everything as direct as possible, lol.
    
    println!("\n\n_________________________________________________CURRENT NOTES_________________________________________________\n");
    unsafe{
        for i in &ALL_NOTES{
            //println!("{:?}", i);
            let new_id = &i.id;
            let new_date = &i.date;
            let new_title = &i.title;
            let new_name = &i.author;
            let new_body = &i.body;
            println!("ID[{new_id}] \nTITLE: {new_title}\nauthor: {new_name} \ncreated on: {new_date} \n\n\tnote: {new_body}");
        }
    }
}