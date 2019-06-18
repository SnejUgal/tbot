use crate::types::{self, message::Text};

media_message! {
    struct Video {
        /// The video.
        video: types::Video,
        /// The caption of the video.
        caption: Text,
        /// The media group's ID.
        media_group_id: Option<i32>,
    } -> EventLoop::video

    fn new(caption: Text, media_group_id: Option<i32>,) -> Self {
        Self {
            caption: caption,
            media_group_id: media_group_id,
        }
    }
}
