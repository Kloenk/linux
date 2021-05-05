
// SPDX-License-Identifier: GPL-2.0

// TODO: crate doc

#![no_std]
#![feature(allocator_api, global_asm)]

use core::ops::Deref;

use kernel::prelude::*;
use kernel::net::prelude::*;

module! {
    type: WireguardRs,
    name: b"wireguard_rs",
    author: b"Rust for Linux Contributors",
    description: b"Rust Wireguard network driver",
    license: b"GPL v2",
    alias_rtnl_link: b"wireguard_rs",
}

struct WireguardRs;

impl KernelModule for WireguardRs {
    fn init() -> KernelResult<Self> {

        Ok(WireguardRs)
    }
}

impl Drop for WireguardRs {
    fn drop(&mut self) {

    }
}