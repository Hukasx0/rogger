use std::sync::{Arc, Mutex};

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
	  date: post.date,
       });
   }

   pub fn edit_post(&self, post: Post) {
       let mut posts = self.posts.lock().unwrap();
       if let Some(post_to_update) = posts.iter_mut().find(|cpost| cpost.id == post.id) {
          *post_to_update = post;
       }
   }

   pub fn rm_post(&self, id: usize) {
       let mut posts = self.posts.lock().unwrap();
       posts.retain(|post| post.id != id);
   }
}
