

    use iced::touch;
    use iced::widget::canvas;
    use iced::widget::canvas::event::{self, Event};
    use iced::widget::canvas::{
        Cache, Canvas, Cursor, Frame, Geometry, Path, Text,
    };
    use iced::{
        alignment, mouse, Color, Element, Length, Point, Rectangle, Size,
        Theme, Vector,
    };
    use rustc_hash::{FxHashMap, FxHashSet};
    use std::future::Future;
    use std::ops::RangeInclusive;
    use std::time::{Duration, Instant};



pub struct Column(u16);
pub struct Row(u16);
pub struct Pixel(Column, Row);

pub trait Display{
    fn toggle_pixel(pixel: Pixel);
    fn clear_canvas();
    fn render_canvas();
}

pub struct Display{

}