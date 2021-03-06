// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    pub struct VideoOverlay(Interface<ffi::GstVideoOverlay>);

    match fn {
        get_type => || ffi::gst_video_overlay_get_type(),
    }
}

impl VideoOverlay {
    //#[cfg(any(feature = "v1_14", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    //#[doc(alias = "gst_video_overlay_install_properties")]
    //pub fn install_properties(oclass: /*Ignored*/&mut glib::ObjectClass, last_prop_id: i32) {
    //    unsafe { TODO: call ffi:gst_video_overlay_install_properties() }
    //}

    //#[cfg(any(feature = "v1_14", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    //#[doc(alias = "gst_video_overlay_set_property")]
    //pub fn set_property<P: IsA<glib::Object>>(object: &P, last_prop_id: i32, property_id: u32, value: /*Ignored*/&glib::Value) -> bool {
    //    unsafe { TODO: call ffi:gst_video_overlay_set_property() }
    //}
}

unsafe impl Send for VideoOverlay {}
unsafe impl Sync for VideoOverlay {}

pub const NONE_VIDEO_OVERLAY: Option<&VideoOverlay> = None;

pub trait VideoOverlayExt: 'static {
    #[doc(alias = "gst_video_overlay_expose")]
    fn expose(&self);

    //#[doc(alias = "gst_video_overlay_got_window_handle")]
    //fn got_window_handle(&self, handle: /*Unimplemented*/Fundamental: UIntPtr);

    #[doc(alias = "gst_video_overlay_handle_events")]
    fn handle_events(&self, handle_events: bool);

    #[doc(alias = "gst_video_overlay_prepare_window_handle")]
    fn prepare_window_handle(&self);

    #[doc(alias = "gst_video_overlay_set_render_rectangle")]
    fn set_render_rectangle(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), glib::error::BoolError>;

    //#[doc(alias = "gst_video_overlay_set_window_handle")]
    //fn set_window_handle(&self, handle: /*Unimplemented*/Fundamental: UIntPtr);
}

impl<O: IsA<VideoOverlay>> VideoOverlayExt for O {
    fn expose(&self) {
        unsafe {
            ffi::gst_video_overlay_expose(self.as_ref().to_glib_none().0);
        }
    }

    //fn got_window_handle(&self, handle: /*Unimplemented*/Fundamental: UIntPtr) {
    //    unsafe { TODO: call ffi:gst_video_overlay_got_window_handle() }
    //}

    fn handle_events(&self, handle_events: bool) {
        unsafe {
            ffi::gst_video_overlay_handle_events(
                self.as_ref().to_glib_none().0,
                handle_events.to_glib(),
            );
        }
    }

    fn prepare_window_handle(&self) {
        unsafe {
            ffi::gst_video_overlay_prepare_window_handle(self.as_ref().to_glib_none().0);
        }
    }

    fn set_render_rectangle(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_video_overlay_set_render_rectangle(
                    self.as_ref().to_glib_none().0,
                    x,
                    y,
                    width,
                    height
                ),
                "Failed to set render rectangle"
            )
        }
    }

    //fn set_window_handle(&self, handle: /*Unimplemented*/Fundamental: UIntPtr) {
    //    unsafe { TODO: call ffi:gst_video_overlay_set_window_handle() }
    //}
}
