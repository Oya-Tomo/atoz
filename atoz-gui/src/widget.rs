use crate::{
    context::Context,
    layout::{Alignment, Constraint, Rect},
};

pub mod container;

pub trait Widget {
    fn constraint(&self) -> Constraint;
    fn alignment(&self) -> Alignment;
    fn layout(&mut self, rect: Option<Rect>);
    fn render(&self, context: &mut Context);
}

pub enum Focus {
    Focused,
    ChildFocused,
    None,
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        return Self { r, g, b, a };
    }

    pub fn to_float(&self) -> [f32; 4] {
        return [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ];
    }
}
