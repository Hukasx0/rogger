use sha2::{Sha512, Digest};
use rand::distributions::{Alphanumeric, DistString};
use rusqlite::Connection;
use std::process;
use sesser::Sesser;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

// use crate::sesser::Database;

pub struct User {}

impl User {
     pub fn init_master(con: Connection) -> Sesser {
		let mut sesser_db = Sesser::new();
		let rng_str: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
		let mut hasher = Sha512::new();
        hasher.update(rng_str.clone());
		let master_hash = hasher.finalize();
		let user_db = format!("INSERT OR IGNORE INTO user (id, username, password) VALUES (1, 'Rogger_Admin', '{}');", format!("{:x}", master_hash));
		let changed_rows = con.execute(&user_db, []).unwrap();
		if changed_rows == 1 {
			println!("Master user credentials:\nusername: Rogger_Admin\npassword: {}\nDO NOT SHARE IT WITH ANYONE!",rng_str);
		} 
		sesser_db.create_table("sessions");
		sesser_db.create_table("api_keys");
		sesser_db
     }
    
    pub fn new_master_user(con: Connection, username: &str) -> String {
		let rng_str: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
		let mut hasher = Sha512::new();
        hasher.update(rng_str.clone());
		let master_hash = hasher.finalize();
		let query = format!("UPDATE user SET username = '{}', password = '{}' WHERE id = 1;", username, format!("{:x}", master_hash));
		con.execute(&query, []).unwrap();
		rng_str
    }
	
     pub fn validate(con: Connection, login: String, password: String) -> bool {
		let mut hasher = Sha512::new();
        hasher.update(password);
		let master_hash = hasher.finalize();
		let mut user_db = con.prepare(&format!("SELECT * FROM user WHERE id = 1 AND username = '{}' AND password = '{}'", login, format!("{:x}", master_hash))).unwrap();
		let mut result = user_db.query([]).unwrap();
		if let Some(_) = result.next().unwrap() {
			return true
		} false
     }

     pub fn new_key(mut sesser_db: RwLockWriteGuard<'_, Sesser>) -> String {
		sesser_db.generate_value("api_keys", 604800)
     }

    pub fn validate_key(sesser_db: RwLockReadGuard<'_, Sesser>, api_key: &str, typev: &str) -> bool {
		sesser_db.check_value_exists(typev, api_key)
    }

    pub fn get_keys(sesser_db: RwLockReadGuard<'_, Sesser>) -> Vec<String> {
		sesser_db.list_values("api_keys")
    }

    pub fn rm_key(mut sesser_db: RwLockWriteGuard<'_, Sesser>, api_key: &str) {
		sesser_db.remove_value("api_keys", api_key);
    }

     pub fn new_session(mut sesser_db: RwLockWriteGuard<'_, Sesser>) -> String {
		sesser_db.generate_value("sessions", 600)
     }

    pub fn end_session(mut sesser_db: RwLockWriteGuard<'_, Sesser>, session: &str) {
		sesser_db.remove_value("sessions", session);
    }
}
