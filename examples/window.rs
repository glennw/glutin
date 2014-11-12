#![feature(phase)]
#![feature(tuple_indexing)]

extern crate glutin;

mod support;

#[cfg(not(feature = "window"))]
fn main() { println!("This example requires glutin to be compiled with the `window` feature"); }

#[cfg(feature = "window")]
fn main() {
    let window = glutin::Window::new().unwrap();
    window.set_title("A fantastic window!");

    unsafe { window.make_current() };

    let context = support::load(&window);

    while !window.is_closed() {
        context.draw_frame((0.0, 1.0, 0.0, 1.0));
        window.swap_buffers();

        println!("{}", window.wait_events().collect::<Vec<glutin::Event>>());
    }
}
