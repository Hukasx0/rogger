use actix_web::{get, post, App, web, HttpResponse, HttpServer, HttpRequest, cookie::CookieBuilder, cookie::time::Duration};
use serde::Deserialize;
use rusqlite::Connection;
use askama::Template;
mod posts;
use posts::{Database, Post};
mod users;
use users::User;
mod cache;
use cache::Cache;
mod rogger_cfg;
use rogger_cfg::{BLOG_NAME, YOUR_NAME};
mod dynamic_site;
use dynamic_site::Pages;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    blog_name: &'a str,
    index_data: &'a str,
}

#[get("/")]
async fn index(pages: web::Data<Pages>) -> HttpResponse {
    let index_file = IndexTemplate { blog_name: BLOG_NAME, index_data: &pages.get_site(0).html_content, };
    HttpResponse::Ok().body(index_file.render().unwrap())
}

#[derive(Template)]
#[template(path = "aboutme.html")]
struct AboutMeTemplate<'a> {
    blog_name: &'a str,
    aboutme_data: &'a str,
}

#[get("/aboutme")]
async fn aboutme(pages: web::Data<Pages>) -> HttpResponse {
    let aboutme_site = AboutMeTemplate { blog_name: BLOG_NAME, aboutme_data: &pages.get_site(1).html_content, };
    HttpResponse::Ok().body(aboutme_site.render().unwrap())
}

#[derive(Template)]
#[template(path = "posts.html")]
struct PostsTemplate<'a> {
    blog_name: &'a str,
    your_name: &'a str,
    posts: &'a [Post],
    counter: [usize; 3],
    curr_page: usize,
}

#[get("/posts/{pathid}")]
async fn list_posts(pathid: actix_web::web::Path<usize>, cache: web::Data<Cache>) -> HttpResponse {
    let con = Connection::open("rogger.db").unwrap();
    let inner_path = pathid.into_inner();
    let offset = if inner_path > 1 {
       inner_path
    } else {
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
    let posts_file = PostsTemplate { blog_name: BLOG_NAME, your_name: YOUR_NAME, posts: &posts, counter: [offset-1, offset, offset+1], curr_page: offset };
    HttpResponse::Ok().body(posts_file.render().unwrap())
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate<'a> {
    blog_name: &'a str,
    post_name: &'a str,
    post_text: &'a str,
    post_date: &'a str,
}

#[get("/post/{pid}")]
async fn get_post(pid: actix_web::web::Path<usize>, cache: web::Data<Cache>) -> HttpResponse {
    let con = Connection::open("rogger.db").unwrap();
    let inner_pid = pid.into_inner();
    let post: Post;
    if inner_pid < 101 && inner_pid < cache.posts.read().unwrap().len() {
	post = cache.get_by_id(inner_pid);
    } else {
	if let Ok(Some(this_post)) = Database::get_post(con, inner_pid) {
            post = this_post;
	} else {
	    return HttpResponse::NotFound().body("Post with this id does not exist");
	}
    }
    let post_file = PostTemplate { blog_name: BLOG_NAME, post_name: &post.title, post_text: &post.html_content, post_date: &post.date };
    HttpResponse::Ok().body(post_file.render().unwrap())
}

#[get("/cms/")]
async fn cms(req: HttpRequest) -> HttpResponse {
    if let Some(cookie) = req.cookie("session") {
	if User::validate_key(cookie.value().to_string(), "sessions") {
	    HttpResponse::Ok().body(include_str!("../templates/cms/index.html").to_string())
	} else {
	    HttpResponse::Found().append_header(("Location","/cms/login")).finish()
	}
    } else {
	HttpResponse::Found().append_header(("Location","/cms/login")).finish()
    }    
}

#[get("/cms/login")]
async fn cms_login_site() -> HttpResponse {
   HttpResponse::Ok().body(include_str!("../templates/login.html").to_string())
}

#[derive(Deserialize)]
struct CmsLogin {
   login: String,
   password: String,
}

#[post("/cms/login")]
async fn cms_login(form: web::Form<CmsLogin>) -> HttpResponse {
   if User::validate(form.login.to_string(), form.password.to_string()) {
       let session_cookie = CookieBuilder::new("session", User::new_session()).path("/").max_age(Duration::minutes(10)).finish();
       HttpResponse::Found().cookie(session_cookie).append_header(("Location","/cms/")).finish()
   } else {
      HttpResponse::Unauthorized().body("Wrong credentials")
   }   
}

#[get("/api/endSession")]
async fn end_session(req: HttpRequest) -> HttpResponse {
    if let Some(sessionc) = req.cookie("session") {
	let session = sessionc.value();
	if User::validate_key(session.to_string(), "sessions") {
	    match User::end_session(session.to_string()) {
		Ok(_) => { let session_cookie = CookieBuilder::new("session", "").path("/").max_age(Duration::seconds(0)).finish();
			   return HttpResponse::Found().cookie(session_cookie).append_header(("Location","/")).finish(); }
		Err(_) => { return HttpResponse::InternalServerError().body("Internal server error"); }
	    }
	} else {
	    HttpResponse::Unauthorized().body("Icorrect session id")
	}
    } else {
	HttpResponse::Found().append_header(("Location","/cms/login")).finish()
    }
}

#[derive(Template)]
#[template(path = "cms/posts.html")]
struct CmsPostsTemplate<'a> {
    your_name: &'a str,
    posts: &'a [Post],
    counter: [usize; 3],
    curr_page: usize,
}

#[get("/cms/posts/{pathid}")]
async fn cms_posts(pathid: actix_web::web::Path<usize>, cache: web::Data<Cache>, req: HttpRequest) -> HttpResponse {
   if let Some(cookie) = req.cookie("session") {
      if User::validate_key(cookie.value().to_string(), "sessions") {
	  let con = Connection::open("rogger.db").unwrap();
	  let inner_path = pathid.into_inner();
	  let offset = if inner_path > 1 {
	      inner_path
	  } else {
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
	  let posts_file = CmsPostsTemplate { your_name: YOUR_NAME, posts: &posts, counter: [offset-1, offset, offset+1], curr_page: offset };
	  HttpResponse::Ok().body(posts_file.render().unwrap())
      } else {
          HttpResponse::Found().append_header(("Location","/cms/login")).finish()
      }
   } else {
      HttpResponse::Found().append_header(("Location","/cms/login")).finish()
   }
}

#[derive(Template)]
#[template(path = "cms/post.html")]
struct CmsNewPostTemplate<'a> {
    operation: &'a str,
    post_title: &'a str,
    server_path: &'a str,
    post_edit: &'a str,
    initial_val: &'a str,
}

#[get("/cms/post_new")]
async fn cms_add_post(req: HttpRequest) -> HttpResponse {
   if let Some(cookie) = req.cookie("session") {
      if User::validate_key(cookie.value().to_string(), "sessions") {
	  let post_cms_file = CmsNewPostTemplate { operation: "upload", post_title: "", server_path: "/api/addPost", post_edit: "", initial_val: ""};
	  HttpResponse::Ok().body(post_cms_file.render().unwrap())
      } else {
	  HttpResponse::Found().append_header(("Location","/cms/login")).finish()
      }
   } else {
       HttpResponse::Found().append_header(("Location","/cms/login")).finish()
   }
}

#[derive(Template)]
#[template(path = "cms/post.html")]
struct CmsAboutMe<'a> {
    operation: &'a str,
    post_title: &'a str,
    server_path: &'a str,
    post_edit: &'a str,
    initial_val: &'a str,
}

#[get("/cms/index")]
async fn cms_index(req: HttpRequest, pages: web::Data<Pages>) -> HttpResponse {
    if let Some(cookie) = req.cookie("session") {
	if User::validate_key(cookie.value().to_string(), "sessions") {
	    let post_cms_file = CmsAboutMe { operation: "edit", post_title: "Index", server_path: "/api/indexEdit", post_edit: "", initial_val: &pages.get_site(0).content, };
	    HttpResponse::Ok().body(post_cms_file.render().unwrap())
	} else {
	    HttpResponse::Found().append_header(("Location","/cms/login")).finish()
	}
    } else {
	HttpResponse::Found().append_header(("Location","/cms/login")).finish()
    }
}


#[get("/cms/aboutme")]
async fn cms_aboutme(req: HttpRequest, pages: web::Data<Pages>) -> HttpResponse {
    if let Some(cookie) = req.cookie("session") {
	if User::validate_key(cookie.value().to_string(), "sessions") {
	    let post_cms_file = CmsAboutMe { operation: "edit", post_title: "About me", server_path: "/api/aboutmeEdit", post_edit: "", initial_val: &pages.get_site(1).content, };
	    HttpResponse::Ok().body(post_cms_file.render().unwrap())
	} else {
	    HttpResponse::Found().append_header(("Location","/cms/login")).finish()
	}
    } else {
	HttpResponse::Found().append_header(("Location","/cms/login")).finish()
    }
}


#[derive(Template)]
#[template(path = "cms/post.html")]
struct CmsEditPostTemplate<'a> {
    operation: &'a str,
    post_title: &'a str,
    server_path: &'a str,
    post_edit: &'a str,
    initial_val: &'a str,
}

#[get("/cms/post_edit/{pid}")]
async fn cms_edit_post(req: HttpRequest, pid: actix_web::web::Path<usize>, cache: web::Data<Cache>) -> HttpResponse {
    if let Some(cookie) = req.cookie("session") {
       if User::validate_key(cookie.value().to_string(), "sessions") {
	   let inner_pid = pid.into_inner();
	    let post: Post;
	    if inner_pid < 101 {
		post = cache.get_by_id(inner_pid);
	    } else {
		let con = Connection::open("rogger.db").unwrap();
		if let Ok(Some(this_post)) = Database::get_post(con, inner_pid) {
		    post = this_post;
		} else {
		    return HttpResponse::NotFound().body("Post with this id does not exist");
		}
	    }
	   let post_cms_file = CmsNewPostTemplate { operation: "edit", post_title: &post.title, server_path: "/api/editPost", post_edit: &format!("id={}&", inner_pid), initial_val: &post.content.replace("`", "\\`")};
           HttpResponse::Ok().body(post_cms_file.render().unwrap())
      } else {
           HttpResponse::Found().append_header(("Location","/cms/login")).finish()
      }
   } else {
	HttpResponse::Found().append_header(("Location","/cms/login")).finish()
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
   if User::validate_key(form.api_key.to_string(), "keys") || User::validate_key(form.api_key.to_string(), "sessions"){
      let con = Connection::open("rogger.db").unwrap();
       match Database::push_post(con, &form.name, &form.text) {
	    Ok(_) => { 	cache.db_sync();
			return HttpResponse::Ok().body(format!("Added {} to database",&form.name)); }
	    Err(_) => { return  HttpResponse::InternalServerError().body("Cannot add post because of Database error"); }
	}
   } else {
      HttpResponse::Unauthorized().body("Api key is not correct")
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
    if User::validate_key(form.api_key.to_string(), "keys") || User::validate_key(form.api_key.to_string(), "sessions"){
	let con = Connection::open("rogger.db").unwrap();
	match Database::edit_post(con, form.id, &form.name, &form.text) {
	    Ok(_) => { 	cache.db_sync();
			return HttpResponse::Ok().body(format!("Modified {} post in database",form.id)); }
	    Err(_) => { return  HttpResponse::InternalServerError().body("Cannot edit post because of Database error"); }
	}
    } else {
       HttpResponse::Unauthorized().body("Api key is not correct")
    }
}

#[derive(Deserialize)]
struct RmPost {
   api_key: String,
   id: usize,
}

#[post("/api/removePost")]
async fn remove_post(form: web::Form<RmPost>, cache: web::Data<Cache>) -> HttpResponse {
    if User::validate_key(form.api_key.to_string(), "keys") || User::validate_key(form.api_key.to_string(), "sessions")  {
       let con = Connection::open("rogger.db").unwrap();
	match Database::rm_post(con, form.id) {
	    Ok(_) => { 	cache.db_sync();
			return HttpResponse::Ok().body(format!("Post with id {} has been removed",form.id)); }
	    Err(_) => {  return HttpResponse::InternalServerError().body("Cannot remove post because of Database error"); }
	}
    } else {
       HttpResponse::Unauthorized().body("Api key is not correct")
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
      HttpResponse::Unauthorized().body("Your login credentials are not correct")
   }
}

#[derive(Deserialize)]
struct AboutMeForm {
    api_key: String,
    text: String,
} 

#[post("/api/aboutmeEdit")]
async fn aboutme_edit(form: web::Form<AboutMeForm>, pages: web::Data<Pages>) -> HttpResponse {
    if User::validate_key(form.api_key.to_string(), "keys") || User::validate_key(form.api_key.to_string(), "sessions")  {
	pages.modify_site(1, form.text.to_string());
	HttpResponse::Ok().body("Aboutme website has been modified")
    } else {
	HttpResponse::Unauthorized().body("Api key is not correct")
    }
}

#[post("/api/indexEdit")]
async fn index_edit(form: web::Form<AboutMeForm>, pages: web::Data<Pages>) -> HttpResponse {
    if User::validate_key(form.api_key.to_string(), "keys") || User::validate_key(form.api_key.to_string(), "sessions")  {
	pages.modify_site(0, form.text.to_string());
	HttpResponse::Ok().body("Index website has been modified")
    } else {
	HttpResponse::Unauthorized().body("Api key is not correct")
    }
}

#[get("/css/main.css")]
async fn css_main() -> HttpResponse {
    let css_file = include_str!("../templates/css/main.css");
    HttpResponse::Ok().body(css_file)
}

#[get("css/cms.css")]
async fn css_cms() -> HttpResponse {
    let css_file = include_str!("../templates/css/cms.css");
    HttpResponse::Ok().body(css_file)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match Database::new() {
	Ok(_) => { println!("Connected to SQLite Successfully!");}
	Err(error) => { println!("Cannot connect to SQLite database because of: {}", error);}
    }
    User::init_master();
    let cache = web::Data::new(Cache::new());
    let pages = web::Data::new(Pages::new());
    HttpServer::new(move || {
        App::new()
	    .app_data(cache.clone())
	    .app_data(pages.clone())
            .service(index)
	    .service(index_edit)
	    .service(web::redirect("/posts", "/posts/1"))
	    .service(web::redirect("/posts/", "/posts/1"))
            .service(list_posts)
            .service(get_post)
            .service(add_post)
            .service(modify_post)
            .service(remove_post)
	    .service(generate_key)
            .service(css_main)
	    .service(css_cms)
	    .service(web::redirect("/cms", "/cms/"))
	    .service(cms)
	    .service(web::redirect("/cms/posts", "/cms/posts/1"))
	    .service(cms_posts)
	    .service(cms_add_post)
	    .service(cms_aboutme)
	    .service(cms_index)
	    .service(cms_edit_post)
	    .service(cms_login)
	    .service(cms_login_site)
	    .service(end_session)
	    .service(aboutme)
	    .service(aboutme_edit)
})
    .bind(("0.0.0.0", 1337))?
    .run()
    .await
}
