use actix_web::{get, post, App, web, HttpResponse, HttpServer};
use std::fs;
use serde::Deserialize;
mod posts;
use posts::{Posts, Post};

static YOUR_NAME: &str = "Hubert";
static YOUR_DESCRIPTION: &str = "test description, which is a placeholder";

#[get("/")]
async fn index() -> HttpResponse {
    let indexFile = fs::read_to_string("web/index.html")
    .expect("Problem with reading index.html file");
    HttpResponse::Ok().body(indexFile.replace("{{author_name}}",YOUR_NAME).replace("{{author_description}}",YOUR_DESCRIPTION))
}

#[get("/posts")]
async fn list_posts(posts: web::Data<Posts>) -> HttpResponse {
    let postsFile = fs::read_to_string("web/posts.html")
    .expect("Problem with reading posts.html file");
    let mut post_list = String::new();
    for post in posts.get_list().iter() {
    	post_list.push_str(&format!(r#"
	<div class="post">
	   <h2 class="title">{}</h2>
	   <p class="description">{}</p>
	   <span class="date">{}</span>
	</div>
	"#, post.name, post.text, post.date));
    }
    HttpResponse::Ok().body(postsFile.replace("{{author_name}}",YOUR_NAME).replace("{{post_list}}",&post_list))
}

#[get("/post/{pid}")]
async fn get_post(pid: actix_web::web::Path<usize>, posts: web::Data<Posts>) -> HttpResponse {
    let postFile = fs::read_to_string("web/post.html")
    .expect("Problem with reading post.html file");
    if let Some(post) = posts.get_post(pid.into_inner()) {
       HttpResponse::Ok().body(postFile.replace("{{post_name}}",&post.name).replace("{{post_text}}",&post.text).replace("{{post_date}}",&post.date).replace("{{author_name}}",YOUR_NAME))
    } else {
      HttpResponse::Ok().body("Post with this id does not exist")
    }
}

#[derive(Deserialize)]
struct AddPost {
   name: String,
   text: String,
   date: String,
}

#[post("/api/addPost")]
async fn add_post(form: web::Form<AddPost>, posts: web::Data<Posts>) -> HttpResponse {
   posts.push_post(Post {
      id: 0,
      name: form.name.to_string(),
      text: form.text.to_string(),
      date: form.date.to_string(),
   });
   HttpResponse::Ok().body(format!("Added {} to database",&form.name))
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
    let posts = web::Data::new(Posts::new());
    HttpServer::new(move || {
        App::new()
	    .app_data(posts.clone())
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
