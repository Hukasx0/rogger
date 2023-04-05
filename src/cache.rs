use crate::posts::{Database, Post};
use std::sync::RwLock;

pub struct Cache {
   posts: RwLock<Vec<Post>>,
}

impl Cache {
    pub fn new() -> Self {
	Cache { posts: RwLock::new(Database::get_cache().unwrap()) }
    }

    pub fn db_sync(&self) {
	*self.posts.write().unwrap() = Database::get_cache().unwrap();
    }

    pub fn get_posts(&self, offset: usize) -> Vec<Post> {
	(*self.posts.read().unwrap().iter().skip((offset-1)*10).take(10).cloned().collect::<Vec<Post>>()).to_vec()
    }

    pub fn get_by_id(&self, id: usize) -> Post {
	let post = &self.posts.read().unwrap()[(self.fst_id()-id)];
	post.clone()
    }

    pub fn fst_id(&self) -> usize {
	self.posts.read().unwrap().first().unwrap().id
    }
}
