use serde::Serialize;
use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder,
};

#[derive(Serialize, Debug)]
pub struct MyObj {
    pub name: String,
}

// Responder
impl Responder for MyObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = match serde_json::to_string(&self) {
            Ok(b) => b,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
