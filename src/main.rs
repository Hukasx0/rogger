use actix_web::{get, post, App, web, HttpResponse, HttpServer, HttpRequest, cookie::Cookie};
use serde::Deserialize;
use rusqlite::Connection;
mod posts;
use posts::{Database, Post};
mod users;
use users::User;
mod cache;
use cache::Cache;

static YOUR_NAME: &str = "Hubert";
static YOUR_DESCRIPTION: &str = "test description, which is a placeholder";

#[get("/")]
async fn index() -> HttpResponse {
    let index_file = include_str!("../web/index.html").replace("{{author_name}}",YOUR_NAME).replace("{{author_description}}",YOUR_DESCRIPTION);
    HttpResponse::Ok().body(index_file)
}

#[get("/posts/{pathid}")]
async fn list_posts(pathid: actix_web::web::Path<usize>, cache: web::Data<Cache>) -> HttpResponse {
    let con = Connection::open("rogger.db").unwrap();
    let mut posts_file: String = include_str!("../web/posts.html").to_string();
    let mut post_list = String::new();
    let inner_path = pathid.into_inner();
    let offset = if inner_path > 1 {
       posts_file = posts_file.replace("{{counter}}", &format!(r#"<p><a href="/posts/{}">{}</a> <span style="color: rgb(242, 242, 242);">{}</span> <a href="/posts/{}">{}</a></p>"#,
       			                 	      inner_path-1, inner_path-1, inner_path, inner_path+1, inner_path+1));
       inner_path
    } else {
       posts_file = posts_file.replace("{{counter}}", r#"<p><span style="color: rgb(242, 242, 242);">1</span> <a href="/posts/2">2</a></p>"#);
       1
    };
    let posts: Vec<Post>;
    if inner_path < 11 {
	posts = cache.get_posts(inner_path);
    } else {
	if let Ok(posts_vec) = Database::get_list(con , offset-1) { posts = posts_vec; }
	 else {
	     return HttpResponse::Ok().body("Cannot find posts with this id");
	 }
    }
    for post in posts {
    	post_list.push_str(&format!(r#"
	<div class="post">
	   <h2 class="title"><a href="/post/{}"><b>{}</b></a></h2>
	   <p class="description">{}</p>
	   <span class="date">{}</span>
	</div>
	"#,post.id ,post.title, format!("{}...<br><a href=\"/post/{}\"><b>Read more</b></a>", post.content.chars().take(355).collect::<String>(), post.id), post.date));
    }
    HttpResponse::Ok().body(posts_file.replace("{{author_name}}",YOUR_NAME).replace("{{post_list}}",&post_list))
}

#[get("/post/{pid}")]
async fn get_post(pid: actix_web::web::Path<usize>, cache: web::Data<Cache>) -> HttpResponse {
    let con = Connection::open("rogger.db").unwrap();
    let post_file = include_str!("../web/post.html");
    let inner_pid = pid.into_inner();
    let post: Post;
    if inner_pid < 101 {
	post = cache.get_by_id(inner_pid);
    } else {
	if let Ok(Some(this_post)) = Database::get_post(con, inner_pid) {
            post = this_post;
	} else {
	    return HttpResponse::Ok().body("Post with this id does not exist");
	}
    }
       HttpResponse::Ok().body(post_file.replace("{{post_name}}",&post.title).replace("{{post_text}}",&post.html_content).replace("{{post_date}}",&post.date).replace("{{author_name}}",YOUR_NAME))
}

#[get("/cms/login")]
async fn cms_login_site() -> HttpResponse {
   HttpResponse::Ok().body(include_str!("../web/cms/login.html").to_string())
}

#[derive(Deserialize)]
struct CmsLogin {
   login: String,
   password: String,
}


#[post("/cms/login")]
async fn cms_login(form: web::Form<CmsLogin>) -> HttpResponse {
   if User::validate(form.login.to_string(), form.password.to_string()) {
      let session_cookie = Cookie::new("session", User::new_session());
      HttpResponse::Ok().cookie(session_cookie).body("Successfully logged in")
   } else {
      HttpResponse::Ok().body("Wrong credentials")
   }   
}

#[get("/cms/posts/{pathid}")]
async fn cms(pathid: actix_web::web::Path<usize>, cache: web::Data<Cache>, req: HttpRequest) -> HttpResponse {
   if let Some(cookie) = req.cookie("session") {
      if User::validate_key(cookie.value().to_string(), "sessions") {
	  let con = Connection::open("rogger.db").unwrap();
	  let mut posts_file: String = include_str!("../web/cms/posts.html").to_string();
	  let mut post_list = String::new();
	  let inner_path = pathid.into_inner();
	  let offset = if inner_path > 1 {
	      posts_file = posts_file.replace("{{counter}}", &format!(r#"<p><a href="/cms/posts/{}">{}</a> <span style="color: rgb(242, 242, 242);">{}</span> <a href="/cms/posts/{}">{}</a></p>"#,
       			                 	      inner_path-1, inner_path-1, inner_path, inner_path+1, inner_path+1));
	      inner_path
	  } else {
	      posts_file = posts_file.replace("{{counter}}", r#"<p><span style="color: rgb(242, 242, 242);">1</span> <a href="/cms/posts/2">2</a></p>"#);
	      1
	  };
	  let posts: Vec<Post>;
	  if inner_path < 11 {
	      posts = cache.get_posts(inner_path);
	  } else {
	      if let Ok(posts_vec) = Database::get_list(con , offset-1) { posts = posts_vec; }
	      else {
		  return HttpResponse::Ok().body("Cannot find posts with this id");
	      }
	  }
	  for post in posts {
    	      post_list.push_str(&format!(r#"
	<div class="post">
	   <h2 class="title"><a href="/cms/post_edit/{}"><b>{}</b></a></h2>
	   <p class="description">{}</p>
	   <span class="date">{}</span>
           <p class="remove-link" onclick="rmp({})"><b>remove post</b></p>
	</div>
	"#,post.id ,post.title, format!("{}...<br><a href=\"/cms/post_edit/{}\"><b>Edit post</b></a>", post.content.chars().take(355).collect::<String>(), post.id), post.date, post.id));
	  }
	  HttpResponse::Ok().body(posts_file.replace("{{author_name}}",YOUR_NAME).replace("{{post_list}}",&post_list))
      } else {
          HttpResponse::Ok().body("Incorrect session id")
      }
   } else {
       HttpResponse::Ok().body("You need to log in")
   }
}

#[get("/cms/post_new")]
async fn cms_add_post(req: HttpRequest) -> HttpResponse {
   if let Some(cookie) = req.cookie("session") {
      if User::validate_key(cookie.value().to_string(), "sessions") {
	  let post_cms_file = include_str!("../web/cms/post.html");
	  HttpResponse::Ok().body(post_cms_file.replace("{{operation}}", "upload")
	                          .replace("{{server-path}}", "/api/addPost")
	                          .replace("{{post_edit}}", "")
	                          .replace("{{initial_val}}", ""))
      } else {
         HttpResponse::Ok().body("Incorrect session id")
      }
   } else {
     HttpResponse::Ok().body("You need to log in")
   }
}

#[get("/cms/post_edit/{pid}")]
async fn cms_edit_post(req: HttpRequest, pid: actix_web::web::Path<usize>, cache: web::Data<Cache>) -> HttpResponse {
    if let Some(cookie) = req.cookie("session") {
       if User::validate_key(cookie.value().to_string(), "sessions") {
	   let post_cms_file = include_str!("../web/cms/post.html");
	   let inner_pid = pid.into_inner();
	    let post: Post;
	    if inner_pid < 101 {
		post = cache.get_by_id(inner_pid);
	    } else {
		let con = Connection::open("rogger.db").unwrap();
		if let Ok(Some(this_post)) = Database::get_post(con, inner_pid) {
		    post = this_post;
		} else {
		    return HttpResponse::Ok().body("Post with this id does not exist");
		}
	    }
           HttpResponse::Ok().body(post_cms_file.replace("{{operation}}", "edit")
	                           .replace("{{server-path}}","/api/editPost")
	                           .replace("{{post_edit}}", &format!("id={}&", inner_pid))
	                           .replace("{{initial_val}}", &post.content.replace("`", "\\`")))
      } else {
         HttpResponse::Ok().body("Incorrect session id")
      }
   } else {
     HttpResponse::Ok().body("You need to log in")
   }
}
    
#[derive(Deserialize)]
struct AddPost {
   api_key: String,
   name: String,
   text: String,
}

#[post("/api/addPost")]
async fn add_post(form: web::Form<AddPost>, cache: web::Data<Cache>) -> HttpResponse {
   if User::validate_key(form.api_key.to_string(), "keys") {
      let con = Connection::open("rogger.db").unwrap();
       Database::push_post(con, &form.name, &form.text);
       cache.db_sync();
       HttpResponse::Ok().body(format!("Added {} to database",&form.name))
   } else if User::validate_key(form.api_key.to_string(), "sessions") {
       let con = Connection::open("rogger.db").unwrap();
       Database::push_post(con, &form.name, &form.text);
       cache.db_sync();
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
async fn modify_post(form: web::Form<ModPost>, cache: web::Data<Cache>) -> HttpResponse {
    if User::validate_key(form.api_key.to_string(), "keys") {
       let con = Connection::open("rogger.db").unwrap();
	Database::edit_post(con, form.id, &form.name, &form.text);
	cache.db_sync();
        HttpResponse::Ok().body(format!("Modified {} post in database",form.id))
    } else if User::validate_key(form.api_key.to_string(), "sessions") {
	let con = Connection::open("rogger.db").unwrap();
	Database::edit_post(con, form.id, &form.name, &form.text);
	cache.db_sync();
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
async fn remove_post(form: web::Form<RmPost>, cache: web::Data<Cache>) -> HttpResponse {
    if User::validate_key(form.api_key.to_string(), "keys") {
       let con = Connection::open("rogger.db").unwrap();
	Database::rm_post(con, form.id);
	cache.db_sync();
        HttpResponse::Ok().body(format!("Post with id {} has been removed",form.id))
    } else if User::validate_key(form.api_key.to_string(), "sessions") {	
	let con = Connection::open("rogger.db").unwrap();
	Database::rm_post(con, form.id);
	cache.db_sync();
        HttpResponse::Ok().body(format!("Post with id {} has been removed",form.id))
    } else {
       HttpResponse::Ok().body("Api key is not correct")
    }
}

#[derive(Deserialize)]
struct Login {
    login: String,
    password: String,
}

#[post("/api/genKey")]
async fn generate_key(form: web::Form<Login>) -> HttpResponse {
   if User::validate(form.login.to_string(), form.password.to_string()) {
      HttpResponse::Ok().body(User::new_key())
   }
   else {
      HttpResponse::Ok().body("Your login credentials are not correct")
   }
}

#[get("/css/main.css")]
async fn css_main() -> HttpResponse {
    let css_file = include_str!("../web/css/main.css");
    HttpResponse::Ok().body(css_file)
}

#[get("css/cms.css")]
async fn css_cms() -> HttpResponse {
    let css_file = include_str!("../web/css/cms.css");
    HttpResponse::Ok().body(css_file)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Database::new();
    User::init_master();
    let cache = web::Data::new(Cache::new());
    HttpServer::new(move || {
        App::new()
	    .app_data(cache.clone())
            .service(index)
            .service(list_posts)
            .service(get_post)
            .service(add_post)
            .service(modify_post)
            .service(remove_post)
	    .service(generate_key)
            .service(css_main)
	    .service(css_cms)
	    .service(cms)
	    .service(cms_add_post)
	    .service(cms_edit_post)
	    .service(cms_login)
	    .service(cms_login_site)
})
    .bind(("0.0.0.0", 1337))?
    .run()
    .await
}
