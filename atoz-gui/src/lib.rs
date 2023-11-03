pub mod context;
pub mod layout;
pub mod widget;
pub mod window;

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoopBuilder},
        platform::wayland::EventLoopBuilderExtWayland,
    };

    use crate::{
        context::Context,
        layout::{Alignment, Constraint},
        widget::{
            container::{Horizontal, HorizontalDecoration, Vertical, VerticalDecoration},
            Color,
        },
        window::{Window, WindowDecoration},
    };

    #[test]
    fn container_layout_test() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        env_logger::init();

        let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
        let context = rt.block_on(Context::init(&event_loop));
        let mut window = Window::new(
            context,
            Box::new(
                Vertical::new(Constraint::percent(100, 1000000, 0), Alignment::Start)
                    .set_decoration(
                        VerticalDecoration::default()
                            .set_background_color(Color::new(30, 30, 30, 255))
                            .set_border_radius(5.0, 5.0, 5.0, 5.0),
                    )
                    .set_children(vec![
                        Box::new(
                            Horizontal::new(Constraint::pixel(32, 0), Alignment::Start)
                                .set_decoration(
                                    HorizontalDecoration::default()
                                        .set_background_color(Color::new(40, 40, 40, 255)),
                                ),
                        ),
                        Box::new(
                            Horizontal::new(
                                Constraint::percent(100, 10000000, 0),
                                Alignment::Start,
                            )
                            .set_decoration(
                                HorizontalDecoration::default()
                                    .set_background_color(Color::new(70, 70, 70, 255)),
                            )
                            .set_children(vec![Box::new(
                                Vertical::new(Constraint::percent(30, 400, 200), Alignment::Start)
                                    .set_decoration(
                                        VerticalDecoration::default()
                                            .set_background_color(Color::new(30, 30, 30, 255))
                                            .set_border_radius(5.0, 5.0, 5.0, 5.0),
                                    ),
                            )]),
                        ),
                    ]),
            ),
            Some(
                WindowDecoration::default()
                    .set_size((800, 600))
                    .set_window_size_limit((10000, 2000), (500, 400)),
            ),
        );

        let frame_rate = Duration::from_secs_f64(1.0 / 30.0);
        let mut ticker = Instant::now();
        event_loop.run(move |event, _, control_frow| match event {
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::CloseRequested => *control_frow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    window.resize(size);
                }
                WindowEvent::ScaleFactorChanged {
                    scale_factor: _,
                    new_inner_size,
                } => {
                    window.resize(*new_inner_size);
                }
                _ => {}
            },
            Event::RedrawRequested(windos_id) => {
                if windos_id == window.get_window_id() {
                    window.render();
                }
            }
            Event::MainEventsCleared => {
                if frame_rate <= ticker.elapsed() {
                    window.render_request();
                    ticker = Instant::now();
                } else {
                    *control_frow = ControlFlow::WaitUntil(
                        Instant::now().checked_sub(ticker.elapsed()).unwrap() + frame_rate,
                    );
                }
            }
            _ => (),
        });
    }
}
