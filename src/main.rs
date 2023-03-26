use actix_web::{get, post, App, HttpResponse, HttpServer};
use std::fs;

static YOUR_NAME: &str = "Hubert";
static YOUR_DESCRIPTION: &str = "test description, which is a placeholder";

#[get("/")]
async fn index() -> HttpResponse {
    let indexFile = fs::read_to_string("web/index.html")
    .expect("Problem with reading index.html file");
    HttpResponse::Ok().body(indexFile.replace("{{author_name}}",YOUR_NAME).replace("{{author_description}}",YOUR_DESCRIPTION))
}

#[get("/posts")]
async fn list_posts() -> HttpResponse {
    let postsFile = fs::read_to_string("web/posts.html")
    .expect("Problem with reading posts.html file");
    HttpResponse::Ok().body(postsFile.replace("{{author_name}}",YOUR_NAME))
}

#[get("/post/{pid}")]
async fn get_post(pid: actix_web::web::Path<u32>) -> HttpResponse {
    let postFile = fs::read_to_string("web/post.html")
    .expect("Problem with reading post.html file");
    HttpResponse::Ok().body(postFile.replace("{{postid}}",&pid.to_string()).replace("{{author_name}}",YOUR_NAME))
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

#[get("/css/main.css")]
async fn css_main() -> HttpResponse {
    let cssFile = fs::read_to_string("web/css/main.css")
    .expect("Problem with reading main.css file");
    HttpResponse::Ok().body(cssFile)
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
            .service(css_main)
    })
    .bind(("0.0.0.0", 1337))?
    .run()
    .await
}
