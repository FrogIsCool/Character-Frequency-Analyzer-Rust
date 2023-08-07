use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::env;
use::collections::HashMap;

fn main() {
    let mut args: Vec<_> = env::args().collect();
    let mut input = String::new();
    
    let arg_count = args.len();
    if arg_count == 1 {
        print!("Enter the path to file: ");
        io::stdin().read_line(&mut input).unwrap();

    } else if arg_count == 2 {
        input = args.remove(1);
    } else {
        panic!("Too many arguments.\nTakes either a path to a file or no arguments.\n");
    }
    
    let path = Path::new(&input);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut text = String::new();
    if let Err(why) = file.read_to_string(&mut text) {
        panic!("Failed to read {}: {}", display, why);
    }
    
    let mut charCounts = HashMap::new();
    for c in test.chars() {
        
}
