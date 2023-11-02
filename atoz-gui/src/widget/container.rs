use atoz_renderer::{layer::Layer, pipeline::rect::RectInstance};

use super::{Alignment, Constraint, Rect, Widget};
use crate::{context::Context, layout::Padding};

pub struct Horizontal {
    constraint: Constraint,
    alignment: Alignment,
    padding: Padding,
    rect: Option<Rect>,
    children: Vec<Box<dyn Widget>>,
}

impl Horizontal {
    pub fn new(constraint: Constraint, alignment: Alignment) -> Self {
        return Self {
            constraint,
            alignment,
            padding: Padding::default(),
            rect: Some(Rect::default()),
            children: vec![],
        };
    }

    pub fn set_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        return self;
    }

    pub fn set_children(mut self, widgets: Vec<Box<dyn Widget>>) -> Self {
        self.children = widgets;
        return self;
    }
}

impl Widget for Horizontal {
    fn constraint(&self) -> Constraint {
        return self.constraint;
    }

    fn alignment(&self) -> Alignment {
        return self.alignment;
    }

    fn layout(&mut self, rect: Option<Rect>) {
        self.rect = rect;
        if rect.is_none() {
            return;
        }
        let inner_rect = rect.unwrap().subtract_padding(self.padding);
        let mut child_rect: Rect;

        let mut start = inner_rect.left();
        let mut space = inner_rect.width();
        let mut size: usize;

        for widget in &mut self.children {
            if widget.constraint().has_pixel() {
                child_rect = inner_rect;
                size = widget.constraint().calculate(inner_rect.width(), space);
                if size > 0 {
                    match widget.alignment() {
                        Alignment::Start => {
                            child_rect.left = start;
                            child_rect.width = size;
                            start += size;
                            space -= size;
                        }
                        Alignment::End => {
                            child_rect.left = start + space - size;
                            child_rect.width = size;
                            space -= size;
                        }
                    }
                    widget.layout(Some(child_rect));
                } else {
                    widget.layout(None);
                }
            }
        }

        let percent_parent = space;
        for widget in &mut self.children {
            if widget.constraint().has_percent() {
                child_rect = inner_rect;
                size = widget.constraint().calculate(percent_parent, space);
                if size > 0 {
                    match widget.alignment() {
                        Alignment::Start => {
                            child_rect.left = start;
                            child_rect.width = size;
                            start += size;
                            space -= size;
                        }
                        Alignment::End => {
                            child_rect.left = start + space - size;
                            child_rect.width = size;
                            space -= size;
                        }
                    }
                    widget.layout(Some(child_rect));
                } else {
                    widget.layout(None)
                }
            }
        }
    }

    fn render(&self, context: &mut Context) {
        if self.rect.is_none() {
            return;
        }
        let rect = self.rect.unwrap();
        let mut layer = Layer::default();
        layer.push_rect(RectInstance::fill(
            [rect.left() as _, rect.top() as _],
            [rect.width() as _, rect.height() as _],
            [5.0, 5.0, 5.0, 5.0],
            [0.5, 0.6, 0.7, 1.0],
        ));
        context.push_layers(layer);

        self.children
            .iter()
            .for_each(|widget| widget.render(context));
    }
}

pub struct Vertical {
    constraint: Constraint,
    alignment: Alignment,
    padding: Padding,
    rect: Option<Rect>,
    children: Vec<Box<dyn Widget>>,
}

impl Vertical {
    pub fn new(constraint: Constraint, alignment: Alignment) -> Self {
        return Self {
            constraint,
            alignment,
            padding: Padding::default(),
            rect: Some(Rect::default()),
            children: vec![],
        };
    }

    pub fn set_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        return self;
    }

    pub fn set_children(mut self, widgets: Vec<Box<dyn Widget>>) -> Self {
        self.children = widgets;
        return self;
    }
}

impl Widget for Vertical {
    fn constraint(&self) -> Constraint {
        return self.constraint;
    }

    fn alignment(&self) -> Alignment {
        return self.alignment;
    }

    fn layout(&mut self, rect: Option<Rect>) {
        self.rect = rect;
        if self.rect.is_none() {
            return;
        }
        let inner_rect = rect.unwrap().subtract_padding(self.padding);
        let mut child_rect: Rect;

        let mut start = inner_rect.top();
        let mut space = inner_rect.height();
        let mut size: usize;

        for widget in &mut self.children {
            if widget.constraint().has_pixel() {
                child_rect = inner_rect;
                size = widget.constraint().calculate(inner_rect.height(), space);
                if size > 0 {
                    match widget.alignment() {
                        Alignment::Start => {
                            child_rect.top = start;
                            child_rect.height = size;
                            start += size;
                            space -= size;
                        }
                        Alignment::End => {
                            child_rect.top = start + space - size;
                            child_rect.height = size;
                            space -= size;
                        }
                    }
                    widget.layout(Some(child_rect));
                } else {
                    widget.layout(None);
                }
            }
        }

        let percent_parent = space;
        for widget in &mut self.children {
            if widget.constraint().has_percent() {
                child_rect = inner_rect;
                size = widget.constraint().calculate(percent_parent, space);
                if size > 0 {
                    match widget.alignment() {
                        Alignment::Start => {
                            child_rect.top = start;
                            child_rect.height = size;
                            start += size;
                            space -= size;
                        }
                        Alignment::End => {
                            child_rect.top = start + space - size;
                            child_rect.height = size;
                            space -= size;
                        }
                    }
                    widget.layout(Some(child_rect));
                } else {
                    widget.layout(None);
                }
            }
        }
    }

    fn render(&self, context: &mut Context) {
        if self.rect.is_none() {
            return;
        }
        let rect = self.rect.unwrap();
        let mut layer = Layer::default();
        layer.push_rect(RectInstance::fill(
            [rect.left() as _, rect.top() as _],
            [rect.width() as _, rect.height() as _],
            [5.0, 5.0, 5.0, 5.0],
            [0.3, 0.9, 0.2, 1.0],
        ));
        context.push_layers(layer);
    }
}

pub struct VerticalDecoration {}

#[cfg(test)]
mod test {

    use crate::{
        layout::Padding,
        widget::{Alignment, Constraint},
    };

    use super::Horizontal;

    #[test]
    fn container_test() {
        let _ = Horizontal::new(Constraint::pixel(10, 10), Alignment::Start)
            .set_children(vec![])
            .set_padding(Padding::default());
    }
}
