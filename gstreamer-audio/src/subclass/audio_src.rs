// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem;

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use gst::subclass::prelude::*;
use gst::LoggableError;
use gst_base::subclass::prelude::*;

use crate::AudioRingBufferSpec;
use crate::AudioSrc;

pub trait AudioSrcImpl: AudioSrcImplExt + BaseSrcImpl {
    fn close(&self, src: &Self::Type) -> Result<(), LoggableError> {
        self.parent_close(src)
    }

    fn delay(&self, src: &Self::Type) -> u32 {
        self.parent_delay(src)
    }

    fn open(&self, src: &Self::Type) -> Result<(), LoggableError> {
        self.parent_open(src)
    }

    fn prepare(
        &self,
        src: &Self::Type,
        spec: &mut AudioRingBufferSpec,
    ) -> Result<(), LoggableError> {
        AudioSrcImplExt::parent_prepare(self, src, spec)
    }

    fn unprepare(&self, src: &Self::Type) -> Result<(), LoggableError> {
        self.parent_unprepare(src)
    }

    fn read(
        &self,
        src: &Self::Type,
        audio_data: &mut [u8],
    ) -> Result<(u32, gst::ClockTime), LoggableError> {
        self.parent_read(src, audio_data)
    }

    fn reset(&self, src: &Self::Type) {
        self.parent_reset(src)
    }
}

pub trait AudioSrcImplExt: ObjectSubclass {
    fn parent_close(&self, src: &Self::Type) -> Result<(), LoggableError>;
    fn parent_delay(&self, src: &Self::Type) -> u32;
    fn parent_open(&self, src: &Self::Type) -> Result<(), LoggableError>;
    fn parent_prepare(
        &self,
        src: &Self::Type,
        spec: &mut AudioRingBufferSpec,
    ) -> Result<(), LoggableError>;
    fn parent_unprepare(&self, src: &Self::Type) -> Result<(), LoggableError>;
    fn parent_read(
        &self,
        src: &Self::Type,
        audio_data: &mut [u8],
    ) -> Result<(u32, gst::ClockTime), LoggableError>;
    fn parent_reset(&self, src: &Self::Type);
}

impl<T: AudioSrcImpl> AudioSrcImplExt for T {
    fn parent_close(&self, src: &Self::Type) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAudioSrcClass;
            let f = match (*parent_class).close {
                None => return Ok(()),
                Some(f) => f,
            };
            gst::result_from_gboolean!(
                f(src.unsafe_cast_ref::<AudioSrc>().to_glib_none().0),
                gst::CAT_RUST,
                "Failed to close element using the parent function"
            )
        }
    }

    fn parent_delay(&self, src: &Self::Type) -> u32 {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAudioSrcClass;
            let f = match (*parent_class).delay {
                Some(f) => f,
                None => return 0,
            };
            f(src.unsafe_cast_ref::<AudioSrc>().to_glib_none().0)
        }
    }

    fn parent_open(&self, src: &Self::Type) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAudioSrcClass;
            let f = match (*parent_class).open {
                Some(f) => f,
                None => return Ok(()),
            };
            gst::result_from_gboolean!(
                f(src.unsafe_cast_ref::<AudioSrc>().to_glib_none().0),
                gst::CAT_RUST,
                "Failed to open element using the parent function"
            )
        }
    }

    fn parent_prepare(
        &self,
        src: &Self::Type,
        spec: &mut AudioRingBufferSpec,
    ) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAudioSrcClass;
            let f = match (*parent_class).prepare {
                Some(f) => f,
                None => return Ok(()),
            };
            gst::result_from_gboolean!(
                f(
                    src.unsafe_cast_ref::<AudioSrc>().to_glib_none().0,
                    &mut spec.0
                ),
                gst::CAT_RUST,
                "Failed to prepare element using the parent function"
            )
        }
    }

    fn parent_unprepare(&self, src: &Self::Type) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAudioSrcClass;
            let f = match (*parent_class).unprepare {
                Some(f) => f,
                None => {
                    return Err(gst::loggable_error!(
                        gst::CAT_RUST,
                        "Unprepare is not implemented!"
                    ))
                }
            };
            gst::result_from_gboolean!(
                f(src.unsafe_cast_ref::<AudioSrc>().to_glib_none().0),
                gst::CAT_RUST,
                "Failed to unprepare element using the parent function"
            )
        }
    }

    fn parent_read(
        &self,
        src: &Self::Type,
        buffer: &mut [u8],
    ) -> Result<(u32, gst::ClockTime), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAudioSrcClass;
            let f = match (*parent_class).read {
                Some(f) => f,
                None => return Ok((0, gst::CLOCK_TIME_NONE)),
            };
            let buffer_ptr = buffer.as_mut_ptr() as *mut _;
            let mut timestamp = mem::MaybeUninit::uninit();
            let ret = f(
                src.unsafe_cast_ref::<AudioSrc>().to_glib_none().0,
                buffer_ptr,
                buffer.len() as u32,
                timestamp.as_mut_ptr(),
            );
            if ret > 0 {
                Ok((ret, from_glib(timestamp.assume_init())))
            } else {
                Err(gst::loggable_error!(
                    gst::CAT_RUST,
                    "Failed to read using the parent function"
                ))
            }
        }
    }

    fn parent_reset(&self, src: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAudioSrcClass;
            if let Some(f) = (*parent_class).reset {
                f(src.unsafe_cast_ref::<AudioSrc>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: AudioSrcImpl> IsSubclassable<T> for AudioSrc
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(klass: &mut glib::Class<Self>) {
        <gst_base::BaseSrc as IsSubclassable<T>>::override_vfuncs(klass);
        let klass = klass.as_mut();
        klass.close = Some(audiosrc_close::<T>);
        klass.delay = Some(audiosrc_delay::<T>);
        klass.open = Some(audiosrc_open::<T>);
        klass.prepare = Some(audiosrc_prepare::<T>);
        klass.unprepare = Some(audiosrc_unprepare::<T>);
        klass.read = Some(audiosrc_read::<T>);
        klass.reset = Some(audiosrc_reset::<T>);
    }
}

unsafe extern "C" fn audiosrc_close<T: AudioSrcImpl>(
    ptr: *mut ffi::GstAudioSrc,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.close(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audiosrc_delay<T: AudioSrcImpl>(ptr: *mut ffi::GstAudioSrc) -> u32
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &instance.panicked(), 0, {
        imp.delay(wrap.unsafe_cast_ref())
    })
}

unsafe extern "C" fn audiosrc_open<T: AudioSrcImpl>(
    ptr: *mut ffi::GstAudioSrc,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.open(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audiosrc_prepare<T: AudioSrcImpl>(
    ptr: *mut ffi::GstAudioSrc,
    spec: *mut ffi::GstAudioRingBufferSpec,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSrc> = from_glib_borrow(ptr);

    let spec = &mut *(spec as *mut AudioRingBufferSpec);

    gst::panic_to_error!(&wrap, &instance.panicked(), false, {
        match AudioSrcImpl::prepare(imp, wrap.unsafe_cast_ref(), spec) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audiosrc_unprepare<T: AudioSrcImpl>(
    ptr: *mut ffi::GstAudioSrc,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.unprepare(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audiosrc_read<T: AudioSrcImpl>(
    ptr: *mut ffi::GstAudioSrc,
    data: glib::ffi::gpointer,
    length: u32,
    timestamp: *mut gst::ffi::GstClockTime,
) -> u32
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSrc> = from_glib_borrow(ptr);
    let data_slice = std::slice::from_raw_parts_mut(data as *mut u8, length as usize);

    gst::panic_to_error!(&wrap, &instance.panicked(), 0, {
        let (res, timestamp_res) = imp
            .read(wrap.unsafe_cast_ref(), data_slice)
            .unwrap_or((0, gst::CLOCK_TIME_NONE));
        *timestamp = timestamp_res.to_glib();

        res
    })
}

unsafe extern "C" fn audiosrc_reset<T: AudioSrcImpl>(ptr: *mut ffi::GstAudioSrc)
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &instance.panicked(), (), {
        imp.reset(wrap.unsafe_cast_ref());
    });
}
