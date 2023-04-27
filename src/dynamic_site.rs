use std::sync::RwLock;

#[derive(Clone)]
pub struct Page {
    pub content: String,
    pub html_content: String,
}

impl Page {
    fn new(data: &str) -> Self {
	Page { content: data.to_string(), html_content: markdown::to_html(data) }
    }
}

pub struct Pages {
    pub index_page: RwLock<Page>,
    pub aboutme_page: RwLock<Page>,
}

impl Pages {
    pub fn new() -> Self {
	Pages { index_page: RwLock::new(Page::new("# **My blog**\nThis is example description ")), 
            aboutme_page: RwLock::new(Page::new("# About me")), }
    }

    pub fn get_index(&self) -> Page {
	    self.index_page.read().unwrap().clone()
    }

    pub fn get_aboutme(&self) -> Page {
	    self.aboutme_page.read().unwrap().clone()
    }

    pub fn modify_index(&self, content: String) {
	    *self.index_page.write().unwrap() = Page::new(&content);
    }

    pub fn modify_aboutme(&self, content: String) {
	    *self.aboutme_page.write().unwrap() = Page::new(&content);
    }
}

pub struct DynVal {
    pub blog_name: RwLock<String>,
    pub your_name: RwLock<String>,
    pub master_user_login: RwLock<String>,
    pub favicon: RwLock<String>,
}

impl DynVal {
    pub fn new(vs: Vec<String>) -> Self {
	DynVal { blog_name: RwLock::new(vs[0].to_string()), 
             your_name: RwLock::new(vs[1].to_string()),
             master_user_login: RwLock::new(vs[2].to_string()),
             favicon: RwLock::new(vs[3].to_string()), }
    }
}
