use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser.pest"]
struct SystemParser;

#[derive(Debug, PartialEq)]
enum FolderMode {
    Append,
    Speak,
}

#[derive(Debug)]
struct Folder<'a> {
    name: String,
    default: bool,
    immediate: bool,
    rows: usize,
    cols: usize,
    buttons: Vec<&'a str>,
}

#[derive(Debug)]
struct System<'a> {
    name: String,
    description: String,
    default: String,
    rows: usize,
    cols: usize,
    folders: Vec<Folder<'a>>,
}

fn parse(system: &str) -> Result<System, Box<dyn std::error::Error>> {
    let pairs = SystemParser::parse(Rule::program, system)?;

    let mut metadata: HashMap<&str, &str> = HashMap::new();
    let mut folders: Vec<Folder> = Vec::new();

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        match pair.as_rule() {
            Rule::assignment => {
                let mut inner = pair.into_inner();
                let key = inner.next().ok_or("Got Rule::assignment with no inner items?")?.as_str();
                let value = inner.next().ok_or("Got Rule::assignment with only one inner item?")?.as_str();

                metadata.insert(key, value);
            },
            Rule::folder => {
                let mut inner = pair.into_inner();
                let name = inner.next().ok_or("Got Rule::folder with no name?")?.as_str();
                let immediate = match inner.next().ok_or("Got Rule::folder with no mode?")?.as_str() {
                    "append" => false,
                    "immediate" => true,
                    _ => unreachable!("The only folder modes are \"append\" or \"immediate\". This error should be impossible."),
                };

                let buttons: Vec<&str> = inner.map(|x| x.as_str()).collect();

                // The grammar requires all assignments to be before all folders.
                // This allows us to specify data at the system level, but
                // store it internally in each +Folder+ for simplicity.
                let default = metadata.get("default").ok_or("Expected 'default' to be specified.")? == &name;
                let rows = metadata.get("rows").ok_or("Expected 'rows' to be specified.")?.parse()?;
                let cols = metadata.get("cols").ok_or("Expected 'cols' to be specified.")?.parse()?;

                folders.push(Folder { name: name.to_string(), default, immediate, rows, cols, buttons });
            },
            _ => unreachable!("The only language constructs are ASSIGNMENTS and FOLDERS. This error should be impossible."),
        }
    }

    let name = metadata.get("name").ok_or("Expected 'name' to be specified.")?.to_string();
    let description = metadata.get("description").ok_or("Expected 'description' to be specified.")?.to_string();
    let default = metadata.get("default").ok_or("Expected 'default' to be specified.")?.to_string();
    let rows: usize = metadata.get("rows").ok_or("Expected 'rows' to be specified.")?.parse()?;
    let cols: usize = metadata.get("cols").ok_or("Expected 'cols' to be specified.")?.parse()?;

    Ok(System { name, description, default, rows, cols, folders })
}

fn main() {
    let system = r#"
name = "Example system."
description = "This is an example system."
default = "Example Folder"

rows = 2
cols = 3

folder "foo" (append) "asdf" "fdsa" "beep" "boop" "meep" "moop";

folder "Example Folder" (append)
        "1"     "2"     "3"
        "4"     "5"     "6"
;
    "#;

    let system = parse(system).unwrap();
    println!("{:?}", system);
}


#[test]
fn test_parser() {
    let system = r#"
        name = "Example System"
        description = "This is an example system."
        default = "Example Folder"

        rows = 2
        cols = 3

        folder "foo" (immediate) "asdf" "fdsa" "beep" "boop" "meep" "moop";

        folder "Example Folder" (append)
                "0"     "1"     "2"
                "3"     "4"     "5"
        ;
    "#;

    let system = parse(system).unwrap();

    assert_eq!("Example System", system.name);
    assert_eq!("This is an example system.", system.description);

    assert_eq!(2, system.folders.len());

    let folder = &system.folders[0];
    assert_eq!(false, folder.default);
    assert_eq!(true, folder.immediate);
    assert_eq!(2, folder.rows);
    assert_eq!(3, folder.cols);
    assert_eq!(6, folder.buttons.len());
    assert_eq!("asdf", folder.buttons[0]);
    assert_eq!("fdsa", folder.buttons[1]);
    assert_eq!("beep", folder.buttons[2]);
    assert_eq!("boop", folder.buttons[3]);
    assert_eq!("meep", folder.buttons[4]);
    assert_eq!("moop", folder.buttons[5]);

    let folder = &system.folders[1];
    assert_eq!(true, folder.default);
    assert_eq!(false, folder.immediate);
    assert_eq!(2, folder.rows);
    assert_eq!(3, folder.cols);
    assert_eq!(6, folder.buttons.len());
    assert_eq!("0", folder.buttons[0]);
    assert_eq!("1", folder.buttons[1]);
    assert_eq!("2", folder.buttons[2]);
    assert_eq!("3", folder.buttons[3]);
    assert_eq!("4", folder.buttons[4]);
    assert_eq!("5", folder.buttons[5]);
}

#[test]
fn test_incomplete_metadata() {
    let broken_system = r#"
        name = "foo"

        folder "foo" (append)
            "beep boop"
        ;
    "#;

    let broken_system = parse(broken_system);

    assert!(broken_system.is_err());
}

#[test]
fn test_invalid_folder_mode() {
    let broken_system = r#"
        name = "foo"

        folder "foo" (definitely-not-valid)
            "beep" "boop"
        ;
    "#;

    let broken_system = parse(broken_system);

    // This should be a pest::error::ErrorVariant::ParserError,
    // but I don't know how to test for that.
    assert!(broken_system.is_err());
}
