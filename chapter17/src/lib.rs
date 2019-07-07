pub struct Post {
    content: String,
    status: Option<Box<dyn State>>,
}
impl Post {
    pub fn new() -> Post {
        Post {
            content: String::new(),
            status: Some(Box::new(Draft {})),
        }
    }

    pub fn add_text(&mut self, content: &str) {
        self.content.push_str(content)
    }

    pub fn content(&self) -> &str {
        self.status.as_ref().unwrap().as_ref().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(status) = self.status.take() {
            self.status = Some(status.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(status) = self.status.take() {
            self.status = Some(status.approve());
        }
    }
}

trait State {
    fn content<'a>(&self, post: &'a Post) -> &'a str;

    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}
impl State for Draft {
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}
impl State for Published {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        post.content.as_str()
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
