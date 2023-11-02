use crate::{context::Context, widget::Widget};

pub struct Window {
    context: Context,
    children: Vec<Box<dyn Widget>>,
}

impl Window {
    pub fn new(context: Context) -> Self {
        return Self {
            context,
            children: vec![],
        };
    }

    pub fn set_children(mut self, widgets: Vec<Box<dyn Widget>>) -> Self {
        self.children = widgets;
        return self;
    }
}
