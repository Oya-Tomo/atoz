pub mod context;
pub mod layout;
pub mod widget;
pub mod window;

#[cfg(test)]
mod tests {
    use std::{
        process::exit,
        time::{Duration, Instant},
    };

    use winit::{
        error::EventLoopError,
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
    fn container_layout_test() -> Result<(), EventLoopError> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        env_logger::init();

        let event_loop = EventLoopBuilder::new()
            .with_any_thread(true)
            .build()
            .unwrap();
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
                                        .set_background_color(Color::new(7, 7, 7, 255)),
                                ),
                        ),
                        Box::new(
                            Horizontal::new(Constraint::percent(100, 1000000, 0), Alignment::Start)
                                .set_decoration(
                                    HorizontalDecoration::default()
                                        .set_background_color(Color::new(10, 10, 10, 255)),
                                )
                                .set_children(vec![
                                    Box::new(
                                        Vertical::new(
                                            Constraint::percent(30, 400, 200),
                                            Alignment::Start,
                                        )
                                        .set_decoration(
                                            VerticalDecoration::default()
                                                .set_background_color(Color::new(5, 5, 5, 255)),
                                        )
                                        .set_children(
                                            vec![
                                                Box::new(
                                                    Horizontal::new(
                                                        Constraint::pixel(20, 10),
                                                        Alignment::Start,
                                                    )
                                                    .set_decoration(
                                                        HorizontalDecoration::default()
                                                            .set_background_color(Color::new(
                                                                4, 4, 4, 255,
                                                            )),
                                                    ),
                                                ),
                                                Box::new(
                                                    Horizontal::new(
                                                        Constraint::pixel(20, 10),
                                                        Alignment::Start,
                                                    )
                                                    .set_decoration(
                                                        HorizontalDecoration::default()
                                                            .set_background_color(Color::new(
                                                                6, 6, 6, 255,
                                                            )),
                                                    ),
                                                ),
                                                Box::new(
                                                    Horizontal::new(
                                                        Constraint::pixel(20, 10),
                                                        Alignment::Start,
                                                    )
                                                    .set_decoration(
                                                        HorizontalDecoration::default()
                                                            .set_background_color(Color::new(
                                                                4, 4, 4, 255,
                                                            )),
                                                    ),
                                                ),
                                                Box::new(
                                                    Horizontal::new(
                                                        Constraint::pixel(20, 10),
                                                        Alignment::Start,
                                                    )
                                                    .set_decoration(
                                                        HorizontalDecoration::default()
                                                            .set_background_color(Color::new(
                                                                6, 6, 6, 255,
                                                            )),
                                                    ),
                                                ),
                                                Box::new(
                                                    Horizontal::new(
                                                        Constraint::pixel(20, 10),
                                                        Alignment::Start,
                                                    )
                                                    .set_decoration(
                                                        HorizontalDecoration::default()
                                                            .set_background_color(Color::new(
                                                                4, 4, 4, 255,
                                                            )),
                                                    ),
                                                ),
                                                Box::new(
                                                    Horizontal::new(
                                                        Constraint::pixel(20, 10),
                                                        Alignment::Start,
                                                    )
                                                    .set_decoration(
                                                        HorizontalDecoration::default()
                                                            .set_background_color(Color::new(
                                                                6, 6, 6, 255,
                                                            )),
                                                    ),
                                                ),
                                            ],
                                        ),
                                    ),
                                    Box::new(
                                        Vertical::new(
                                            Constraint::percent(70, 400, 100),
                                            Alignment::Start,
                                        )
                                        .set_decoration(
                                            VerticalDecoration::default()
                                                .set_border_radius(10.0, 20.0, 30.0, 40.0)
                                                .set_background_color(Color::new(10, 20, 40, 50)),
                                        ),
                                    ),
                                    Box::new(
                                        Vertical::new(Constraint::pixel(200, 100), Alignment::End)
                                            .set_decoration(
                                                VerticalDecoration::default().set_background_color(
                                                    Color::new(40, 10, 20, 80),
                                                ),
                                            ),
                                    ),
                                ]),
                        ),
                    ]),
            ),
            Some(
                WindowDecoration::default()
                    .set_size((800, 600))
                    .set_window_size_limit((10000, 2000), (500, 400)),
            ),
        );

        let frame_rate = Duration::from_secs_f64(1.0 / 60.0);
        let mut ticker = Instant::now();

        return event_loop.run(move |event, elwt| match event {
            Event::WindowEvent { window_id, event } if window_id == window.get_window_id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    WindowEvent::Resized(physical_size) => {
                        window.resize(physical_size);
                    }
                    WindowEvent::RedrawRequested => {
                        let i = Instant::now();
                        window.render();
                        println!("{}", i.elapsed().as_millis());
                    }
                    _ => {}
                }
            }
            Event::AboutToWait => {
                if frame_rate <= ticker.elapsed() {
                    window.render_request();
                    println!("ticker : {}", ticker.elapsed().as_micros());
                    ticker = Instant::now();
                } else {
                    elwt.set_control_flow(ControlFlow::WaitUntil(
                        Instant::now().checked_sub(ticker.elapsed()).unwrap() + frame_rate,
                    ));
                }
            }
            _ => (),
        });
    }
}
