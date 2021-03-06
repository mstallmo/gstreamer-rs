// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DiscovererVideoInfo;

use glib::translate::*;

impl DiscovererVideoInfo {
    pub fn get_framerate(&self) -> gst::Fraction {
        unsafe {
            gst::Fraction::new(
                ffi::gst_discoverer_video_info_get_framerate_num(self.to_glib_none().0) as i32,
                ffi::gst_discoverer_video_info_get_framerate_denom(self.to_glib_none().0) as i32,
            )
        }
    }

    pub fn get_par(&self) -> gst::Fraction {
        unsafe {
            gst::Fraction::new(
                ffi::gst_discoverer_video_info_get_par_num(self.to_glib_none().0) as i32,
                ffi::gst_discoverer_video_info_get_par_denom(self.to_glib_none().0) as i32,
            )
        }
    }
}
