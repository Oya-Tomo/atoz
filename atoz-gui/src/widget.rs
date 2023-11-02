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

pub trait Decoration {}

pub enum Focus {
    Focused,
    ChildFocused,
    None,
}
