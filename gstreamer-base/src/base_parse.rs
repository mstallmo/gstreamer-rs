// Take a look at the license at the top of the repository in the LICENSE file.

use crate::BaseParse;
use crate::BaseParseFrame;
use glib::object::IsA;
use glib::translate::*;
use gst::FormattedValue;
use std::convert::TryFrom;
use std::mem;

pub trait BaseParseExtManual: 'static {
    fn get_sink_pad(&self) -> gst::Pad;
    fn get_src_pad(&self) -> gst::Pad;

    fn set_duration<V: Into<gst::GenericFormattedValue>>(&self, duration: V, interval: u32);
    fn set_frame_rate(&self, fps: gst::Fraction, lead_in: u32, lead_out: u32);

    fn convert_default<V: Into<gst::GenericFormattedValue>, U: gst::SpecificFormattedValue>(
        &self,
        src_val: V,
    ) -> Option<U>;
    fn convert_default_generic<V: Into<gst::GenericFormattedValue>>(
        &self,
        src_val: V,
        dest_format: gst::Format,
    ) -> Option<gst::GenericFormattedValue>;

    fn finish_frame(
        &self,
        frame: BaseParseFrame,
        size: u32,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;
}

impl<O: IsA<BaseParse>> BaseParseExtManual for O {
    fn get_sink_pad(&self) -> gst::Pad {
        unsafe {
            let elt: &ffi::GstBaseParse = &*(self.as_ptr() as *const _);
            from_glib_none(elt.sinkpad)
        }
    }

    fn get_src_pad(&self) -> gst::Pad {
        unsafe {
            let elt: &ffi::GstBaseParse = &*(self.as_ptr() as *const _);
            from_glib_none(elt.srcpad)
        }
    }

    fn set_duration<V: Into<gst::GenericFormattedValue>>(&self, duration: V, interval: u32) {
        let duration = duration.into();
        unsafe {
            ffi::gst_base_parse_set_duration(
                self.as_ref().to_glib_none().0,
                duration.get_format().to_glib(),
                duration.get_value(),
                interval as i32,
            );
        }
    }

    fn set_frame_rate(&self, fps: gst::Fraction, lead_in: u32, lead_out: u32) {
        let (fps_num, fps_den) = fps.into();
        unsafe {
            ffi::gst_base_parse_set_frame_rate(
                self.as_ref().to_glib_none().0,
                fps_num as u32,
                fps_den as u32,
                lead_in,
                lead_out,
            );
        }
    }

    fn convert_default<V: Into<gst::GenericFormattedValue>, U: gst::SpecificFormattedValue>(
        &self,
        src_val: V,
    ) -> Option<U> {
        let src_val = src_val.into();
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_base_parse_convert_default(
                self.as_ref().to_glib_none().0,
                src_val.get_format().to_glib(),
                src_val.to_raw_value(),
                U::get_default_format().to_glib(),
                dest_val.as_mut_ptr(),
            ));
            if ret {
                Some(U::from_raw(U::get_default_format(), dest_val.assume_init()))
            } else {
                None
            }
        }
    }

    fn convert_default_generic<V: Into<gst::GenericFormattedValue>>(
        &self,
        src_val: V,
        dest_format: gst::Format,
    ) -> Option<gst::GenericFormattedValue> {
        let src_val = src_val.into();
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_base_parse_convert_default(
                self.as_ref().to_glib_none().0,
                src_val.get_format().to_glib(),
                src_val.to_raw_value(),
                dest_format.to_glib(),
                dest_val.as_mut_ptr(),
            ));
            if ret {
                Some(gst::GenericFormattedValue::new(
                    dest_format,
                    dest_val.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    fn finish_frame(
        &self,
        frame: BaseParseFrame,
        size: u32,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            gst::FlowReturn::from_glib(ffi::gst_base_parse_finish_frame(
                self.as_ref().to_glib_none().0,
                frame.to_glib_none().0,
                i32::try_from(size).expect("size higher than i32::MAX"),
            ))
            .into_result()
        }
    }
}
