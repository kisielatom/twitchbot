use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Credentials {
    pub channel: String,
    pub botname: String,
    pub oauth: String,
}

pub fn get_credentials() -> Credentials {
    let path = Path::new("src/credentials.json");
    let credentials = read_credentials_from_json(path).unwrap();
    println!("{:#?}", credentials);
    return credentials
    
}

fn read_credentials_from_json<P: AsRef<Path>>(path: P) -> Result<Credentials, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let credentials = serde_json::from_reader(reader)?;

    Ok(credentials)
}
