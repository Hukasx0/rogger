use actix_web::{get, post, App, web, HttpResponse, HttpServer};
use std::fs;
use serde::Deserialize;
mod posts;
use posts::{Posts, Post};

static YOUR_NAME: &str = "Hubert";
static YOUR_DESCRIPTION: &str = "test description, which is a placeholder";

#[get("/")]
async fn index() -> HttpResponse {
    let index_file = fs::read_to_string("web/index.html")
    .expect("Problem with reading index.html file");
    HttpResponse::Ok().body(index_file.replace("{{author_name}}",YOUR_NAME).replace("{{author_description}}",YOUR_DESCRIPTION))
}

#[get("/posts")]
async fn list_posts(posts: web::Data<Posts>) -> HttpResponse {
    let posts_file = fs::read_to_string("web/posts.html")
    .expect("Problem with reading posts.html file");
    let mut post_list = String::new();
    for post in posts.get_list().iter().rev() {
    	post_list.push_str(&format!(r#"
	<div class="post">
	   <a href="/post/{}"<h2 class="title">{}</h2></a>
	   <p class="description">{}</p>
	   <span class="date">{}</span>
	</div>
	"#,post.id ,post.name, post.text, post.date));
    }
    HttpResponse::Ok().body(posts_file.replace("{{author_name}}",YOUR_NAME).replace("{{post_list}}",&post_list))
}

#[get("/post/{pid}")]
async fn get_post(pid: actix_web::web::Path<usize>, posts: web::Data<Posts>) -> HttpResponse {
    let post_file = fs::read_to_string("web/post.html")
    .expect("Problem with reading post.html file");
    if let Some(post) = posts.get_post(pid.into_inner()) {
       HttpResponse::Ok().body(post_file.replace("{{post_name}}",&post.name).replace("{{post_text}}",&post.html_text).replace("{{post_date}}",&post.date).replace("{{author_name}}",YOUR_NAME))
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
      html_text: form.text.to_string(),
      date: form.date.to_string(),
   });
   HttpResponse::Ok().body(format!("Added {} to database",&form.name))
}

#[derive(Deserialize)]
struct ModPost {
   id: usize,
   name: String,
   text: String,
   date: String,
}

#[post("/api/editPost")]
async fn modify_post(form: web::Form<ModPost>, posts: web::Data<Posts>) -> HttpResponse {
    posts.edit_post(Post {
       id: form.id,
       name: form.name.to_string(),
       text: form.text.to_string(),
       html_text: form.text.to_string(),
       date: form.date.to_string(),
    });
    HttpResponse::Ok().body(format!("Modified {} post in database",form.id))
}

#[derive(Deserialize)]
struct RmPost {
   id: usize,
}

#[post("/api/removePost")]
async fn remove_post(form: web::Form<RmPost>, posts: web::Data<Posts>) -> HttpResponse {
    posts.rm_post(form.id);
    HttpResponse::Ok().body(format!("Post with id {} has been removed",form.id))
}

#[get("/css/main.css")]
async fn css_main() -> HttpResponse {
    let css_file = fs::read_to_string("web/css/main.css")
    .expect("Problem with reading main.css file");
    HttpResponse::Ok().body(css_file)
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
