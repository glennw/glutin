extern crate native;

use libc;
use {CreationError, OsError, Event, WindowBuilder};

pub struct Window {
    native_display: ffi::Display,
    display: ffi::egl::types::EGLDisplay,
    context: ffi::egl::types::EGLContext,
    surface: ffi::egl::types::EGLSurface,
}

pub struct MonitorID;

mod ffi;

compile_warning!("The Android implementation is not fully working yet")

pub fn get_available_monitors() -> Vec<MonitorID> {
    vec![ MonitorID ]
}

pub fn get_primary_monitor() -> MonitorID {
    MonitorID
}

impl MonitorID {
    pub fn get_name(&self) -> Option<String> {
        Some("Primary".to_string())
    }

    pub fn get_dimensions(&self) -> (uint, uint) {
        unimplemented!()
    }
}

impl Window {
    pub fn new(_builder: WindowBuilder) -> Result<Window, CreationError> {
        use std::{mem, ptr};

        let mut native_window = ptr::null();
        let native_display = unsafe { ffi::display_create(&mut native_window) };

        let display = unsafe {
            let display = ffi::egl::GetDisplay(mem::transmute(ffi::egl::DEFAULT_DISPLAY));
            if display.is_null() {
                return Err(OsError("No EGL display connection available".to_string()));
            }
            display
        };

        let (_major, _minor) = unsafe {
            let mut major: ffi::egl::types::EGLint = mem::uninitialized();
            let mut minor: ffi::egl::types::EGLint = mem::uninitialized();

            if ffi::egl::Initialize(display, &mut major, &mut minor) == 0 {
                return Err(OsError(format!("eglInitialize failed")))
            }

            (major, minor)
        };

        let config = unsafe {
            let attribute_list = [
                ffi::egl::RED_SIZE as i32, 8,
                ffi::egl::GREEN_SIZE as i32, 8,
                ffi::egl::BLUE_SIZE as i32, 8,
                ffi::egl::ALPHA_SIZE as i32, 8,
                ffi::egl::SURFACE_TYPE as i32, ffi::egl::WINDOW_BIT as i32,
                ffi::egl::NONE as i32
            ];

            let mut num_config: ffi::egl::types::EGLint = mem::uninitialized();
            let mut config: ffi::egl::types::EGLConfig = mem::uninitialized();
            if ffi::egl::ChooseConfig(display, attribute_list.as_ptr(), &mut config, 1,
                &mut num_config) == 0
            {
                return Err(OsError(format!("eglChooseConfig failed")))
            }

            if num_config <= 0 {
                return Err(OsError(format!("eglChooseConfig returned no available config")))
            }

            config
        };

        let context = unsafe {
            let context = ffi::egl::CreateContext(display, config, ptr::null(), ptr::null());
            if context.is_null() {
                return Err(OsError(format!("eglCreateContext failed")))
            }
            context
        };

        let surface = unsafe {
            let surface = ffi::egl::CreateWindowSurface(display, config, native_window, ptr::null());
            if surface.is_null() {
                return Err(OsError(format!("eglCreateWindowSurface failed")))
            }
            surface
        };

        Ok(Window {
            native_display: native_display,
            display: display,
            context: context,
            surface: surface,
        })
    }

    pub fn is_closed(&self) -> bool {
        false
    }

    pub fn set_title(&self, _: &str) {
    }

    pub fn show(&self) {
    }

    pub fn hide(&self) {
    }

    pub fn get_position(&self) -> Option<(int, int)> {
        None
    }

    pub fn set_position(&self, _x: int, _y: int) {
    }

    pub fn get_inner_size(&self) -> Option<(uint, uint)> {
        panic!("todo");

    }

    pub fn get_outer_size(&self) -> Option<(uint, uint)> {
        self.get_inner_size()
    }

    pub fn set_inner_size(&self, _x: uint, _y: uint) {
    }

    pub fn poll_events(&self) -> Vec<Event> {
        use std::time::Duration;
        use std::io::timer;
        timer::sleep(Duration::milliseconds(16));
        Vec::new()
    }

    pub fn wait_events(&self) -> Vec<Event> {
        use std::time::Duration;
        use std::io::timer;
        timer::sleep(Duration::milliseconds(16));
        Vec::new()
    }

    pub fn make_current(&self) {
        unsafe {
            ffi::egl::MakeCurrent(self.display, self.surface, self.surface, self.context);
        }
    }

    pub fn get_proc_address(&self, addr: &str) -> *const () {
        use std::c_str::ToCStr;

        unsafe {
            addr.with_c_str(|s| {
                ffi::egl::GetProcAddress(s) as *const ()
            })
        }
    }

    pub fn swap_buffers(&self) {
        unsafe {
            ffi::display_swap_buffers(self.native_display, self.display, self.surface);
        }
    }

    pub fn platform_display(&self) -> *mut libc::c_void {
        self.display as *mut libc::c_void
    }
}

#[unsafe_destructor]
impl Drop for Window {
    fn drop(&mut self) {
        use std::ptr;

        unsafe {
            ffi::egl::MakeCurrent(self.display, ptr::null(), ptr::null(), ptr::null());
            ffi::egl::DestroySurface(self.display, self.surface);
            ffi::egl::DestroyContext(self.display, self.context);
            ffi::egl::Terminate(self.display);
        }
    }
}
