// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ClockTime;
use crate::ControlSource;
use glib::object::IsA;
use glib::translate::*;

pub trait ControlSourceExtManual: 'static {
    fn get_value_array(
        &self,
        timestamp: ClockTime,
        interval: ClockTime,
        values: &mut [f64],
    ) -> Result<(), glib::error::BoolError>;
}

impl<O: IsA<ControlSource>> ControlSourceExtManual for O {
    fn get_value_array(
        &self,
        timestamp: ClockTime,
        interval: ClockTime,
        values: &mut [f64],
    ) -> Result<(), glib::error::BoolError> {
        let n_values = values.len() as u32;
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_control_source_get_value_array(
                    self.as_ref().to_glib_none().0,
                    timestamp.to_glib(),
                    interval.to_glib(),
                    n_values,
                    values.to_glib_none().0,
                ),
                "Failed to get value array"
            )
        }
    }
}
