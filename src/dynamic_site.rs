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
    pub pages: RwLock<Vec<Page>>,
}

impl Pages {
    pub fn new() -> Self {
	Pages { pages: RwLock::new(vec![Page::new("# **My blog**\nThis is example description "), Page::new("# About me")]) }
    }

    pub fn get_site(&self, id: usize) -> Page {
	self.pages.read().unwrap()[id].clone()
    }

    pub fn modify_site(&self, id: usize, content: String) {
	self.pages.write().unwrap()[id] = Page::new(&content);
    }
}
