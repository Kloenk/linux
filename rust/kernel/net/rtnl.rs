// SPDX-License-Identifier: GPL-2.0

//! Net Device Operations.
//!
//! C header: [`include/linux/rtnetlink.h`](../../../../include/linux/rtnetlink.h)

use core::ptr;

use crate::bindings;
use crate::error::{Error, KernelResult};

use super::device::{NetDevice, NetDeviceAdapter};

// TODO: inner bool, to allow other unlock mechanism?
#[must_use = "the rtnl unlocks immediately when the guard is unused"]
pub struct RtnlLock {
    _private: (),
}

impl RtnlLock {
    pub fn lock() -> Self {
        // SAFETY: C function without parameters
        unsafe { bindings::rtnl_lock() };

        Self { _private: () }
    }
}

impl Drop for RtnlLock {
    fn drop(&mut self) {
        // SAFETY: C function without parameters
        unsafe { bindings::rtnl_unlock() };
    }
}

pub const RTNL_LINK_OPS_EMPTY: bindings::rtnl_link_ops = bindings::rtnl_link_ops {
    list: bindings::list_head {
        next: ptr::null::<bindings::list_head>() as *mut bindings::list_head,
        prev: ptr::null::<bindings::list_head>() as *mut bindings::list_head,
    },
    kind: ptr::null::<i8>(),
    priv_size: 0,
    setup: None,
    maxtype: 0,
    policy: ptr::null::<bindings::nla_policy>(),
    validate: None,
    newlink: None,
    changelink: None,
    dellink: None,
    get_size: None,
    fill_info: None,
    get_xstats_size: None,
    fill_xstats: None,
    get_num_tx_queues: None,
    get_num_rx_queues: None,
    slave_maxtype: 0,
    slave_policy: ptr::null::<bindings::nla_policy>(),
    slave_changelink: None,
    get_slave_size: None,
    fill_slave_info: None,
    get_link_net: None,
    get_linkxstats_size: None,
    fill_linkxstats: None,
};

#[repr(transparent)]
pub struct RtnlLinkOps(pub bindings::rtnl_link_ops);

unsafe impl Sync for RtnlLinkOps {}

impl RtnlLinkOps {
    pub fn register(&self) -> KernelResult {
        // SAFETY: ptr of self is valid if self is valid
        let ret = unsafe {
            let ptr = self.get_ptr();

            bindings::rtnl_link_register(ptr as *mut bindings::rtnl_link_ops)
        };

        if ret != 0 {
            Err(Error::from_kernel_errno(ret))
        } else {
            Ok(())
        }
    }

    pub unsafe fn get_ptr(&self) -> *const bindings::rtnl_link_ops {
        self as *const _ as *const bindings::rtnl_link_ops
    }

    pub fn unregister(&self) {
        let ptr = self as *const _ as *mut bindings::rtnl_link_ops;

        // SAFETY: ptr is valid if self is valid
        unsafe { bindings::rtnl_link_unregister(ptr) };
    }
}

impl Drop for RtnlLinkOps {
    fn drop(&mut self) {
        crate::pr_info!("dropping rtnl_link_ops");
    }
}

#[repr(transparent)]
pub struct RtnlLinkStats64 {
    ptr: *const bindings::rtnl_link_stats64,
}

impl RtnlLinkStats64 {
    pub fn dev_read<T: NetDeviceAdapter>(&mut self, dev: &NetDevice<T>) {
        let stats = self.deref_int();
        // SAFETY: call to C function
        unsafe {
            bindings::dev_lstats_read(
                dev.get_ptr() as *mut bindings::net_device,
                &stats.tx_packets as *const u64 as *mut u64,
                &stats.tx_bytes as *const u64 as *mut u64,
            );
        }
    }

    /// Constructs a new [`struct rtnl_link_stats64`] wrapper.
    ///
    /// # Safety
    ///
    /// The pointer `ptr` must be non-null and valid for the lifetime of the object.
    pub unsafe fn from_ptr(ptr: *const bindings::rtnl_link_stats64) -> Self {
        // INVARIANTS: the safety contract ensures the type invariant will hold.
        Self { ptr }
    }

    fn deref_int(&self) -> &bindings::rtnl_link_stats64 {
        // SAFETY: self.ptr is valid if self is valid
        unsafe { self.ptr.as_ref() }.unwrap()
    }
}
