use crate::types::{message::Text, PhotoSize};

media_message! {
    struct Photo {
        /// The photo.
        photo: Vec<PhotoSize>,
        /// The caption of the photo.
        caption: Text,
        /// The media group's ID.
        media_group_id: Option<i32>,
    } -> EventLoop::photo

    fn new(caption: Text, media_group_id: Option<i32>,) -> Self {
        Self {
            caption: caption,
            media_group_id: media_group_id,
        }
    }
}
