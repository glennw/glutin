#![feature(phase)]
#![feature(tuple_indexing)]

extern crate glutin;

use std::io::stdio::stdin;

mod support;

#[cfg(not(feature = "window"))]
fn main() { println!("This example requires glutin to be compiled with the `window` feature"); }

#[cfg(feature = "window")]
fn main() {
    // enumerating monitors
    let monitor = {
        for (num, monitor) in glutin::get_available_monitors().enumerate() {
            println!("Monitor #{}: {}", num, monitor.get_name());
        }

        print!("Please write the number of the monitor to use: ");
        let num = from_str(stdin().read_line().unwrap().as_slice().trim())
            .expect("Plase enter a number");
        let monitor = glutin::get_available_monitors().nth(num).expect("Please enter a valid ID");

        println!("Using {}", monitor.get_name());

        monitor
    };

    let window = glutin::WindowBuilder::new()
        .with_title("Hello world!".to_string())
        .with_fullscreen(monitor)
        .build()
        .unwrap();

    unsafe { window.make_current() };

    
    let context = support::load(&window);

    while !window.is_closed() {
        context.draw_frame((0.0, 1.0, 0.0, 1.0));
        window.swap_buffers();

        println!("{}", window.wait_events().collect::<Vec<glutin::Event>>());
    }
}
