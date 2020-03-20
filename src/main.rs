use std::env;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use text_io::try_read;

fn main() {
    let mut editor = load_editor();
    if editor != 1 && editor != 2 {
        editor = select_editor();
    }

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Use ironjournal -h for help");
        return;
    }

    for i in 1..args.len() {
        match args[i].as_ref() {
            "-n" => {
                if i == args.len() - 1 {
                    println!("Please specify a title and/or category");
                } else {
                    if i == args.len() - 2 {
                        new_note(&args[i + 1], &String::from("misc"), editor);
                    } else if i == args.len() - 3 {
                        new_note(&args[i + 1], &args[i + 2], editor);
                    } else {
                        println!("Invalid arguments");
                    }
                }
                return;
            }
            "-h" | "-help" => {
                println!("Help Menu");
                return;
            }
            "-c" => {
                if i == args.len() - 1 {
                    println!("Not enough arguments");
                } else if i == args.len() - 2 {
                    list_notes_in_category(&args[i + 1], editor);
                } else {
                    println!("Invalid arguments");
                }
                return;
            }
            _ => {
                println!("Invalid arguments used");
                return;
            }
        }
    }
}

fn list_notes_in_category(category: &String, editor: i16) {
    let path = Path::new(category);
    if path.is_dir() {
        for (i, entry) in fs::read_dir(path).unwrap().enumerate() {
            println!("{}: {}", i + 1, entry.unwrap().path().display());
        }
        print!("Select a note to open: ");
        io::stdout().flush().unwrap();
        let input: Result<u32, _> = try_read!();
        match input {
            Ok(file) => {
                for (i, entry) in fs::read_dir(path).unwrap().enumerate() {
                    if i == file as usize {
                        open_note(
                            &entry
                                .unwrap()
                                .path()
                                .into_os_string()
                                .into_string()
                                .unwrap(),
                            editor,
                        );
                    }
                }
            }
            Err(_) => {
                println!("Invalid input");
            }
        }
    } else {
        println!("{} is not an existing category", category);
    }
}

fn new_note(title: &String, category: &String, editor: i16) {
    println!("Creating note titled {} in category {}", title, category);
    let path = format!("{}/{}.txt", category, title);
    if !Path::new(category).exists() {
        fs::create_dir(category).expect("Unable to create directory");
    }
    File::create(Path::new(&path[..])).expect("Unable to create note");

    open_note(&path, editor);
}

fn open_note(path: &String, editor: i16) {
    match editor {
        1 => {
            Command::new("vim")
                .arg(path)
                .status()
                .expect("Failed to run");
        }
        2 => {
            Command::new("nano")
                .arg(path)
                .status()
                .expect("Failed to run");
        }
        _ => println!("Invalid editor, try setting editor through ironjournal -e"),
    }
}

fn load_editor() -> i16 {
    let path = Path::new("editor.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(_) => {
            return select_editor();
        }
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            let editor: i16 = s.parse().unwrap();
            return editor;
        }
    }
}

fn select_editor() -> i16 {
    let mut input: i16;
    println!("Vim(1) or Nano(2)");
    print!("Choose an editor by entering a number: ");
    io::stdout().flush().unwrap();
    loop {
        let e: Result<i16, _> = try_read!();
        match e {
            Ok(v) => {
                input = v;
                if input != 1 && input != 2 {
                    println!("Invalid input: {}", v);
                    print!("Choose an editor by entering a number: ");
                    io::stdout().flush().unwrap();
                } else {
                    break;
                }
            }
            Err(r) => {
                println!("Invalid input: {}", r);
                print!("Choose an editor by entering a number: ");
                io::stdout().flush().unwrap();
            }
        }
    }

    fs::write("editor.txt", format!("{}", input)).expect("Unable to write to editor file");

    return input;
}

/*
Expected features -
- Support vim and nano text editors
- Sorts journal notes by Month, Day of the Week, Year, Day of the Month, Title, and Category
- Search up certain notes
- Create notes given title and category (Must find date to sort the note)
- Query location or open notes based on title, cateory, day of the week, day of the month, month, and/or year

Possible commands
ironjournal -n title category
ironjournal -n title (defaults to no category which will be named Misc)
ironjournal -h or ironjournal -help
ironjournal -t title (searches and numbers notes with title and prompts to open or exit)
ironjournal -c category (searches and numbers  lists notes with category and prompts to open or exit)
ironjournal -t title -c category (searcehs and numbers lists notes with title and category and prompts to open or exit)
ironjournal -e (Selects editor)

File structure
Category
Title
Note

Metadata file that holds preferred editor

Metadata file that holds data about every note
-Title (Key)

Value is a struct Note
-Location
-Category
-Date
*/
