// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::WebRTCDTLSTransport;
use glib::translate::*;

glib::wrapper! {
    pub struct WebRTCRTPSender(Object<ffi::GstWebRTCRTPSender, ffi::GstWebRTCRTPSenderClass>);

    match fn {
        get_type => || ffi::gst_webrtc_rtp_sender_get_type(),
    }
}

impl WebRTCRTPSender {
    #[doc(alias = "gst_webrtc_rtp_sender_new")]
    pub fn new() -> WebRTCRTPSender {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gst_webrtc_rtp_sender_new()) }
    }

    #[doc(alias = "gst_webrtc_rtp_sender_set_rtcp_transport")]
    pub fn set_rtcp_transport(&self, transport: &WebRTCDTLSTransport) {
        unsafe {
            ffi::gst_webrtc_rtp_sender_set_rtcp_transport(
                self.to_glib_none().0,
                transport.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_webrtc_rtp_sender_set_transport")]
    pub fn set_transport(&self, transport: &WebRTCDTLSTransport) {
        unsafe {
            ffi::gst_webrtc_rtp_sender_set_transport(
                self.to_glib_none().0,
                transport.to_glib_none().0,
            );
        }
    }
}

impl Default for WebRTCRTPSender {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for WebRTCRTPSender {}
unsafe impl Sync for WebRTCRTPSender {}
