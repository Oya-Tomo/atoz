pub mod context;
pub mod layout;
pub mod widget;
pub mod window;

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use winit::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoopBuilder},
        platform::wayland::EventLoopBuilderExtWayland,
    };

    use crate::{
        context::Context,
        layout::{Alignment, Constraint, Padding, Rect},
        widget::{
            container::{Horizontal, Vertical},
            Widget,
        },
    };

    #[test]
    fn container_layout_test() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        env_logger::init();

        let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
        let mut context = rt.block_on(Context::init(&event_loop, LogicalSize::new(1000, 800)));

        let mut cnt = Horizontal::new(Constraint::percent(100, 1000000000, 0), Alignment::Start)
            .set_padding(Padding {
                left: Constraint::pixel(10, 0),
                right: Constraint::pixel(10, 0),
                top: Constraint::pixel(20, 0),
                bottom: Constraint::pixel(20, 0),
            })
            .set_children(vec![
                Box::new(Vertical::new(Constraint::pixel(150, 100), Alignment::Start)),
                Box::new(Vertical::new(Constraint::pixel(150, 100), Alignment::Start)),
            ]);

        let frame_rate = Duration::from_secs_f64(1.0 / 30.0);
        let mut ticker = Instant::now();

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => context.resize(physical_size),
                WindowEvent::ScaleFactorChanged {
                    scale_factor: _,
                    new_inner_size,
                } => context.resize(*new_inner_size),
                _ => {}
            },
            Event::RedrawRequested(window_id) => {
                if window_id == context.get_window_id() {
                    cnt.layout(Some(Rect::new(
                        0,
                        0,
                        context.get_config().width as _,
                        context.get_config().height as _,
                    )));
                    context.clear_layers();
                    cnt.render(&mut context);
                    context.render();
                }
            }
            Event::MainEventsCleared => {
                if frame_rate <= ticker.elapsed() {
                    context.request_redraw();
                    println!("{}", ticker.elapsed().as_nanos());
                    ticker = Instant::now();
                } else {
                    *control_flow = ControlFlow::WaitUntil(
                        Instant::now().checked_sub(ticker.elapsed()).unwrap() + frame_rate,
                    );
                }
            }
            _ => {}
        });
    }
}
