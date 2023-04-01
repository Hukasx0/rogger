use std::sync::{Arc, Mutex};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use bincode::{serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Post {
   pub id: usize,
   pub name: String,
   pub text: String,
   pub html_text: String,
   pub date: String,
}

pub struct Posts {
   posts: Arc<Mutex<Vec<Post>>>,
}

impl Posts {
   pub fn new() -> Self {
      Posts { posts: Arc::new(Mutex::new(Vec::new())) }
   }

   pub fn db_check_create() {
      let db_file = OpenOptions::new()
      	  	    .write(true)
		    .create_new(true)
		    .open("rogger.bin");
      match db_file {
         Ok(_) => drop(db_file),
	 Err(e) => println!("Error while creating database file {:?}", e),
      };
   }

   pub fn save_db(&self) {
      let posts = self.posts.lock().unwrap();
      let encoded_data: Vec<u8> = serialize(&*posts).unwrap();
      let mut db_file = File::create("rogger.bin").unwrap();
      db_file.write_all(&encoded_data).unwrap();
   }

   pub fn load_db(&self) {
      let mut db_file = File::open("rogger.bin").unwrap();
      let mut encoded_data = Vec::new();
      db_file.read_to_end(&mut encoded_data).unwrap();
      let posts = bincode::deserialize(&encoded_data);
      match posts {
         Ok(r) => *self.posts.lock().unwrap() = r,
	 Err(_) => println!("Database is empty"),
      };
   }

   pub fn get_list(&self) -> Vec<Post> {
      let posts = self.posts.lock().unwrap();
      posts.iter().cloned().collect()
   }

   pub fn get_post(&self, id: usize) -> Option<Post> {
      let posts = self.posts.lock().unwrap();
      posts.iter().find(|post| post.id == id).cloned()
   }

   pub fn push_post(&self, post: Post) {
       let mut posts = self.posts.lock().unwrap();
       let posts_len = posts.len();
       let mut last_post_id: usize = 0;
       if posts_len > 0 {
          last_post_id = &posts[posts_len-1].id+1;
       }
       posts.push(Post {
          id: last_post_id,
	  name: post.name,
	  text: post.text,
	  html_text: markdown::to_html(&post.html_text),
	  date: post.date,
       });
   }

   pub fn edit_post(&self, mut post: Post) {
       let mut posts = self.posts.lock().unwrap();
       if let Some(post_to_update) = posts.iter_mut().find(|cpost| cpost.id == post.id) {
           post.html_text = markdown::to_html(&post.html_text);
          *post_to_update = post;
       }
   }

   pub fn rm_post(&self, id: usize) {
       let mut posts = self.posts.lock().unwrap();
       posts.retain(|post| post.id != id);
   }
}
