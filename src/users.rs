use sha2::{Sha256, Sha512, Digest};
use rand::distributions::{Alphanumeric, DistString};
use redis::{Commands, Client, LposOptions, RedisResult};
use std::env;

pub struct User {}

impl User {
     pub fn init_master() {
	let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1"));
	let client = Client::open(redis_url).unwrap();
	let mut con;
	match client.get_connection() {
	    Ok(value) => { println!("Redis connected successfully!"); con = value; }
	    Err(error) => { println!("Cannot connect to Redis because of: {}", error);
	                    std::process::exit(1); }
	}
	let username = "Rogger_Admin";
	let rng_str: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
	println!("Master user credentials:\nusername: {}\npassword: {}\nDO NOT SHARE IT WITH ANYONE!",username,rng_str);
        let mut hasher = Sha512::new();
        hasher.update(rng_str);
	let master_hash = hasher.finalize();
	let _: () = con.hset("users", username, format!("{:x}", master_hash)).unwrap();
	let _: () = con.del("keys").unwrap();
        let _: () = con.del("sessions").unwrap();
     }
    
    pub fn new_master_user(username: &str) -> String {
	let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1"));
	let client = Client::open(redis_url).unwrap();
	let mut con = client.get_connection().unwrap();
	let _: () = con.del("users").unwrap();
	let rng_str: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
	let mut hasher = Sha512::new();
        hasher.update(&rng_str);
	let master_hash = hasher.finalize();
	let _: () = con.hset("users", username, format!("{:x}", master_hash)).unwrap();
	rng_str
    }
	
     pub fn validate(login: String, password: String) -> bool {
	 let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1"));
	 let client = Client::open(redis_url).unwrap();
	 let mut con = client.get_connection().unwrap();
	 let get_master: Option<String> = con.hget("users", login).unwrap();
	 match get_master {
	    Some(hash) => {
	       let mut hasher = Sha512::new();
	       hasher.update(password);
	       let password_hash = hasher.finalize();
	       hash == format!("{:x}", password_hash)
	    },
	    None => {
	       false
	    }
	 }
     }

     pub fn new_key() -> String {
     	 let rng_str: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
	 let mut hasher = Sha256::new();
	 hasher.update(rng_str);
	 let key_hash = hasher.finalize();
	 let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1"));
     	 let client = Client::open(redis_url).unwrap();
	 let mut con = client.get_connection().unwrap();
	 let _: () = con.rpush("keys",format!("{:x}", key_hash)).unwrap();
	 format!("{:x}", key_hash)
     }

    pub fn validate_key(api_key: String, typev: &str) -> bool {
	let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1"));
        let client = Client::open(redis_url).unwrap();
	let mut con = client.get_connection().unwrap();
	let options = LposOptions::default();
	let key_index: Option<i32> = con.lpos(typev, api_key, options).unwrap();
    /* 	match key_index {
	    Some(_) => true,
	    None => false,
	}*/
	key_index.is_some()
    }

    pub fn get_keys() -> Vec<String> {
	let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1"));
     	let client = Client::open(redis_url).unwrap();
	let mut con = client.get_connection().unwrap();
	con.lrange("keys", 0, -1).unwrap()
    }

    pub fn rm_key(api_key: String) {
	let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1"));
     	let client = Client::open(redis_url).unwrap();
	let mut con = client.get_connection().unwrap();
	let _: () = con.lrem("keys", 1, api_key).unwrap();	
    }

     pub fn new_session() -> String {
         let rng_str: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 48);
	 let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1"));
     	 let client = Client::open(redis_url).unwrap();
	 let mut con = client.get_connection().unwrap();
	 let _: () = con.rpush("sessions", rng_str.to_string()).unwrap();
	 let _: () = con.expire(rng_str.to_string(), 600).unwrap();
	 rng_str   
     }

    pub fn end_session(session: String) -> RedisResult<()> {
	let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1"));
     	let client = Client::open(redis_url).unwrap();
	let mut con = client.get_connection().unwrap();
	con.del(session)?;
	Ok(())
    }
}
