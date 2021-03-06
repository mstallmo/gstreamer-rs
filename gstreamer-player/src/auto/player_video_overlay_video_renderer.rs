// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::PlayerVideoRenderer;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    pub struct PlayerVideoOverlayVideoRenderer(Object<ffi::GstPlayerVideoOverlayVideoRenderer, ffi::GstPlayerVideoOverlayVideoRendererClass>) @implements PlayerVideoRenderer;

    match fn {
        get_type => || ffi::gst_player_video_overlay_video_renderer_get_type(),
    }
}

impl PlayerVideoOverlayVideoRenderer {
    #[doc(alias = "gst_player_video_overlay_video_renderer_expose")]
    pub fn expose(&self) {
        unsafe {
            ffi::gst_player_video_overlay_video_renderer_expose(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_player_video_overlay_video_renderer_get_render_rectangle")]
    pub fn get_render_rectangle(&self) -> (i32, i32, i32, i32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::gst_player_video_overlay_video_renderer_get_render_rectangle(
                self.to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            let x = x.assume_init();
            let y = y.assume_init();
            let width = width.assume_init();
            let height = height.assume_init();
            (x, y, width, height)
        }
    }

    //#[doc(alias = "gst_player_video_overlay_video_renderer_get_window_handle")]
    //pub fn get_window_handle(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:gst_player_video_overlay_video_renderer_get_window_handle() }
    //}

    #[doc(alias = "gst_player_video_overlay_video_renderer_set_render_rectangle")]
    pub fn set_render_rectangle(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            ffi::gst_player_video_overlay_video_renderer_set_render_rectangle(
                self.to_glib_none().0,
                x,
                y,
                width,
                height,
            );
        }
    }

    //#[doc(alias = "gst_player_video_overlay_video_renderer_set_window_handle")]
    //pub fn set_window_handle(&self, window_handle: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:gst_player_video_overlay_video_renderer_set_window_handle() }
    //}

    pub fn get_property_video_sink(&self) -> Option<gst::Element> {
        unsafe {
            let mut value = glib::Value::from_type(<gst::Element as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"video-sink\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `video-sink` getter")
        }
    }

    pub fn set_property_video_sink<P: IsA<gst::Element>>(&self, video_sink: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"video-sink\0".as_ptr() as *const _,
                glib::Value::from(video_sink).to_glib_none().0,
            );
        }
    }

    //#[doc(alias = "gst_player_video_overlay_video_renderer_new")]
    //pub fn new(window_handle: /*Unimplemented*/Option<Fundamental: Pointer>) -> Option<PlayerVideoRenderer> {
    //    unsafe { TODO: call ffi:gst_player_video_overlay_video_renderer_new() }
    //}

    //#[doc(alias = "gst_player_video_overlay_video_renderer_new_with_sink")]
    //pub fn new_with_sink<P: IsA<gst::Element>>(window_handle: /*Unimplemented*/Option<Fundamental: Pointer>, video_sink: &P) -> Option<PlayerVideoRenderer> {
    //    unsafe { TODO: call ffi:gst_player_video_overlay_video_renderer_new_with_sink() }
    //}

    pub fn connect_property_video_sink_notify<
        F: Fn(&PlayerVideoOverlayVideoRenderer) + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_video_sink_trampoline<
            F: Fn(&PlayerVideoOverlayVideoRenderer) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstPlayerVideoOverlayVideoRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::video-sink\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_video_sink_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_window_handle_notify<
        F: Fn(&PlayerVideoOverlayVideoRenderer) + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_window_handle_trampoline<
            F: Fn(&PlayerVideoOverlayVideoRenderer) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstPlayerVideoOverlayVideoRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::window-handle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_window_handle_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for PlayerVideoOverlayVideoRenderer {}
unsafe impl Sync for PlayerVideoOverlayVideoRenderer {}
