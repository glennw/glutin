#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_uppercase_statics)]

use libc;

pub mod egl {
    pub type khronos_utime_nanoseconds_t = super::khronos_utime_nanoseconds_t;
    pub type khronos_uint64_t = super::khronos_uint64_t;
    pub type khronos_ssize_t = super::khronos_ssize_t;
    pub type EGLNativeDisplayType = super::EGLNativeDisplayType;
    pub type EGLNativePixmapType = super::EGLNativePixmapType;
    pub type EGLNativeWindowType = super::EGLNativeWindowType;
    pub type EGLint = super::EGLint;
    pub type NativeDisplayType = super::EGLNativeDisplayType;
    pub type NativePixmapType = super::EGLNativePixmapType;
    pub type NativeWindowType = super::EGLNativeWindowType;

    generate_gl_bindings! {
        api: "egl",
        profile: "core",
        version: "1.5",
        generator: "static"
    }
}

pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
pub type khronos_uint64_t = libc::uint64_t;
pub type khronos_ssize_t = libc::c_long;
pub type EGLint = libc::int32_t;
pub type EGLNativeDisplayType = *const libc::c_void;
pub type EGLNativePixmapType = *const libc::c_void;     // FIXME: egl_native_pixmap_t instead
pub type EGLNativeWindowType = *const ANativeWindow;

#[link(name = "android")]
#[link(name = "EGL")]
#[link(name = "GLESv2")]
extern {}

pub type ANativeWindow = ();
pub type Display = *const libc::c_void;

#[link(name = "eglflame")]
extern {
    pub fn display_create(egl_native_window: *mut EGLNativeWindowType) -> Display;
    pub fn display_swap_buffers(d: Display, dpy: egl::types::EGLDisplay, sur: egl::types::EGLSurface);
}

