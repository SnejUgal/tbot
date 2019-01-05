use super::*;

/// Represents a media that can be sent in a group (aka albums).
pub enum GroupMedia<'a> {
    /// A group's photo.
    Photo(Photo<'a>),
    /// A group's video.
    Video(Video<'a>),
}

impl<'a> From<Photo<'a>> for GroupMedia<'a> {
    fn from(photo: Photo<'a>) -> Self {
        GroupMedia::Photo(photo)
    }
}

impl<'a> From<Video<'a>> for GroupMedia<'a> {
    fn from(video: Video<'a>) -> Self {
        GroupMedia::Video(video)
    }
}

impl<'a> serde::Serialize for GroupMedia<'a> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            GroupMedia::Photo(photo) => photo.serialize(s),
            GroupMedia::Video(video) => video.serialize(s),
        }
    }
}