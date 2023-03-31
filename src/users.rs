use sha2::{Sha256, Sha512, Digest};
use rand::distributions::{Alphanumeric, DistString};

#[derive(Clone)]
pub struct User {
    master_key: String,
    keys: Vec<String>,
}

impl User {
     pub fn new() -> Self {
        let rng_str: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
        let mut hasher = Sha512::new();
        hasher.update(rng_str);
	let master_hash = hasher.finalize();
	println!("Your master key (never show it to anyone)\n{:x}", master_hash);
        User { master_key: format!("{:x}",master_hash),
	       keys: Vec::new(),
	}
     }

     pub fn validate(&self, master_key: String) -> bool {
     	 if self.master_key == master_key {
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
