extern crate metl;

use metl::extras::window::{Event, WindowBuilder};

fn main() {
    let window = WindowBuilder::new()
                     .with_dimensions(800, 600)
                     .build()
                     .unwrap();
    let _device = metl::Device::system_default_device().unwrap();

    'mainloop: loop {
        for event in window.poll_events() {
            match event {
                Event::Closed => {
                    break 'mainloop
                }
                _ => {}
            }
        }

        // drawing code here
    }
}
