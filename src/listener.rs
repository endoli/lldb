// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::broadcaster::SBBroadcaster;
use super::debugger::SBDebugger;
use super::event::SBEvent;
use std::ffi::CString;
use sys;

/// Listen for debugger events.
#[derive(Debug)]
pub struct SBListener {
    /// The underlying raw `SBListenerRef`.
    pub raw: sys::SBListenerRef,
}

impl SBListener {
    /// Construct a new `SBListener`.
    pub fn new() -> SBListener {
        SBListener::wrap(unsafe { sys::CreateSBListener() })
    }

    /// Construct a new `SBListener` with name.
    pub fn new_with_name(name: &str) -> SBListener {
        let name = CString::new(name).unwrap();
        SBListener::wrap(unsafe { sys::CreateSBListener2(name.as_ptr()) })
    }

    /// Construct a new `SBListener`.
    pub fn wrap(raw: sys::SBListenerRef) -> SBListener {
        SBListener { raw }
    }

    /// Construct a new `Some(SBListener)` or `None`.
    pub fn maybe_wrap(raw: sys::SBListenerRef) -> Option<SBListener> {
        if unsafe { sys::SBListenerIsValid(raw) } {
            Some(SBListener { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBListener` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBListenerIsValid(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn start_listening_for_event_class(
        &self,
        debugger: &SBDebugger,
        broadcaster_class: &str,
        event_mask: u32,
    ) -> u32 {
        let bc = CString::new(broadcaster_class).unwrap();
        unsafe {
            sys::SBListenerStartListeningForEventClass(
                self.raw,
                debugger.raw,
                bc.as_ptr(),
                event_mask,
            )
        }
    }

    #[allow(missing_docs)]
    pub fn stop_listening_for_event_class(
        &self,
        debugger: &SBDebugger,
        broadcaster_class: &str,
        event_mask: u32,
    ) -> bool {
        let bc = CString::new(broadcaster_class).unwrap();
        unsafe {
            sys::SBListenerStopListeningForEventClass(
                self.raw,
                debugger.raw,
                bc.as_ptr(),
                event_mask,
            )
        }
    }

    #[allow(missing_docs)]
    pub fn start_listening_for_events(&self, broadcaster: &SBBroadcaster, event_mask: u32) -> u32 {
        unsafe { sys::SBListenerStartListeningForEvents(self.raw, broadcaster.raw, event_mask) }
    }

    #[allow(missing_docs)]
    pub fn stop_listening_for_events(&self, broadcaster: &SBBroadcaster, event_mask: u32) -> bool {
        unsafe { sys::SBListenerStopListeningForEvents(self.raw, broadcaster.raw, event_mask) }
    }

    #[allow(missing_docs)]
    pub fn wait_for_event(&self, num_seconds: u32, event: &mut SBEvent) -> bool {
        unsafe { sys::SBListenerWaitForEvent(self.raw, num_seconds, event.raw) }
    }

    #[allow(missing_docs)]
    pub fn wait_for_event_for_broadcaster(
        &self,
        num_seconds: u32,
        broadcaster: &SBBroadcaster,
        event: &mut SBEvent,
    ) -> bool {
        unsafe {
            sys::SBListenerWaitForEventForBroadcaster(
                self.raw,
                num_seconds,
                broadcaster.raw,
                event.raw,
            )
        }
    }

    #[allow(missing_docs)]
    pub fn wait_for_event_for_broadcaster_with_type(
        &self,
        num_seconds: u32,
        broadcaster: &SBBroadcaster,
        event_type_mask: u32,
        event: &mut SBEvent,
    ) -> bool {
        unsafe {
            sys::SBListenerWaitForEventForBroadcasterWithType(
                self.raw,
                num_seconds,
                broadcaster.raw,
                event_type_mask,
                event.raw,
            )
        }
    }

    #[allow(missing_docs)]
    pub fn peek_at_next_event(&self, event: &mut SBEvent) -> bool {
        unsafe { sys::SBListenerPeekAtNextEvent(self.raw, event.raw) }
    }

    #[allow(missing_docs)]
    pub fn peek_at_next_event_for_broadcaster(
        &self,
        broadcaster: &SBBroadcaster,
        event: &mut SBEvent,
    ) -> bool {
        unsafe {
            sys::SBListenerPeekAtNextEventForBroadcaster(self.raw, broadcaster.raw, event.raw)
        }
    }

    #[allow(missing_docs)]
    pub fn peek_at_next_event_for_broadcaster_with_type(
        &self,
        broadcaster: &SBBroadcaster,
        event_type_mask: u32,
        event: &mut SBEvent,
    ) -> bool {
        unsafe {
            sys::SBListenerPeekAtNextEventForBroadcasterWithType(
                self.raw,
                broadcaster.raw,
                event_type_mask,
                event.raw,
            )
        }
    }

    #[allow(missing_docs)]
    pub fn get_next_event(&self, event: &mut SBEvent) -> bool {
        unsafe { sys::SBListenerGetNextEvent(self.raw, event.raw) }
    }

    #[allow(missing_docs)]
    pub fn get_next_event_for_broadcaster(
        &self,
        broadcaster: &SBBroadcaster,
        event: &mut SBEvent,
    ) -> bool {
        unsafe { sys::SBListenerGetNextEventForBroadcaster(self.raw, broadcaster.raw, event.raw) }
    }

    #[allow(missing_docs)]
    pub fn get_next_event_for_broadcaster_with_type(
        &self,
        broadcaster: &SBBroadcaster,
        event_type_mask: u32,
        event: &mut SBEvent,
    ) -> bool {
        unsafe {
            sys::SBListenerGetNextEventForBroadcasterWithType(
                self.raw,
                broadcaster.raw,
                event_type_mask,
                event.raw,
            )
        }
    }

    #[allow(missing_docs)]
    pub fn handle_broadcast_event(&self, event: &SBEvent) -> bool {
        unsafe { sys::SBListenerHandleBroadcastEvent(self.raw, event.raw) }
    }
}

impl Clone for SBListener {
    fn clone(&self) -> SBListener {
        SBListener {
            raw: unsafe { sys::CloneSBListener(self.raw) },
        }
    }
}

impl Default for SBListener {
    fn default() -> SBListener {
        SBListener::new()
    }
}

impl Drop for SBListener {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBListener(self.raw) };
    }
}

unsafe impl Send for SBListener {}
unsafe impl Sync for SBListener {}
