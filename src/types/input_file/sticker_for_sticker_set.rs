use super::{PngSticker, TgsSticker};
use crate::types::InteriorBorrow;
use is_macro::Is;

/// Represents a sticker that can be added to a sticker set.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Is)]
pub enum StickerForStickerSet<'a> {
    /// A PNG sticker.
    Png(PngSticker<'a>),
    /// A TGS (animated) sticker.
    Tgs(TgsSticker<'a>),
}

impl<'a> From<PngSticker<'a>> for StickerForStickerSet<'a> {
    fn from(sticker: PngSticker<'a>) -> Self {
        Self::Png(sticker)
    }
}

impl<'a> From<TgsSticker<'a>> for StickerForStickerSet<'a> {
    fn from(sticker: TgsSticker<'a>) -> Self {
        Self::Tgs(sticker)
    }
}

impl<'a> InteriorBorrow<'a> for StickerForStickerSet<'a> {
    fn borrow_inside(&'a self) -> Self {
        match self {
            Self::Png(png_sticker) => Self::Png(png_sticker.borrow_inside()),
            Self::Tgs(tgs_sticker) => Self::Tgs(tgs_sticker.borrow_inside()),
        }
    }
}
