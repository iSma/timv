extern crate image;
extern crate termion;
extern crate itertools;

mod timg;
mod spec;
mod pixelize;

pub use timg::{Pixel, Image};
pub use spec::Spec;
pub use pixelize::pixelize;
