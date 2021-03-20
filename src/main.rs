use std::collections::HashMap;
use std::io::Error;
use std::fs::{read_to_string, File};

const DB_PATH: &str = "kvstore.db";

fn main() -> Result<(), Error> {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next();

    let mut database = Database::new()?;

    if value.is_none() {
        match database.get(&key) {
            None => {}
            Some(value) => { println!("{}", value) }
        }
    } else {
        database.insert(key.to_uppercase(), value.unwrap());
    }

    Ok(())
}

struct Database {
    map: HashMap<String, String>,
    flush: bool
}

impl Database {
    fn new() -> Result<Database, Error> {
        let db_exists = std::path::Path::new(DB_PATH).exists();

        if !db_exists {
            let _ = File::create(DB_PATH)?;
            return Ok(Database { map: Default::default(), flush: false });
        }

        let contents = read_to_string(DB_PATH)?;
        let mut map = HashMap::new();

        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().unwrap();
            let value = chunks.next().unwrap();
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map, flush: false })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
        self.flush = true;
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.map.get(&key.to_uppercase())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut contents = String::new();

        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }

        std::fs::write(DB_PATH, contents).and_then(|_| { self.flush = true; Ok(()) } )
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if self.flush { let _ = self.flush(); }
    }
}
