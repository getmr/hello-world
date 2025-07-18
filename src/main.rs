use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
mod libs; // Import the libs module to access the add function
mod utils; // Import the utils module for additional functionality
mod response; // Import the response module for handling different types of responses   
use libs::errors::MyError;
use sea_orm::prelude::DatabaseConnection;
use sea_orm::Database;


#[derive(Debug, Clone)]
struct AppState {
    // You can add fields here if needed
    app_name: String,
    conn: DatabaseConnection,

}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello from {}!", app_name))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/add/{a}/{b}")]
async fn add_numbers(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let result = utils::help::add_numbers(a, b); // Using the add function from utils module
    HttpResponse::Ok().body(format!("The sum of {} and {} is {}", a, b, result))
}

#[get("/json")]
async fn json_response() -> impl Responder {
    let obj = response::json::MyObj { name: "John Doe".to_owned() };
    obj
}

#[get("/error")]
async fn res_error() -> Result<&'static str, MyError> {
    Err(MyError::BadClientData)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "mysql://root:root@localhost:3306/hello_world".to_owned();
    let conn = Database::connect(&db_url).await.unwrap();
    let state = AppState { app_name: "Actix Web Example".to_string(), conn };

    // Use move to capture conn by value
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(
                web::scope("/app")
                    // ...so this handles requests for `GET /app/index.html`
                    .service(hello)
                    .service(web::resource("/manual").route(web::get().to(manual_hello)))
            )
            .service(add_numbers)
            .service(json_response)
            .service(res_error)
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
