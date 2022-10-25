use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get_credentials() {
    let path = Path::new("src/credentials.txt");
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why) => panic!("Couldn't open file {}, because : {}", display,why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

}