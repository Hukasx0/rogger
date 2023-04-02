use actix_web::{get, post, App, web, HttpResponse, HttpServer};
use std::sync::{Arc, Mutex};
use serde::Deserialize;
use rusqlite::Connection;
mod posts;
use posts::Database;
mod users;
use users::User;

static YOUR_NAME: &str = "Hubert";
static YOUR_DESCRIPTION: &str = "test description, which is a placeholder";

#[get("/")]
async fn index() -> HttpResponse {
    let index_file = include_str!("../web/index.html").replace("{{author_name}}",YOUR_NAME).replace("{{author_description}}",YOUR_DESCRIPTION);
    HttpResponse::Ok().body(index_file)
}

#[get("/posts")]
async fn list_posts() -> HttpResponse {
    let con = Connection::open("rogger.db").unwrap();
    let posts_file = include_str!("../web/posts.html");
    let mut post_list = String::new();
    if let Ok(posts_vec) = Database::get_list(con ,0) {
    for post in posts_vec {
    	post_list.push_str(&format!(r#"
	<div class="post">
	   <a href="/post/{}"<h2 class="title"><b>{}</b></h2></a>
	   <p class="description">{}</p>
	   <span class="date">{}</span>
	</div>
	"#,post.id ,post.title, format!("{}...<br><a href=\"/post/{}\"><b>Read more</b></a>", post.content.chars().take(355).collect::<String>(), post.id), post.date));
    }
    HttpResponse::Ok().body(posts_file.replace("{{author_name}}",YOUR_NAME).replace("{{post_list}}",&post_list))
    } else {
     HttpResponse::Ok().body("Cannot find posts with this id")
    }
}

#[get("/post/{pid}")]
async fn get_post(pid: actix_web::web::Path<usize>) -> HttpResponse {
    let con = Connection::open("rogger.db").unwrap();
    let post_file = include_str!("../web/post.html");
    if let Ok(Some(post)) = Database::get_post(con, pid.into_inner()) {
       HttpResponse::Ok().body(post_file.replace("{{post_name}}",&post.title).replace("{{post_text}}",&post.html_content).replace("{{post_date}}",&post.date).replace("{{author_name}}",YOUR_NAME))
    } else {
      HttpResponse::Ok().body("Post with this id does not exist")
    }
}

#[derive(Deserialize)]
struct AddPost {
   api_key: String,
   name: String,
   text: String,
}

#[post("/api/addPost")]
async fn add_post(form: web::Form<AddPost>,  user: web::Data<Arc<Mutex<User>>>) -> HttpResponse {
   if user.lock().unwrap().validate_key(form.api_key.to_string()) {
      let con = Connection::open("rogger.db").unwrap();
      Database::push_post(con, &form.name, &form.text);
      HttpResponse::Ok().body(format!("Added {} to database",&form.name))
   } else {
      HttpResponse::Ok().body("Api key is not correct")
   }
}

#[derive(Deserialize)]
struct ModPost {
   api_key: String,
   id: usize,
   name: String,
   text: String,
}

#[post("/api/editPost")]
async fn modify_post(form: web::Form<ModPost>,  user: web::Data<Arc<Mutex<User>>>) -> HttpResponse {
    if user.lock().unwrap().validate_key(form.api_key.to_string()) {
       let con = Connection::open("rogger.db").unwrap();
       Database::edit_post(con, form.id, &form.name, &form.text);
       HttpResponse::Ok().body(format!("Modified {} post in database",form.id))
    } else {
       HttpResponse::Ok().body("Api key is not correct")
    }
}

#[derive(Deserialize)]
struct RmPost {
   api_key: String,
   id: usize,
}

#[post("/api/removePost")]
async fn remove_post(form: web::Form<RmPost>,  user: web::Data<Arc<Mutex<User>>>) -> HttpResponse {
    if user.lock().unwrap().validate_key(form.api_key.to_string()) {
       let con = Connection::open("rogger.db").unwrap();
       Database::rm_post(con, form.id);
       HttpResponse::Ok().body(format!("Post with id {} has been removed",form.id))
    } else {
       HttpResponse::Ok().body("Api key is not correct")
    }
}

#[derive(Deserialize)]
struct Login {
    master_key: String,
}

#[post("/api/genKey")]
async fn generate_key(form: web::Form<Login>, user: web::Data<Arc<Mutex<User>>>) -> HttpResponse {
   if user.lock().unwrap().validate(form.master_key.to_string()) {
      HttpResponse::Ok().body(user.lock().unwrap().new_key())
   }
   else {
      HttpResponse::Ok().body("Your Masterkey is not correct")
   }
}

#[derive(Deserialize)]
struct RmKey {
   master_key: String,
   key_id: usize,
}

#[post("/api/rmKey")]
async fn remove_key(form: web::Form<RmKey>, user: web::Data<Arc<Mutex<User>>>) -> HttpResponse {
    if user.lock().unwrap().validate(form.master_key.to_string()) {
       user.lock().unwrap().remove_key(form.key_id);
       HttpResponse::Ok().body(format!("Key with id {} has been removed",form.key_id))
    }
    else {
       HttpResponse::Ok().body("Your Masterkey is not correct")
    }
}

#[get("/css/main.css")]
async fn css_main() -> HttpResponse {
    let css_file = include_str!("../web/css/main.css");
    HttpResponse::Ok().body(css_file)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Database::new();
    let user = web::Data::new(Arc::new(Mutex::new(User::new())));
    HttpServer::new(move || {
        App::new()
	    .app_data(user.clone())
            .service(index)
            .service(list_posts)
            .service(get_post)
            .service(add_post)
            .service(modify_post)
            .service(remove_post)
	    .service(generate_key)
	    .service(remove_key)
            .service(css_main)
})
    .bind(("0.0.0.0", 1337))?
    .run()
    .await
}
