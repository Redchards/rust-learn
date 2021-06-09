pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().can_edit() {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn can_edit(&self) -> bool {
        false
    }

    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }

    fn to_string(&self) -> &'static str;

}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn to_string(&self) -> &'static str {
        "Draft"
    }

    fn can_edit(&self) -> bool {
        true
    }
}

struct PendingReview {
    current_approval_count: u8,
}

impl PendingReview {
    fn new() -> Self {
        PendingReview {
            current_approval_count: 0,
        }
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        if self.current_approval_count < 1 {
            self.current_approval_count += 1;
            self
        }
        else {
            Box::new(Approved {})
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn to_string(&self) -> &'static str {
        "PendingReview"
    }
}

struct Approved {}

impl State for Approved {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn to_string(&self) -> &'static str {
        "Approved"
    }
}

#[cfg(test)]
mod tests {

use super::*;

#[test]
fn must_get_two_approvals() {
    let mut post = Post::new();

    post.add_text("Hello world!");
    post.request_review();
    post.approve();

    assert_eq!(post.state.as_ref().unwrap().to_string(), "PendingReview");

    post.approve();

    assert_eq!(post.state.as_ref().unwrap().to_string(), "Approved");
}

#[test]
fn can_edit_only_in_draft_mode() {
    let mut post = Post::new();

    post.add_text("Hello world!");
    post.add_text(" And everybody");
    assert_eq!(post.content, String::from("Hello world! And everybody"));

    post.request_review();
    post.add_text("Won't be added");
    assert_eq!(post.content, String::from("Hello world! And everybody"));

    post.approve();
    post.add_text("Won't be added");
    assert_eq!(post.content, String::from("Hello world! And everybody"));
}

#[test]
fn can_edit_only_in_draft_mode2() {
    let mut post = Post::new();

    post.add_text("Hello world!");
    post.add_text(" And everybody");
    assert_eq!(post.content, String::from("Hello world! And everybody"));

    post.request_review();
    post.add_text("Won't be added");
    assert_eq!(post.content, String::from("Hello world! And everybody"));

    post.reject();
    post.add_text(" too!");
    assert_eq!(post.content, String::from("Hello world! And everybody too!"));
}

}