mod combination;
mod model;
mod view;
use crate::view::{update, view};

fn main() -> iced::Result {
    iced::application("Boxing Trainer", update, view)
        .window_size((2000.0, 800.0))
        .run()
}
