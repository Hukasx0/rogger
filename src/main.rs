use actix_web::{get, post, App, HttpResponse, HttpServer};

static YOUR_NAME: &str = "Hubert";

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body(format!(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta http-equiv="X-UA-Compatible" content="IE=edge">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>{}'s blog</title>
        </head>
        <body>
            Hello world!
        </body>
        </html>
        "#,YOUR_NAME))
}

#[get("/posts")]
async fn list_posts() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/post/{pid}")]
async fn get_post(pid: actix_web::web::Path<u32>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello, this is post nr {}", pid))
}

#[post("/api/addPost")]
async fn add_post() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/api/modifyPost")]
async fn modify_post() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/api/removePost")]
async fn remove_post() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(list_posts)
            .service(get_post)
            .service(add_post)
            .service(modify_post)
            .service(remove_post)
    })
    .bind(("0.0.0.0", 1337))?
    .run()
    .await
}
