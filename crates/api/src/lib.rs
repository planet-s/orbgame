/*!
   API crate that provides additional game api and elements for OrbGame.
*/

pub use orbgame_utils::prelude as utils;
pub use orbtk::api::Application as Game;
pub use orbtk::api::*;

pub mod prelude;
pub mod properties;
pub mod render_object;
