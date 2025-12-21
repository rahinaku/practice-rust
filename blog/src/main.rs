extern crate blog;
use blog::{ApproveResult, Post};
fn main() {
    let mut post = Post::new();

    post.add_text("text");

    let post = post.request_review();

    let mut post = post.approve();

    loop {
        match post {
            ApproveResult::PendingReviewPost(pending_review_post) => {
                post = pending_review_post.approve();
            }
            ApproveResult::Post(approve_post) => {
                println!("{}", approve_post.content());
                break;
            }
        }
    }
}
