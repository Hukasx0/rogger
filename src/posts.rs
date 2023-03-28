use std::sync::{Arc, Mutex};
use std::ops::Deref;

#[derive(Clone)]
pub struct Post {
   pub id: usize,
   pub name: String,
   pub text: String,
   pub date: String,
}

pub struct Posts {
   posts: Arc<Mutex<Vec<Post>>>,
}

impl Posts {
   pub fn new() -> Self {
      Posts { posts: Arc::new(Mutex::new(Vec::new())) }
   }

   pub fn get_list(&self) -> Vec<Post> {
      let posts = self.posts.lock().unwrap();
      return posts.iter().cloned().collect();
   }

   pub fn get_post(&self, id: usize) -> Option<Post> {
      let posts = self.posts.lock().unwrap();
      posts.get(id).cloned()
   }

   pub fn push_post(&self, post: Post) {
       let mut posts = self.posts.lock().unwrap();
       posts.push(post);
   }
}
