//
// Copyright (C) 2019 Signal Messenger, LLC.
// All rights reserved.
//
// SPDX-License-Identifier: GPL-3.0-only
//

//! WebRTC FFI Data Channel Interface.


use std::os::raw::c_char;
use libc::size_t;

use crate::webrtc::peer_connection::RffiDataChannelInterface;
use crate::webrtc::data_channel_observer::RffiDataChannelObserverInterface;

extern {
    pub fn Rust_dataChannelSend(dc_interface: *const RffiDataChannelInterface,
                                buffer: *const u8, len: size_t, binary: bool) -> bool;

    pub fn Rust_registerDataChannelObserver(dc_interface: *const RffiDataChannelInterface,
                                            dc_observer:  *const RffiDataChannelObserverInterface);

    pub fn Rust_unregisterDataChannelObserver(dc_interface: *const RffiDataChannelInterface,
                                              dc_observer:  *const RffiDataChannelObserverInterface);

    pub fn Rust_dataChannelGetLabel(dc_interface: *const RffiDataChannelInterface) -> *const c_char;
}
