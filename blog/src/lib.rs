pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    pub fn get_state(&self) -> &'static str {
        "Approved"
    }
}

pub struct DraftPost {
    content: String,
}
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            count: 0,
        }
    }

    pub fn get_state(&self) -> &'static str {
        "Draft"
    }
}

pub enum ApproveResult {
    PendingReviewPost(PendingReviewPost),
    Post(Post),
}

pub struct PendingReviewPost {
    content: String,
    count: u32,
}

impl PendingReviewPost {
    pub fn approve(self) -> ApproveResult {
        let count = self.count + 1;
        println!("approve count {}", count);
        if count >= 2 {
            ApproveResult::Post(Post {
                content: self.content,
            })
        } else {
            ApproveResult::PendingReviewPost(PendingReviewPost {
                content: self.content,
                count: count,
            })
        }
    }

    pub fn get_state(&self) -> &'static str {
        "PendingReview"
    }
}
