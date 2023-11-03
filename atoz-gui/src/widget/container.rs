use atoz_renderer::{layer::Layer, pipeline::rect::RectInstance};

use super::{Alignment, Color, Constraint, Rect, Widget};
use crate::{context::Context, layout::Padding};

pub struct Horizontal {
    constraint: Constraint,
    alignment: Alignment,
    padding: Padding,
    decoration: HorizontalDecoration,
    rect: Option<Rect>,
    children: Vec<Box<dyn Widget>>,
}

impl Horizontal {
    pub fn new(constraint: Constraint, alignment: Alignment) -> Self {
        return Self {
            constraint,
            alignment,
            padding: Padding::default(),
            decoration: HorizontalDecoration::default(),
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

    pub fn set_decoration(mut self, decoration: HorizontalDecoration) -> Self {
        self.decoration = decoration;
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
            self.decoration.border_radius,
            self.decoration.background_color,
        ));
        context.push_layers(layer);

        self.children
            .iter()
            .for_each(|widget| widget.render(context));
    }
}

pub struct HorizontalDecoration {
    border_radius: [f32; 4],
    background_color: [f32; 4],
}

impl HorizontalDecoration {
    pub fn set_border_radius(mut self, lt: f32, lb: f32, rb: f32, rt: f32) -> Self {
        self.border_radius[0] = lt;
        self.border_radius[1] = lb;
        self.border_radius[2] = rb;
        self.border_radius[3] = rt;
        return self;
    }

    pub fn set_background_color(mut self, color: Color) -> Self {
        self.background_color = color.to_float();
        return self;
    }
}

impl Default for HorizontalDecoration {
    fn default() -> Self {
        Self {
            border_radius: [0.0, 0.0, 0.0, 0.0],
            background_color: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

pub struct Vertical {
    constraint: Constraint,
    alignment: Alignment,
    padding: Padding,
    decoration: VerticalDecoration,
    rect: Option<Rect>,
    children: Vec<Box<dyn Widget>>,
}

impl Vertical {
    pub fn new(constraint: Constraint, alignment: Alignment) -> Self {
        return Self {
            constraint,
            alignment,
            padding: Padding::default(),
            decoration: VerticalDecoration::default(),
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

    pub fn set_decoration(mut self, decoration: VerticalDecoration) -> Self {
        self.decoration = decoration;
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
            self.decoration.border_radius,
            self.decoration.background_color,
        ));
        context.push_layers(layer);

        self.children
            .iter()
            .for_each(|widget| widget.render(context));
    }
}

pub struct VerticalDecoration {
    border_radius: [f32; 4],
    background_color: [f32; 4],
}

impl VerticalDecoration {
    pub fn set_border_radius(mut self, lt: f32, lb: f32, rb: f32, rt: f32) -> Self {
        self.border_radius[0] = lt;
        self.border_radius[1] = lb;
        self.border_radius[2] = rb;
        self.border_radius[3] = rt;
        return self;
    }

    pub fn set_background_color(mut self, color: Color) -> Self {
        self.background_color = color.to_float();
        return self;
    }
}

impl Default for VerticalDecoration {
    fn default() -> Self {
        Self {
            border_radius: [0.0, 0.0, 0.0, 0.0],
            background_color: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

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
