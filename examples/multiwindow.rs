#![feature(phase)]
#![feature(tuple_indexing)]

extern crate glutin;

mod support;

#[cfg(not(feature = "window"))]
fn main() { println!("This example requires glutin to be compiled with the `window` feature"); }

#[cfg(feature = "window")]
fn main() {
    let window1 = glutin::Window::new().unwrap();
    let window2 = glutin::Window::new().unwrap();
    let window3 = glutin::Window::new().unwrap();

    spawn(proc() {
        run(window1, (0.0, 1.0, 0.0, 1.0));
    });

    spawn(proc() {
        run(window2, (0.0, 0.0, 1.0, 1.0));
    });

    spawn(proc() {
        run(window3, (1.0, 0.0, 0.0, 1.0));
    });
}

#[cfg(feature = "window")]
fn run(window: glutin::Window, color: (f32, f32, f32, f32)) {
    unsafe { window.make_current() };

    let context = support::load(&window);

    while !window.is_closed() {
        context.draw_frame(color);
        window.swap_buffers();

        window.wait_events().collect::<Vec<glutin::Event>>();
    }
}
