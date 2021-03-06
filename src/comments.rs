use dashmap::DashMap;
use std::sync::Arc;

use swc_common::{
    comments::{Comment, Comments},
    BytePos,
};

type CommentMap = Arc<DashMap<BytePos, Vec<Comment>>>;

/// Multi-threaded implementation of [Comments]
#[derive(Clone, Default)]
pub struct KeepComments {
    leading: CommentMap,
    trailing: CommentMap,
}

impl Comments for KeepComments {
    fn add_leading(&self, pos: BytePos, cmt: Comment) {
        self.leading.entry(pos).or_default().push(cmt);
    }

    fn add_leading_comments(&self, pos: BytePos, comments: Vec<Comment>) {
        self.leading.entry(pos).or_default().extend(comments);
    }

    fn has_leading(&self, pos: BytePos) -> bool {
        self.leading.contains_key(&pos)
    }

    fn move_leading(&self, from: BytePos, to: BytePos) {
        let cmt = self.leading.remove(&from);

        if let Some(cmt) = cmt {
            self.leading.entry(to).or_default().extend(cmt.1);
        }
    }

    fn take_leading(&self, pos: BytePos) -> Option<Vec<Comment>> {
        self.leading.remove(&pos).map(|v| v.1)
    }

    fn add_trailing(&self, pos: BytePos, cmt: Comment) {
        self.trailing.entry(pos).or_default().push(cmt)
    }

    fn add_trailing_comments(&self, pos: BytePos, comments: Vec<Comment>) {
        self.trailing.entry(pos).or_default().extend(comments)
    }

    fn has_trailing(&self, pos: BytePos) -> bool {
        self.trailing.contains_key(&pos)
    }

    fn move_trailing(&self, from: BytePos, to: BytePos) {
        let cmt = self.trailing.remove(&from);

        if let Some(cmt) = cmt {
            self.trailing.entry(to).or_default().extend(cmt.1);
        }
    }

    fn take_trailing(&self, pos: BytePos) -> Option<Vec<Comment>> {
        self.trailing.remove(&pos).map(|v| v.1)
    }
}
