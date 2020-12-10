use serde::Deserialize;

/// Live location struct.
#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
#[non_exhaustive]
pub struct LiveConfiguration {
    /// Time relative to the message sending date, during which the
    /// location can be updated, in seconds. For active live locations only.
    pub live_period: u32,
    /// The direction in which user is moving, in degrees; 1-360. For active live locations only.
    pub heading: Option<u16>,
    /// Maximum distance for proximity alerts about approaching another
    /// chat member, in meters. For sent live locations only
    pub proximity_alert_radius: Option<u32>,
}

/// Represents a [`Location`].
///
/// [`Location`]: https://core.telegram.org/bots/api#location
#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
#[non_exhaustive]
pub struct Location {
    /// The longitude of the location.
    pub longitude: f64,
    /// The latitude of the location.
    pub latitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// The live location configurations.
    #[serde(flatten)]
    pub live_location: Option<LiveConfiguration>,
}
