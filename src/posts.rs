use serde::{Serialize, Deserialize};
use rusqlite::{Connection, Result};

pub struct Database {}

#[derive(Clone, Serialize, Deserialize)]
pub struct Post {
   pub id: usize,
   pub title: String,
   pub content: String,
   pub html_content: String,
   pub date: String,
}

impl Database {
   pub fn new() -> Result<usize> {
      let con = Connection::open("rogger.db")?;
      Ok(con.execute("
         CREATE TABLE IF NOT EXISTS posts (
	    id INTEGER PRIMARY KEY,
	    title TEXT NOT NULL,
	    content TEXT NOT NULL,
	    html_content TEXT NOT NULL,
	    date DATE NOT NULL
	 )", [], )?)
   }

   pub fn get_list(con: Connection, page: usize) -> Result<Vec<Post>> {
      let mut posts_db = con.prepare("SELECT id, title, content, html_content, date FROM posts ORDER BY id DESC LIMIT 10 OFFSET ?")?;
      let posts_iter = posts_db.query_map([page*10], |row| {
         Ok(Post {
 	    id: row.get(0)?,
	    title: row.get(1)?,
	    content: row.get(2)?,
	    html_content: row.get(3)?,
	    date: row.get(4)?,
	 })
      })?;
      let mut posts = Vec::new();
      for posts_fin in posts_iter {
         posts.push(posts_fin?);
      }
      Ok(posts)
   }

   pub fn get_post(con: Connection, id: usize) -> Result<Option<Post>> {
      let mut posts_db = con.prepare("SELECT id, title, content, html_content, date FROM posts WHERE id = ?")?;
      let mut query = posts_db.query([id])?;
      if let Some(row) = query.next()? {
         Ok(Some(Post {
	    id: row.get(0)?,
	    title: row.get(1)?,
	    content: row.get(2)?,
	    html_content: row.get(3)?,
	    date: row.get(4)?,
	 }))
      } else {
         Ok(None)
      }
   }

   pub fn push_post(con: Connection, title: &str, content_md: &str) -> Result<i64> {
       let mut posts_db = con.prepare("INSERT INTO posts (id, title, content, html_content, date) VALUES (NULL, ?, ?, ?, date('now'))")?;
       posts_db.execute([title, content_md, &markdown::to_html(content_md)]);
       Ok(con.last_insert_rowid())
   }

   pub fn edit_post(con: Connection, id: usize, title: &str, content_md: &str) -> Result<()> {
       let mut posts_db = con.prepare("UPDATE posts SET title = ?, content = ?, html_content = ?, date = date('now') WHERE id = ?")?;
       posts_db.execute([title, content_md, &markdown::to_html(content_md), &id.to_string()])?;
       Ok(())
   }

   pub fn rm_post(con: Connection, id: usize) -> Result<()> {
       let mut posts_db = con.prepare("DELETE FROM posts WHERE id = ?")?;
       posts_db.execute([id])?;
       Ok(())
   }
}
