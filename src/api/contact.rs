use actix_web::{get, Responder};
use askama::Template;

use crate::model::contact::Contact;

#[derive(Template)]
#[template(path = "contacts.html")]
struct ContactsTemplate {
    contacts: Vec<Contact>,
}

#[get("/contacts")]
pub async fn contacts() -> impl Responder {
    ContactsTemplate {
        contacts: vec![Contact {
            id: "1".to_string(),
            first: "Software".to_string(),
            last: "Engineer".to_string(),
            phone: "222".to_string(),
            email: "engineer@google.com".to_string(),
        }],
    }
}
