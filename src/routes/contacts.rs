use actix_web::{web, HttpRequest, Responder};

pub async fn contacts(req: HttpRequest) -> impl Responder {
    let path = req.path();
    format!("Rust engineer - {:?}", path)
}
