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
mod dynamic_site;
use dynamic_site::{Pages, DynVal};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    blog_name: &'a str,
    index_data: &'a str,
    favicon: &'a str,
}

#[get("/")]
async fn index(pages: web::Data<Pages>, strings: web::Data<DynVal>) -> HttpResponse {
    let index_file = IndexTemplate { blog_name: &strings.your_name.read().unwrap(), index_data: &pages.get_index().html_content, favicon: &strings.favicon.read().unwrap(), };
    HttpResponse::Ok().body(index_file.render().unwrap())
}

#[derive(Template)]
#[template(path = "aboutme.html")]
struct AboutMeTemplate<'a> {
    blog_name: &'a str,
    aboutme_data: &'a str,
    favicon: &'a str,
}

#[get("/aboutme")]
async fn aboutme(pages: web::Data<Pages>, strings: web::Data<DynVal>) -> HttpResponse {
    let aboutme_site = AboutMeTemplate { blog_name: &strings.blog_name.read().unwrap(), aboutme_data: &pages.get_aboutme().html_content, favicon: &strings.favicon.read().unwrap(), };
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
    favicon: &'a str,
}

#[get("/posts/{pathid}")]
async fn list_posts(pathid: actix_web::web::Path<usize>, cache: web::Data<Cache>, strings: web::Data<DynVal>) -> HttpResponse {
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
    } else if let Ok(posts_vec) = Database::get_list(con , offset-1) { posts = posts_vec; }
        else {
            return HttpResponse::Ok().body("Cannot find posts with this id");
        }
    let posts_file = PostsTemplate { blog_name: &strings.blog_name.read().unwrap(), your_name: &strings.your_name.read().unwrap(), posts: &posts, counter: [offset-1, offset, offset+1], curr_page: offset, favicon: &strings.favicon.read().unwrap(), };
    HttpResponse::Ok().body(posts_file.render().unwrap())
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate<'a> {
    blog_name: &'a str,
    post_name: &'a str,
    post_text: &'a str,
    post_date: &'a str,
    favicon: &'a str,
}

#[get("/post/{pid}")]
async fn get_post(pid: actix_web::web::Path<usize>, cache: web::Data<Cache>, strings: web::Data<DynVal>) -> HttpResponse {
    let con = Connection::open("rogger.db").unwrap();
    let inner_pid = pid.into_inner();
    let post: Post;
    if inner_pid < 101 && inner_pid < cache.posts.read().unwrap().len() {
	post = cache.get_by_id(inner_pid);
    } else if let Ok(Some(this_post)) = Database::get_post(con, inner_pid) {
        post = this_post;
    } else {
        return HttpResponse::NotFound().body("Post with this id does not exist");
    }
    let post_file = PostTemplate { blog_name: &strings.blog_name.read().unwrap(), post_name: &post.title, post_text: &post.html_content, post_date: &post.date, favicon: &strings.favicon.read().unwrap(), };
    HttpResponse::Ok().body(post_file.render().unwrap())
}

#[derive(Template)]
#[template(path = "cms/index.html")]
struct CMSTemplate<'a> {
    master_user_login: &'a str,
    blog_name: &'a str,
    author_name: &'a str,
    favicon: &'a str,
}

#[get("/cms/")]
async fn cms(req: HttpRequest, strings: web::Data<DynVal>) -> HttpResponse {
    if let Some(cookie) = req.cookie("session") {
	if User::validate_key(cookie.value().to_string(), "sessions") {
	    let cms_file = CMSTemplate { master_user_login: &strings.master_user_login.read().unwrap(), blog_name: &strings.blog_name.read().unwrap(), author_name: &strings.your_name.read().unwrap(), favicon: &strings.favicon.read().unwrap(), };
	    HttpResponse::Ok().body(cms_file.render().unwrap())
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
			   HttpResponse::Found().cookie(session_cookie).append_header(("Location","/")).finish() }
		Err(_) => { HttpResponse::InternalServerError().body("Internal server error") }
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
    master_user_login: &'a str,
    your_name: &'a str,
    posts: &'a [Post],
    counter: [usize; 3],
    curr_page: usize,
}

#[get("/cms/posts/{pathid}")]
async fn cms_posts(pathid: actix_web::web::Path<usize>, cache: web::Data<Cache>, req: HttpRequest, strings: web::Data<DynVal>) -> HttpResponse {
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
      } else if let Ok(posts_vec) = Database::get_list(con , offset-1) { posts = posts_vec; }
        else {
            return HttpResponse::Ok().body("Cannot find posts with this id");
        }
	  let posts_file = CmsPostsTemplate { master_user_login: &strings.master_user_login.read().unwrap(), your_name: &strings.your_name.read().unwrap(), posts: &posts, counter: [offset-1, offset, offset+1], curr_page: offset };
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
	    let post_cms_file = CmsAboutMe { operation: "edit", post_title: "Index", server_path: "/api/indexEdit", post_edit: "", initial_val: &pages.get_index().content, };
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
	    let post_cms_file = CmsAboutMe { operation: "edit", post_title: "About me", server_path: "/api/aboutmeEdit", post_edit: "", initial_val: &pages.get_aboutme().content, };
	    HttpResponse::Ok().body(post_cms_file.render().unwrap())
	} else {
	    HttpResponse::Found().append_header(("Location","/cms/login")).finish()
	}
    } else {
	HttpResponse::Found().append_header(("Location","/cms/login")).finish()
    }
}


#[derive(Template)]
#[template(path = "cms/auth.html")]
struct AuthTemplate<'a> {
    api_keys: &'a [String],
    master_user_login: &'a str,
}

#[get("/cms/authorization")]
async fn cms_auth(req: HttpRequest, strings: web::Data<DynVal>) -> HttpResponse {
    if let Some(cookie) = req.cookie("session") {
	if User::validate_key(cookie.value().to_string(), "sessions") {
	    let auth_file = AuthTemplate { api_keys: &User::get_keys(), master_user_login: &strings.master_user_login.read().unwrap() };
	    HttpResponse::Ok().body(auth_file.render().unwrap())
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
	   let post_cms_file = CmsNewPostTemplate { operation: "edit", post_title: &post.title, server_path: "/api/editPost", post_edit: &format!("id={}&", inner_pid), initial_val: &post.content.replace('`', "\\`")};
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
			HttpResponse::Ok().body(format!("Added {} to database",&form.name)) }
	    Err(_) => { HttpResponse::InternalServerError().body("Cannot add post because of Database error") }
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
			HttpResponse::Ok().body(format!("Modified {} post in database",form.id)) }
	    Err(_) => { HttpResponse::InternalServerError().body("Cannot edit post because of Database error") }
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
			HttpResponse::Ok().body(format!("Post with id {} has been removed",form.id)) }
	    Err(_) => {  HttpResponse::InternalServerError().body("Cannot remove post because of Database error") }
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
   if User::validate(form.login.to_string(), form.password.to_string()) || User::validate_key(form.password.to_string(), "sessions") {
      HttpResponse::Ok().body(User::new_key())
   }
   else {
      HttpResponse::Unauthorized().body("Your login credentials are not correct")
   }
}

#[post("/api/getKeys")]
async fn get_keys(form: web::Form<Login>) -> HttpResponse {
    if User::validate(form.login.to_string(), form.password.to_string()) {
	HttpResponse::Ok().body(User::get_keys().join("\n"))
    }
    else {
	HttpResponse::Unauthorized().body("Your login credentials are not correct")
    }
}


#[derive(Deserialize)]
struct RmKey {
    login: String,
    password: String,
    key: String,
}

#[post("/api/rmKey")]
async fn rm_key(form: web::Form<RmKey>) -> HttpResponse {
    if User::validate(form.login.to_string(), form.password.to_string()) || User::validate_key(form.password.to_string(), "sessions")  {
	User::rm_key(form.key.to_string());
	HttpResponse::Ok().body(format!("Removed {} key",form.key))
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
	pages.modify_aboutme(form.text.to_string());
	HttpResponse::Ok().body("Aboutme website has been modified")
    } else {
	HttpResponse::Unauthorized().body("Api key is not correct")
    }
}

#[post("/api/indexEdit")]
async fn index_edit(form: web::Form<AboutMeForm>, pages: web::Data<Pages>) -> HttpResponse {
    if User::validate_key(form.api_key.to_string(), "keys") || User::validate_key(form.api_key.to_string(), "sessions")  {
	pages.modify_index(form.text.to_string());
	HttpResponse::Ok().body("Index website has been modified")
    } else {
	HttpResponse::Unauthorized().body("Api key is not correct")
    }
}

#[post("/api/blognameEdit")]
async fn blogname_edit(form: web::Form<AboutMeForm>, strings: web::Data<DynVal>) -> HttpResponse {
    if User::validate_key(form.api_key.to_string(), "keys") || User::validate_key(form.api_key.to_string(), "sessions")  {
    *strings.blog_name.write().unwrap() = form.text.to_string();
	HttpResponse::Ok().body("Blog name has been modified")
    } else {
	HttpResponse::Unauthorized().body("Api key is not correct")
    }
}

#[post("/api/authornameEdit")]
async fn author_edit(form: web::Form<AboutMeForm>, strings: web::Data<DynVal>) -> HttpResponse {
    if User::validate_key(form.api_key.to_string(), "keys") || User::validate_key(form.api_key.to_string(), "sessions")  {
    *strings.your_name.write().unwrap() = form.text.to_string();
	HttpResponse::Ok().body("Author name has been modified")
    } else {
	HttpResponse::Unauthorized().body("Api key is not correct")
    }
}

#[post("/api/faviconEdit")]
async fn favicon_edit(form: web::Form<AboutMeForm>, strings: web::Data<DynVal>) -> HttpResponse {
    if User::validate_key(form.api_key.to_string(), "keys") || User::validate_key(form.api_key.to_string(), "sessions")  {
    *strings.favicon.write().unwrap() = form.text.to_string();
	HttpResponse::Ok().body("Favicon link has been modified")
    } else {
	HttpResponse::Unauthorized().body("Api key is not correct")
    }
}

#[derive(Deserialize)]
struct NewUsername {
    login: String,
    password: String,
    new_username: String,
}

#[post("/api/newMasterUser")]
async fn master_new(form: web::Form<NewUsername>, strings: web::Data<DynVal>) -> HttpResponse {
    if User::validate(form.login.to_string(), form.password.to_string()) || User::validate_key(form.password.to_string(), "sessions")  {
	if !form.new_username.is_empty() {
	    let password: &str = &User::new_master_user(&form.new_username);
        *strings.master_user_login.write().unwrap() = form.new_username.to_string();
	    HttpResponse::Ok().body(format!("New master user credentials:\nusername: {}\npassword: {}\n",form.new_username, password))
	} else {
	    HttpResponse::BadRequest().body("Master user username cannot be empty")
	}
    }
    else {
	HttpResponse::Unauthorized().body("Your login credentials are not correct")
    }
}

#[get("/favicon.ico")]
async fn favicon() -> HttpResponse {
    HttpResponse::Ok().body(include_bytes!("../templates/img/favicon.ico").to_vec())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match Database::connect() {
	Ok(_) => { println!("Connected to SQLite Successfully!");}
	Err(error) => { println!("Cannot connect to SQLite database because of: {}", error);}
    }
    User::init_master();
    let cache = web::Data::new(Cache::new());
    let pages = web::Data::new(Pages::new());
    let strings = web::Data::new(DynVal::new(vec![String::from("Example blog name"), String::from("blogger"), String::from("Rogger_Admin"), String::from("/favicon.ico")]));
    HttpServer::new(move || {
        App::new()
	    .app_data(cache.clone())
	    .app_data(pages.clone())
	    .app_data(strings.clone())
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
	    .service(get_keys)
	    .service(rm_key)
	    .service(web::redirect("/cms", "/cms/"))
	    .service(cms)
	    .service(web::redirect("/cms/posts", "/cms/posts/1"))
	    .service(cms_posts)
	    .service(cms_add_post)
	    .service(cms_aboutme)
	    .service(cms_index)
	    .service(cms_auth)
	    .service(master_new)
	    .service(cms_edit_post)
	    .service(cms_login)
	    .service(cms_login_site)
	    .service(end_session)
	    .service(aboutme)
	    .service(aboutme_edit)
	    .service(blogname_edit)
	    .service(author_edit)
	    .service(favicon_edit)
	    .service(favicon)
})
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
