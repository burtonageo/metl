extern crate metl;

use metl::extras::window::WindowBuilder;
use metl::extras::events::Event;

fn main() {
    let window = WindowBuilder::new().build().unwrap();
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
