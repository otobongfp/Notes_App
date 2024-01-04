use std::io::{self, Write};
use chrono::prelude::*;
//use serde_json::to_string;


#[derive(Debug)]


struct Note{
    id: u16,
    date: DateTime<Local>,
    title : String,
    author : String,
    body : String,
}

//To hold the structs on notes being pushed in.
static  mut ALL_NOTES : Vec<Note> = Vec::new();

fn main() {
//[L] List all notes
//[C] Create new note
//[D] Delete Note
//[U] Update an old note
//[R] Read a note via a TUI.

    

    println!("\n+----------------------------+\n|  WELCOME TO THE NOTES APP  |\n+----------------------------+");
    
    let now: DateTime<Local> = Local::now();
    println!("Current date and time: {}\n", now);

    //The Name would be assigned as an author of a note, at any particular time
    let mut name = String::new();
    print!("Name of User in this current session ---> ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut name).expect("Something happened while creating name");

    println!("\n\tChoose one of the letters to execute the action");
    println!("
    \t[C] Create new note \t[R] Read a specific note.
    \t[U] Update an old note \t[D] Delete a Note
    \t[L] List all notes
    ");
    
    loop{
        //Choose a Command based on the above options
        let mut choice = String::new();

        print!("\nChoose an Action [C]reate, [R]ead, [U]pdate, [D]elete, [L]ist -----> ");
        //Allows Us to Receive inputs on the same line
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut choice).expect("Something is Wrong");

        let choice = choice.trim().to_lowercase();

        match choice.as_str() {
            "c" => {
                println!("Creating New Note...");
        
                print!("TITLE:   ");
                let mut title = String::new();
                let _ = io::stdout().flush();
                io::stdin().read_line(&mut title).expect("Check the title");
        
                print!("Make Note:   ");
                let mut body = String::new();
                let _ = io::stdout().flush();
                io::stdin().read_line(&mut body).expect("Check the title");

                //I called the read notes function
                //I cloned the name as the author in the function. This way we avoid
                //Any possible memory pointer issues
                create_note(now, title, name.clone(), body);
            },
            "r" => {
                //Reading a specific note by ID
                println!("Input the ID of the note you intend to READ: ");
                let mut num = String::new();
                io::stdin().read_line(&mut num).expect("Could not get the ID");
                
                let mut id: usize = num.trim().parse().expect("Something is wrong");
                id = id - 1;

                unsafe{

                    let the_note = &mut ALL_NOTES[id];
                    let id = &the_note.id;
                    let date = &the_note.date;
                    let title = &the_note.title;
                    let name = &the_note.author;
                    let body = &the_note.body;
                    println!("\n\tID[{id}] \n\tTITLE: {title} \n\tcreated on: {date} \n\tcreated by:{name} \n\n\tNOTE: {body}");
                }
            },
            "u" => {
                //Update a note
                update_note();
            },
            "l" => {
                //Reading the note
                //calls the list note function to list all the notes
                list_notes();
            },
            "d" => {
                //To delete a specific note

                println!("Input the ID of the note you intend to DELETE: ");
                let mut num = String::new();
                io::stdin().read_line(&mut num).expect("Could not get the ID");
                
                let mut id: usize = num.trim().parse().expect("Something is wrong");
                id = id - 1;

                unsafe{
                    if id < ALL_NOTES.len(){
                        //Remove returns the deleted item, so I used print to output it. I could still parse it
                        //But using {:?} allows the println! macro to handle output for a struct
                        //{:?} works because I initially declared #[derive(Debug)]
                        let deleted = ALL_NOTES.remove(id);
                        println!("Deletion, successful!! {:?}", deleted);
                    }else{
                        println!("Failed to Delete. Check the ID");
                    }
                }
            },
            _ => {
                println!("Something is wrong");
            },
    
    
        };
    }
    
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

pub fn list_notes(){
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

pub fn update_note(){
    println!("You are about to edit a note");
    println!("Input the note's ID: ");

    unsafe{
        //The note we want to change
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("The Input was inaccurate");
        let mut id: usize = num.trim().parse().expect("The ID is invalid");

        //Subtracted one below to ensure that we pick the correct position of struct inside the vector
        id = id - 1;

        if id <= ALL_NOTES.len(){
            println!("New Note: ");
            let mut body = String::new();
            io::stdin().read_line(&mut body).expect("Body is invalid");
            body = body.trim().parse().expect("Could not convert");
                        
            let update = &mut ALL_NOTES[id];
            update.body = body.trim().to_string();

            let new_id = &update.id;
            let new_date = &update.date;
            let new_title = &update.title;
            let new_name = &update.author;
            let new_body = &update.body;
            println!("\n\t\t+----------------------------+\n\t\t|  PREVIEW OF UPDATE  |\n\t\t+----------------------------+");
            println!("\nID[{new_id}] \nTITLE: {new_title}\nauthor: {new_name} \ncreated on: {new_date} \n\n\tnote: {new_body}");
                        
        }else{
            println!("Something is wrong with the edit process")
        }
    }


}