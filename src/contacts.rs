use serde::Deserialize;
use std::{fs, io};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Contact {
    id: String,
    first: String,
    last: String,
    phone: String,
    email: String,
}

impl Contact {
    fn new(id: &str, first: &str, last: &str, phone: &str, email: &str) -> Contact {
        Contact {
            id: String::from(id),
            first: String::from(first),
            last: String::from(last),
            phone: String::from(phone),
            email: String::from(email),
        }
    }
}

#[derive(Debug)]
pub struct Contacts {
    contacts: Vec<Contact>,
}

impl Contacts {
    pub fn load_db() -> Result<String, io::ErrorKind> {
        let contacts = fs::read_to_string("contacts.json").expect("Failed to run");
        Ok(contacts)
    }
}
