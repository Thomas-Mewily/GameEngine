pub use super::*;

pub mod zero_sized_number;
pub use zero_sized_number::*;

#[macro_use]
pub mod map_on;
//pub use map_on::*;

pub mod integer;
pub use integer::*;

/// Module related to float (currently f32)
pub mod floating;
pub use floating::*;

/// The general interface for 2D, 3D and 4D coordinates
pub mod coordinate;
pub use coordinate::*;

/// 2D coordinates
pub mod coordinate2;
pub use coordinate2::*;

/// 3D coordinates
pub mod coordinate3;
pub use coordinate3::*;

/// 4D coordinates
pub mod coordinate4;
pub use coordinate4::*;

/// General interface for number
pub mod number;
pub use number::*;

/// A `N` dimension rectangle
pub mod rectangle;
pub use rectangle::*;

pub mod circle;
pub use circle::*;

/// 2D Angle
pub mod angle;
pub use angle::*;

/// 2D Angle
pub mod typedef;
pub use typedef::*;

/// Moving
pub mod move_by;
pub use move_by::*;