pub struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}
impl Post {
    pub fn new() -> Post {
        Post {
            content: String::new(),
            state: Some(Box::new(Draft::default())),
        }
    }

    pub fn add_text(&mut self, content: &str) {
        self.content.push_str(content)
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().as_ref().content(&self)
    }

    pub fn request_review(&mut self) {
        self.state_change(|state| state.request_review())
    }

    pub fn approve(&mut self) {
        self.state_change(|state| state.approve())
    }

    pub fn reject(&mut self) {
        self.state_change(|state| state.reject())
    }

    fn state_change<F: Fn(Box<dyn State>) -> Box<dyn State>>(&mut self, next_state: F) {
        if let Some(state) = self.state.take() {
            self.state = Some(next_state(state));
        }
    }
}

trait State {
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Reviewing::default())
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
impl Default for Draft {
    fn default() -> Draft {
        Draft {}
    }
}

struct Reviewing {
    approvals: u8,
}
impl State for Reviewing {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.approvals < 1 {
            Box::new(Reviewing {
                approvals: self.approvals + 1,
            })
        } else {
            Box::new(Published::default())
        }
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft::default())
    }
}
impl Default for Reviewing {
    fn default() -> Reviewing {
        Reviewing { approvals: 0 }
    }
}

struct Published {}
impl State for Published {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
impl Default for Published {
    fn default() -> Published {
        Published {}
    }
}
