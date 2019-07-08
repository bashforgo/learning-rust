use chapter17 as blog;

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());


    let mut post = Post::new();

    post.add_text("I ate bacon for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());
    post.add_text("; After rejection");
    assert_eq!("", post.content());

    post.request_review();
    post.add_text("[Cannot add content in review]");
    assert_eq!("", post.content());

    post.approve();
    post.approve();
    assert_eq!(
        "I ate bacon for lunch today; After rejection",
        post.content()
    );
}
