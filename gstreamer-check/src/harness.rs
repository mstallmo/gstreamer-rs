// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gst;
use std::mem;
use std::ptr;
use TestClock;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Harness(Boxed<ffi::GstHarness>);

    match fn {
        copy => |ptr| ffi::gst_harness_copy(mut_override(ptr)),
        free => |ptr| ffi::gst_harness_free(ptr),
    }
}

impl Harness {
    pub fn add_element_full<
        'a,
        'b,
        'c,
        'd,
        P: IsA<gst::Element>,
        Q: Into<Option<&'a gst::StaticPadTemplate>>,
        R: Into<Option<&'b str>>,
        S: Into<Option<&'c gst::StaticPadTemplate>>,
        T: Into<Option<&'d str>>,
    >(
        &mut self,
        element: &P,
        hsrc: Q,
        element_sinkpad_name: R,
        hsink: S,
        element_srcpad_name: T,
    ) {
        let hsrc = hsrc.into();
        let element_sinkpad_name = element_sinkpad_name.into();
        let element_sinkpad_name = element_sinkpad_name.to_glib_none();
        let hsink = hsink.into();
        let element_srcpad_name = element_srcpad_name.into();
        let element_srcpad_name = element_srcpad_name.to_glib_none();
        unsafe {
            ffi::gst_harness_add_element_full(
                self.to_glib_none_mut().0,
                element.to_glib_none().0,
                hsrc.to_glib_none_mut().0,
                element_sinkpad_name.0,
                hsink.to_glib_none_mut().0,
                element_srcpad_name.0,
            );
        }
    }

    pub fn add_element_sink_pad<P: IsA<gst::Pad>>(&mut self, sinkpad: &P) {
        unsafe {
            ffi::gst_harness_add_element_sink_pad(
                self.to_glib_none_mut().0,
                sinkpad.to_glib_none().0,
            );
        }
    }

    pub fn add_element_src_pad<P: IsA<gst::Pad>>(&mut self, srcpad: &P) {
        unsafe {
            ffi::gst_harness_add_element_src_pad(
                self.to_glib_none_mut().0,
                srcpad.to_glib_none().0,
            );
        }
    }

    pub fn add_parse(&mut self, launchline: &str) {
        unsafe {
            ffi::gst_harness_add_parse(self.to_glib_none_mut().0, launchline.to_glib_none().0);
        }
    }

    //pub fn add_probe(&mut self, element_name: &str, pad_name: &str, mask: /*Ignored*/gst::PadProbeType, callback: /*Unknown conversion*//*Unimplemented*/PadProbeCallback, destroy_data: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_harness_add_probe() }
    //}

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn add_propose_allocation_meta<'a, P: Into<Option<&'a gst::Structure>>>(
        &mut self,
        api: glib::types::Type,
        params: P,
    ) {
        let params = params.into();
        let params = params.to_glib_none();
        unsafe {
            ffi::gst_harness_add_propose_allocation_meta(
                self.to_glib_none_mut().0,
                api.to_glib(),
                params.0,
            );
        }
    }

    pub fn add_sink(&mut self, sink_element_name: &str) {
        unsafe {
            ffi::gst_harness_add_sink(
                self.to_glib_none_mut().0,
                sink_element_name.to_glib_none().0,
            );
        }
    }

    pub fn add_sink_harness(&mut self, sink_harness: &mut Harness) {
        unsafe {
            ffi::gst_harness_add_sink_harness(
                self.to_glib_none_mut().0,
                sink_harness.to_glib_full(),
            );
        }
    }

    pub fn add_sink_parse(&mut self, launchline: &str) {
        unsafe {
            ffi::gst_harness_add_sink_parse(self.to_glib_none_mut().0, launchline.to_glib_none().0);
        }
    }

    pub fn add_src(&mut self, src_element_name: &str, has_clock_wait: bool) {
        unsafe {
            ffi::gst_harness_add_src(
                self.to_glib_none_mut().0,
                src_element_name.to_glib_none().0,
                has_clock_wait.to_glib(),
            );
        }
    }

    pub fn add_src_harness(&mut self, src_harness: &mut Harness, has_clock_wait: bool) {
        unsafe {
            ffi::gst_harness_add_src_harness(
                self.to_glib_none_mut().0,
                src_harness.to_glib_full(),
                has_clock_wait.to_glib(),
            );
        }
    }

    pub fn add_src_parse(&mut self, launchline: &str, has_clock_wait: bool) {
        unsafe {
            ffi::gst_harness_add_src_parse(
                self.to_glib_none_mut().0,
                launchline.to_glib_none().0,
                has_clock_wait.to_glib(),
            );
        }
    }

    pub fn buffers_in_queue(&mut self) -> u32 {
        unsafe { ffi::gst_harness_buffers_in_queue(self.to_glib_none_mut().0) }
    }

    pub fn buffers_received(&mut self) -> u32 {
        unsafe { ffi::gst_harness_buffers_received(self.to_glib_none_mut().0) }
    }

    pub fn crank_multiple_clock_waits(&mut self, waits: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_harness_crank_multiple_clock_waits(
                self.to_glib_none_mut().0,
                waits,
            ))
        }
    }

    pub fn crank_single_clock_wait(&mut self) -> bool {
        unsafe {
            from_glib(ffi::gst_harness_crank_single_clock_wait(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn create_buffer(&mut self, size: usize) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(ffi::gst_harness_create_buffer(
                self.to_glib_none_mut().0,
                size,
            ))
        }
    }

    pub fn dump_to_file(&mut self, filename: &str) {
        unsafe {
            ffi::gst_harness_dump_to_file(self.to_glib_none_mut().0, filename.to_glib_none().0);
        }
    }

    pub fn events_in_queue(&mut self) -> u32 {
        unsafe { ffi::gst_harness_events_in_queue(self.to_glib_none_mut().0) }
    }

    pub fn events_received(&mut self) -> u32 {
        unsafe { ffi::gst_harness_events_received(self.to_glib_none_mut().0) }
    }

    pub fn find_element(&mut self, element_name: &str) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::gst_harness_find_element(
                self.to_glib_none_mut().0,
                element_name.to_glib_none().0,
            ))
        }
    }

    //pub fn get(&mut self, element_name: &str, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gst_harness_get() }
    //}

    //pub fn get_allocator(&mut self, allocator: /*Ignored*/gst::Allocator, params: /*Ignored*/gst::AllocationParams) {
    //    unsafe { TODO: call ffi::gst_harness_get_allocator() }
    //}

    pub fn get_last_pushed_timestamp(&mut self) -> gst::ClockTime {
        unsafe { ffi::gst_harness_get_last_pushed_timestamp(self.to_glib_none_mut().0) }
    }

    pub fn get_testclock(&mut self) -> Option<TestClock> {
        unsafe { from_glib_full(ffi::gst_harness_get_testclock(self.to_glib_none_mut().0)) }
    }

    pub fn play(&mut self) {
        unsafe {
            ffi::gst_harness_play(self.to_glib_none_mut().0);
        }
    }

    pub fn pull(&mut self) -> Option<gst::Buffer> {
        unsafe { from_glib_full(ffi::gst_harness_pull(self.to_glib_none_mut().0)) }
    }

    pub fn pull_event(&mut self) -> Option<gst::Event> {
        unsafe { from_glib_full(ffi::gst_harness_pull_event(self.to_glib_none_mut().0)) }
    }

    pub fn pull_upstream_event(&mut self) -> Option<gst::Event> {
        unsafe {
            from_glib_full(ffi::gst_harness_pull_upstream_event(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn push(&mut self, buffer: &mut gst::Buffer) -> gst::FlowReturn {
        unsafe {
            from_glib(ffi::gst_harness_push(
                self.to_glib_none_mut().0,
                buffer.to_glib_full(),
            ))
        }
    }

    pub fn push_and_pull(&mut self, buffer: &mut gst::Buffer) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(ffi::gst_harness_push_and_pull(
                self.to_glib_none_mut().0,
                buffer.to_glib_full(),
            ))
        }
    }

    pub fn push_event(&mut self, event: &mut gst::Event) -> bool {
        unsafe {
            from_glib(ffi::gst_harness_push_event(
                self.to_glib_none_mut().0,
                event.to_glib_none_mut().0,
            ))
        }
    }

    pub fn push_from_src(&mut self) -> gst::FlowReturn {
        unsafe { from_glib(ffi::gst_harness_push_from_src(self.to_glib_none_mut().0)) }
    }

    pub fn push_to_sink(&mut self) -> gst::FlowReturn {
        unsafe { from_glib(ffi::gst_harness_push_to_sink(self.to_glib_none_mut().0)) }
    }

    pub fn push_upstream_event(&mut self, event: &mut gst::Event) -> bool {
        unsafe {
            from_glib(ffi::gst_harness_push_upstream_event(
                self.to_glib_none_mut().0,
                event.to_glib_none_mut().0,
            ))
        }
    }

    pub fn query_latency(&mut self) -> gst::ClockTime {
        unsafe { ffi::gst_harness_query_latency(self.to_glib_none_mut().0) }
    }

    //pub fn set(&mut self, element_name: &str, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gst_harness_set() }
    //}

    pub fn set_blocking_push_mode(&mut self) {
        unsafe {
            ffi::gst_harness_set_blocking_push_mode(self.to_glib_none_mut().0);
        }
    }

    pub fn set_caps(&mut self, in_: &mut gst::Caps, out: &mut gst::Caps) {
        unsafe {
            ffi::gst_harness_set_caps(
                self.to_glib_none_mut().0,
                in_.to_glib_full(),
                out.to_glib_full(),
            );
        }
    }

    pub fn set_caps_str(&mut self, in_: &str, out: &str) {
        unsafe {
            ffi::gst_harness_set_caps_str(
                self.to_glib_none_mut().0,
                in_.to_glib_none().0,
                out.to_glib_none().0,
            );
        }
    }

    pub fn set_drop_buffers(&mut self, drop_buffers: bool) {
        unsafe {
            ffi::gst_harness_set_drop_buffers(self.to_glib_none_mut().0, drop_buffers.to_glib());
        }
    }

    pub fn set_forwarding(&mut self, forwarding: bool) {
        unsafe {
            ffi::gst_harness_set_forwarding(self.to_glib_none_mut().0, forwarding.to_glib());
        }
    }

    //pub fn set_propose_allocator<'a, 'b, P: Into<Option<&'a /*Ignored*/gst::Allocator>>, Q: Into<Option<&'b /*Ignored*/gst::AllocationParams>>>(&mut self, allocator: P, params: Q) {
    //    unsafe { TODO: call ffi::gst_harness_set_propose_allocator() }
    //}

    pub fn set_sink_caps(&mut self, caps: &mut gst::Caps) {
        unsafe {
            ffi::gst_harness_set_sink_caps(self.to_glib_none_mut().0, caps.to_glib_full());
        }
    }

    pub fn set_sink_caps_str(&mut self, str: &str) {
        unsafe {
            ffi::gst_harness_set_sink_caps_str(self.to_glib_none_mut().0, str.to_glib_none().0);
        }
    }

    pub fn set_src_caps(&mut self, caps: &mut gst::Caps) {
        unsafe {
            ffi::gst_harness_set_src_caps(self.to_glib_none_mut().0, caps.to_glib_full());
        }
    }

    pub fn set_src_caps_str(&mut self, str: &str) {
        unsafe {
            ffi::gst_harness_set_src_caps_str(self.to_glib_none_mut().0, str.to_glib_none().0);
        }
    }

    pub fn set_time(&mut self, time: gst::ClockTime) -> bool {
        unsafe { from_glib(ffi::gst_harness_set_time(self.to_glib_none_mut().0, time)) }
    }

    pub fn set_upstream_latency(&mut self, latency: gst::ClockTime) {
        unsafe {
            ffi::gst_harness_set_upstream_latency(self.to_glib_none_mut().0, latency);
        }
    }

    pub fn sink_push_many(&mut self, pushes: i32) -> gst::FlowReturn {
        unsafe {
            from_glib(ffi::gst_harness_sink_push_many(
                self.to_glib_none_mut().0,
                pushes,
            ))
        }
    }

    pub fn src_crank_and_push_many(&mut self, cranks: i32, pushes: i32) -> gst::FlowReturn {
        unsafe {
            from_glib(ffi::gst_harness_src_crank_and_push_many(
                self.to_glib_none_mut().0,
                cranks,
                pushes,
            ))
        }
    }

    pub fn src_push_event(&mut self) -> bool {
        unsafe { from_glib(ffi::gst_harness_src_push_event(self.to_glib_none_mut().0)) }
    }

    //pub fn stress_custom_start<'a, P: Into<Option<&'a /*Ignored*/glib::Func>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&mut self, init: P, callback: /*Unknown conversion*//*Unimplemented*/Func, data: Q, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call ffi::gst_harness_stress_custom_start() }
    //}

    //pub fn stress_property_start_full(&mut self, name: &str, value: /*Ignored*/&glib::Value, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call ffi::gst_harness_stress_property_start_full() }
    //}

    //pub fn stress_push_buffer_start_full(&mut self, caps: &mut gst::Caps, segment: /*Ignored*/&gst::Segment, buf: &mut gst::Buffer, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call ffi::gst_harness_stress_push_buffer_start_full() }
    //}

    //pub fn stress_push_buffer_with_cb_start_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&mut self, caps: &mut gst::Caps, segment: /*Ignored*/&gst::Segment, func: /*Unknown conversion*//*Unimplemented*/HarnessPrepareBufferFunc, data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call ffi::gst_harness_stress_push_buffer_with_cb_start_full() }
    //}

    //pub fn stress_push_event_start_full(&mut self, event: &mut gst::Event, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call ffi::gst_harness_stress_push_event_start_full() }
    //}

    //pub fn stress_push_event_with_cb_start_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&mut self, func: /*Unknown conversion*//*Unimplemented*/HarnessPrepareEventFunc, data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call ffi::gst_harness_stress_push_event_with_cb_start_full() }
    //}

    //pub fn stress_push_upstream_event_start_full(&mut self, event: &mut gst::Event, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call ffi::gst_harness_stress_push_upstream_event_start_full() }
    //}

    //pub fn stress_push_upstream_event_with_cb_start_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&mut self, func: /*Unknown conversion*//*Unimplemented*/HarnessPrepareEventFunc, data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call ffi::gst_harness_stress_push_upstream_event_with_cb_start_full() }
    //}

    //pub fn stress_requestpad_start_full(&mut self, templ: /*Ignored*/&gst::PadTemplate, name: &str, caps: &mut gst::Caps, release: bool, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call ffi::gst_harness_stress_requestpad_start_full() }
    //}

    //pub fn stress_statechange_start_full(&mut self, sleep: libc::c_ulong) -> /*Ignored*/Option<HarnessThread> {
    //    unsafe { TODO: call ffi::gst_harness_stress_statechange_start_full() }
    //}

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn take_all_data(&mut self) -> (u8, usize) {
        unsafe {
            let mut size = mem::uninitialized();
            let ret = ffi::gst_harness_take_all_data(self.to_glib_none_mut().0, &mut size);
            (ret, size)
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn take_all_data_as_buffer(&mut self) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(ffi::gst_harness_take_all_data_as_buffer(
                self.to_glib_none_mut().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn take_all_data_as_bytes(&mut self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_full(ffi::gst_harness_take_all_data_as_bytes(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn teardown(&mut self) {
        unsafe {
            ffi::gst_harness_teardown(self.to_glib_none_mut().0);
        }
    }

    pub fn try_pull(&mut self) -> Option<gst::Buffer> {
        unsafe { from_glib_full(ffi::gst_harness_try_pull(self.to_glib_none_mut().0)) }
    }

    pub fn try_pull_event(&mut self) -> Option<gst::Event> {
        unsafe { from_glib_full(ffi::gst_harness_try_pull_event(self.to_glib_none_mut().0)) }
    }

    pub fn try_pull_upstream_event(&mut self) -> Option<gst::Event> {
        unsafe {
            from_glib_full(ffi::gst_harness_try_pull_upstream_event(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn upstream_events_in_queue(&mut self) -> u32 {
        unsafe { ffi::gst_harness_upstream_events_in_queue(self.to_glib_none_mut().0) }
    }

    pub fn upstream_events_received(&mut self) -> u32 {
        unsafe { ffi::gst_harness_upstream_events_received(self.to_glib_none_mut().0) }
    }

    pub fn use_systemclock(&mut self) {
        unsafe {
            ffi::gst_harness_use_systemclock(self.to_glib_none_mut().0);
        }
    }

    pub fn use_testclock(&mut self) {
        unsafe {
            ffi::gst_harness_use_testclock(self.to_glib_none_mut().0);
        }
    }

    pub fn wait_for_clock_id_waits(&mut self, waits: u32, timeout: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_harness_wait_for_clock_id_waits(
                self.to_glib_none_mut().0,
                waits,
                timeout,
            ))
        }
    }

    pub fn new(element_name: &str) -> Option<Harness> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_harness_new(element_name.to_glib_none().0)) }
    }

    pub fn new_empty() -> Option<Harness> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_harness_new_empty()) }
    }

    pub fn new_full<
        'a,
        'b,
        'c,
        'd,
        P: IsA<gst::Element>,
        Q: Into<Option<&'a gst::StaticPadTemplate>>,
        R: Into<Option<&'b str>>,
        S: Into<Option<&'c gst::StaticPadTemplate>>,
        T: Into<Option<&'d str>>,
    >(
        element: &P,
        hsrc: Q,
        element_sinkpad_name: R,
        hsink: S,
        element_srcpad_name: T,
    ) -> Option<Harness> {
        assert_initialized_main_thread!();
        let hsrc = hsrc.into();
        let element_sinkpad_name = element_sinkpad_name.into();
        let element_sinkpad_name = element_sinkpad_name.to_glib_none();
        let hsink = hsink.into();
        let element_srcpad_name = element_srcpad_name.into();
        let element_srcpad_name = element_srcpad_name.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_harness_new_full(
                element.to_glib_none().0,
                hsrc.to_glib_none_mut().0,
                element_sinkpad_name.0,
                hsink.to_glib_none_mut().0,
                element_srcpad_name.0,
            ))
        }
    }

    pub fn new_parse(launchline: &str) -> Option<Harness> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_harness_new_parse(launchline.to_glib_none().0)) }
    }

    pub fn new_with_element<
        'a,
        'b,
        P: IsA<gst::Element>,
        Q: Into<Option<&'a str>>,
        R: Into<Option<&'b str>>,
    >(
        element: &P,
        element_sinkpad_name: Q,
        element_srcpad_name: R,
    ) -> Option<Harness> {
        assert_initialized_main_thread!();
        let element_sinkpad_name = element_sinkpad_name.into();
        let element_sinkpad_name = element_sinkpad_name.to_glib_none();
        let element_srcpad_name = element_srcpad_name.into();
        let element_srcpad_name = element_srcpad_name.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_harness_new_with_element(
                element.to_glib_none().0,
                element_sinkpad_name.0,
                element_srcpad_name.0,
            ))
        }
    }

    pub fn new_with_padnames<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(
        element_name: &str,
        element_sinkpad_name: P,
        element_srcpad_name: Q,
    ) -> Option<Harness> {
        assert_initialized_main_thread!();
        let element_sinkpad_name = element_sinkpad_name.into();
        let element_sinkpad_name = element_sinkpad_name.to_glib_none();
        let element_srcpad_name = element_srcpad_name.into();
        let element_srcpad_name = element_srcpad_name.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_harness_new_with_padnames(
                element_name.to_glib_none().0,
                element_sinkpad_name.0,
                element_srcpad_name.0,
            ))
        }
    }

    pub fn new_with_templates<
        'a,
        'b,
        P: Into<Option<&'a gst::StaticPadTemplate>>,
        Q: Into<Option<&'b gst::StaticPadTemplate>>,
    >(
        element_name: &str,
        hsrc: P,
        hsink: Q,
    ) -> Option<Harness> {
        assert_initialized_main_thread!();
        let hsrc = hsrc.into();
        let hsink = hsink.into();
        unsafe {
            from_glib_full(ffi::gst_harness_new_with_templates(
                element_name.to_glib_none().0,
                hsrc.to_glib_none_mut().0,
                hsink.to_glib_none_mut().0,
            ))
        }
    }

    //pub fn stress_thread_stop(t: /*Ignored*/&mut HarnessThread) -> u32 {
    //    unsafe { TODO: call ffi::gst_harness_stress_thread_stop() }
    //}
}