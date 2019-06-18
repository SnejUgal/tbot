edited_message! {
    struct EditedPhoto {
        /// The photo.
        photo: Vec<types::PhotoSize>,
        /// The caption of the photo.
        caption: types::message::Text,
        /// The media group's ID.
        media_group_id: Option<i32>,
    } -> EventLoop::edited_photo

    fn new(caption: types::message::Text, media_group_id: Option<i32>,) -> Self {
        Self {
            caption: caption,
            media_group_id: media_group_id,
        }
    }
}
