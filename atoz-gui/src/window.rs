use winit::{dpi::PhysicalSize, window::WindowId};

use crate::{context::Context, layout::Rect, widget::Widget};

pub struct Window {
    context: Context,
    child: Box<dyn Widget>,
}

impl Window {
    pub fn new(
        mut context: Context,
        child: Box<dyn Widget>,
        decoration: Option<WindowDecoration>,
    ) -> Self {
        if let Some(decoration) = decoration {
            // TODO : add hide frame
            context.resize(PhysicalSize::new(decoration.size.0, decoration.size.1));
            context.set_min_size(PhysicalSize::new(
                decoration.min_size.0,
                decoration.min_size.1,
            ));
            context.set_max_size(PhysicalSize::new(
                decoration.max_size.0,
                decoration.max_size.1,
            ));
        }
        return Self { context, child };
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.context.resize(size);
    }

    pub fn get_window_id(&self) -> WindowId {
        return self.context.get_window_id();
    }

    pub fn render_request(&self) {
        self.context.request_redraw();
    }

    pub fn render(&mut self) {
        let width = self.context.get_config().width;
        let height = self.context.get_config().height;

        self.context.clear_layers();
        self.child
            .layout(Some(Rect::new(0, 0, width as _, height as _)));
        self.child.render(&mut self.context);
        self.context.render();
    }
}

pub struct WindowDecoration {
    pub frame: bool,
    pub size: (u32, u32),
    pub min_size: (u32, u32),
    pub max_size: (u32, u32),
}

impl WindowDecoration {
    pub fn set_frame(mut self, frame: bool) -> Self {
        self.frame = frame;
        return self;
    }

    pub fn set_size(mut self, size: (u32, u32)) -> Self {
        self.size = size;
        return self;
    }

    pub fn set_window_size_limit(mut self, max: (u32, u32), min: (u32, u32)) -> Self {
        self.min_size = min;
        self.max_size = max;
        return self;
    }
}

impl Default for WindowDecoration {
    fn default() -> Self {
        return Self {
            frame: true,
            size: (800, 600),
            min_size: (200, 150),
            max_size: (100000, 50000),
        };
    }
}
