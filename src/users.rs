use sha2::{Sha256, Sha512, Digest};
use rand::distributions::{Alphanumeric, DistString};
use redis::{Commands, Client, LposOptions};

pub struct User {}

impl User {
     pub fn init_master() {
        let client = Client::open("redis://127.0.0.1").unwrap();
	let mut con = client.get_connection().unwrap();
	let username = "Rogger_Admin";
	let rng_str: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
	println!("Master user credentials:\nusername: {}\npassword: {}\nDO NOT SHARE IT WITH ANYONE!",username,rng_str);
        let mut hasher = Sha512::new();
        hasher.update(rng_str);
	let master_hash = hasher.finalize();
	let _: () = con.hset("users", username, format!("{:x}", master_hash)).unwrap();
	let _: () = con.del("keys").unwrap();
     }

     pub fn validate(login: String, password: String) -> bool {
         let client = Client::open("redis://127.0.0.1").unwrap();
	 let mut con = client.get_connection().unwrap();
	 let get_master: Option<String> = con.hget("users", login.to_string()).unwrap();
	 match get_master {
	    Some(hash) => {
	       let mut hasher = Sha512::new();
	       hasher.update(password);
	       let password_hash = hasher.finalize();
	       if hash == format!("{:x}", password_hash) {
	          true
	       }
	       else {
	          false
	       }
	    },
	    None => {
	       println!("{} user does not exist", login);
	       false
	    }
	 }
     }

     pub fn new_key() -> String {
     	 let rng_str: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
	 let mut hasher = Sha256::new();
	 hasher.update(rng_str);
	 let key_hash = hasher.finalize();
     	 let client = Client::open("redis://127.0.0.1").unwrap();
	 let mut con = client.get_connection().unwrap();
	 let _: () = con.rpush("keys",format!("{:x}", key_hash)).unwrap();
	 format!("{:x}", key_hash)
     }

     pub fn validate_key(api_key: String) -> bool {
        let client = Client::open("redis://127.0.0.1").unwrap();
	let mut con = client.get_connection().unwrap();
	let options = LposOptions::default();
	let key_index: Option<i32> = con.lpos("keys", api_key, options).unwrap();
     	match key_index {
	   Some(_) => true,
	   None => false,
	}
     }
}
