use uuid::Uuid;

pub struct Contact {
    pub id: String,
    pub first: String,
    pub last: String,
    pub phone: String,
    pub email: String,
}

impl Contact {
    pub fn new(first: String, last: String, phone: String, email: String) -> Contact {
        Contact {
            id: Uuid::new_v4().to_string(),
            first,
            last,
            phone,
            email,
        }
    }
}
