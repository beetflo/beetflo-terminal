

use conrod::{self, widget, Colorable, Positionable, Widget};
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};
use conrod::glium::glutin::Event;
use std;


use midi::Message;
use layouts::{BasicTopDownLayout};

use std::sync::mpsc::Receiver;
use surface::{Window, Update, UpdateType};

use widgets::{Piano, TrackOverview};
use widgets;

pub struct Canvas {
    recv: Receiver<Update>
}

impl Canvas {
    pub fn new(canvas_update_recv: Receiver<Update>) -> Canvas {
        Canvas {
            recv: canvas_update_recv
        }
    }

    pub fn init(&mut self, width: u32, height: u32) {
        // Build the window.
        let display = Window::build(width, height);

        // construct our `Ui`.
        let mut ui = conrod::UiBuilder::new([width as f64, width as f64]).build();

        // Generate the widget identifiers.
        let mut manager = BasicTopDownLayout::new(&mut ui);

        // Utilise custom type faces and asset loading
        font_setup(&mut ui);

        // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
        // for drawing to the glium `Surface`.
        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

        // The image map describing each of our widget->image mappings (in our case, none).
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        // Poll events from the window.
        let mut last_update = std::time::Instant::now();
        let mut ui_needs_update = true;

        'main: loop {
            // Reset the needs_update flag and time this update.
            ui_needs_update = false;
            last_update = std::time::Instant::now();

            // We don't want to loop any faster than 60 FPS, so wait until it has been at least
            // 16ms since the last yield.
            let sixteen_ms = std::time::Duration::from_millis(16);
            let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }

            // We don't want to loop any faster than 60 FPS, so wait until it has been at least
            // 16ms since the last yield.

            // Collect all pending events.
            let mut events: Vec<Event> = display.poll_events().collect();

            // If there are no events and the `Ui` does not need updating, wait for the next event.
            if events.is_empty() && !ui_needs_update {
                events.extend(display.wait_events().next());
            }

            // Handle all events.
            for event in events {

                // Use the `winit` backend feature to convert the winit event to a conrod one.
                let winit_event = event.clone();

                if let Some(event) = conrod::backend::winit::convert(winit_event, &display) {
                    ui.handle_event(event);
                    ui_needs_update = true;
                }

                match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                    glium::glutin::Event::Closed => {
                        println!("Event::Closed");
                        std::process::exit(1);
                    },

                    a => {
                        debug!("Event:Unknown - {:?}:", a)
                    },
                }
            }

            manager.layout(&mut ui);

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(1.0, 1.0, 1.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }
}

fn font_setup(ui: &mut conrod::Ui) {
    // Add a `Font` to the `Ui`'s `font::Map` from file.
    const FONT_PATH: &'static str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/NotoSans/NotoSans-Regular.ttf");

    ui.fonts.insert_from_file(FONT_PATH).unwrap();
}
