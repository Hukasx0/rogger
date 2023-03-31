use sha2::{Sha256, Digest};
use rand::distributions::{Alphanumeric, DistString};

#[derive(Clone)]
pub struct User {
    login: String,
    password_hash: String,
    keys: Vec<String>,
}

impl User {
     pub fn new() -> Self {
        let mut hasher = Sha256::new();
        hasher.update("pass123");
        User { login: "admin".to_string(),
	       password_hash: format!("{:x}",hasher.finalize()),
	       keys: Vec::new(),
	}
     }

     pub fn validate(&self, login: String, password: String) -> bool {
         let mut hasher = Sha256::new();
         hasher.update(password);
     	 if self.login == login
	     && self.password_hash == format!("{:x}",hasher.finalize()) {
	    true
	 }
	 else {
	    false
	 }
     }

     pub fn new_key(&mut self) -> String {
        let rng_str: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
	let mut hasher = Sha256::new();
	hasher.update(rng_str);
	let hex = format!("{:x}",hasher.finalize());
	self.keys.push(hex.to_string());
	hex
     }

     pub fn remove_key(&mut self, id: usize) {
     	self.keys.remove(id);
     }

     pub fn validate_key(&self, api_key: String) -> bool {
        if self.keys.contains(&api_key) {
	   true
	} else {
	   false
	}
     }
}
