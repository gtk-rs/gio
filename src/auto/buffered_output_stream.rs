// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use FilterOutputStream;
use OutputStream;
use Seekable;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct BufferedOutputStream(Object<ffi::GBufferedOutputStream, ffi::GBufferedOutputStreamClass>): FilterOutputStream, OutputStream, Seekable;

    match fn {
        get_type => || ffi::g_buffered_output_stream_get_type(),
    }
}

impl BufferedOutputStream {
    pub fn new<P: IsA<OutputStream>>(base_stream: &P) -> BufferedOutputStream {
        unsafe {
            OutputStream::from_glib_full(ffi::g_buffered_output_stream_new(base_stream.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_sized<P: IsA<OutputStream>>(base_stream: &P, size: usize) -> BufferedOutputStream {
        unsafe {
            OutputStream::from_glib_full(ffi::g_buffered_output_stream_new_sized(base_stream.to_glib_none().0, size)).downcast_unchecked()
        }
    }
}

pub trait BufferedOutputStreamExt: 'static {
    fn get_auto_grow(&self) -> bool;

    fn get_buffer_size(&self) -> usize;

    fn set_auto_grow(&self, auto_grow: bool);

    fn set_buffer_size(&self, size: usize);

    fn connect_property_auto_grow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_buffer_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<BufferedOutputStream>> BufferedOutputStreamExt for O {
    fn get_auto_grow(&self) -> bool {
        unsafe {
            from_glib(ffi::g_buffered_output_stream_get_auto_grow(self.to_glib_none().0))
        }
    }

    fn get_buffer_size(&self) -> usize {
        unsafe {
            ffi::g_buffered_output_stream_get_buffer_size(self.to_glib_none().0)
        }
    }

    fn set_auto_grow(&self, auto_grow: bool) {
        unsafe {
            ffi::g_buffered_output_stream_set_auto_grow(self.to_glib_none().0, auto_grow.to_glib());
        }
    }

    fn set_buffer_size(&self, size: usize) {
        unsafe {
            ffi::g_buffered_output_stream_set_buffer_size(self.to_glib_none().0, size);
        }
    }

    fn connect_property_auto_grow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::auto-grow\0".as_ptr() as *const _,
                transmute(notify_auto_grow_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_buffer_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::buffer-size\0".as_ptr() as *const _,
                transmute(notify_buffer_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_auto_grow_trampoline<P>(this: *mut ffi::GBufferedOutputStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BufferedOutputStream> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&BufferedOutputStream::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_buffer_size_trampoline<P>(this: *mut ffi::GBufferedOutputStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BufferedOutputStream> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&BufferedOutputStream::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for BufferedOutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BufferedOutputStream")
    }
}
