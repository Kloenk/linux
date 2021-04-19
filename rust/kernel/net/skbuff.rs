use core::{ops::Drop, ptr};

use crate::bindings;

/// Wraps the kernel's `struct sk_buff`.
///
/// # Invariants
///
/// The pointer [`SkBuff::ptr`] is non-null and valid.
#[repr(transparent)]
pub struct SkBuff {
    ptr: *const bindings::sk_buff,
}

impl SkBuff {
    #[cfg(CONFIG_NETWORK_PHY_TIMESTAMPING)]
    pub fn clone_tx_timestamp(&mut self) {
        // SAFETY: self.ptr is valid if self is valid
        unsafe {
            bindings::skb_clone_tx_timestamp(self.ptr as *mut bindings::sk_buff);
        }
    }

    #[cfg(not(CONFIG_NETWORK_PHY_TIMESTAMPING))]
    pub fn clone_tx_timestamp(&mut self) {
        // NOOP
    }

    /// tx_timestamp - Driver hook for transmit timestamping
    ///
    /// Ethernet MAC Drivers should call this function in their hard_xmit()
    /// function immediately before giving the sk_buff to the MAC hardware.
    ///
    /// Specifically, one should make absolutely sure that this function is
    /// called before TX completion of this packet can trigger.  Otherwise
    /// the packet could potentially already be freed.
    pub fn tx_timestamp(&mut self) {
        self.clone_tx_timestamp();
        if self.shinfo().tx_flags() as u32 & bindings::SKBTX_SW_TSTAMP != 0 {
            unsafe {
                bindings::skb_tstamp_tx(self.ptr as *mut bindings::sk_buff, ptr::null_mut());
            }
            // skb_tstamp_tx(skb, NULL);
        }
    }

    pub fn len(&self) -> u32 {
        let skb = self.deref_int();
        skb.len
    }

    /// Constructs a new [`struct sk_buff`] wrapper.
    ///
    /// # Safety
    ///
    /// The pointer `ptr` must be non-null and valid for the lifetime of the object.
    pub unsafe fn from_ptr(ptr: *const bindings::sk_buff) -> Self {
        // INVARIANTS: the safety contract ensures the type invariant will hold.
        Self { ptr }
    }

    fn deref_int(&self) -> &bindings::sk_buff {
        // SAFETY: self.ptr is valid if self is valid
        unsafe { self.ptr.as_ref() }.unwrap()
    }

    pub fn shinfo(&self) -> SkbSharedInfo {
        // SAFETY: self.ptr is valid if self is valid
        unsafe {
            let info = self.shinfo_int();
            SkbSharedInfo::from_ptr(info)
        }
    }

    unsafe fn shinfo_int(&self) -> *mut bindings::skb_shared_info {
        self.end_pointer() as *mut bindings::skb_shared_info
    }

    // NET_SKBUFF_DATA_USES_OFFSET
    #[cfg(target_pointer_width = "64")]
    fn end_pointer(&self) -> *mut u8 {
        let sk_reff = self.deref_int();
        (sk_reff.head as usize + sk_reff.end as usize) as *mut u8
    }

    // !NET_SKBUFF_DATA_USES_OFFSET
    #[cfg(not(target_pointer_width = "64"))]
    fn end_pointer(&self) -> *mut u8 {
        let sk_reff = self.deref_int();
        (sk_reff.end) as *mut u8
    }
}

impl Drop for SkBuff {
    #[cfg(CONFIG_TRACEPOINTS)]
    fn drop(&mut self) {
        // SAFETY: self.ptr is valid if self is valid
        unsafe {
            bindings::consume_skb(self.ptr as *mut bindings::sk_buff);
        }
    }

    #[cfg(not(CONFIG_TRACEPOINTS))]
    fn drop(&mut self) {
        // SAFETY: self.ptr is valid if self is valid
        unsafe {
            bindings::kfree_skb(self.ptr as *mut bindings::sk_buff);
        }
    }
}

/// Wraps the kernel's `struct skb_shared_info`.
///
/// # Invariants
///
/// The pointer [`SkbSharedInfo::ptr`] is non-null and valid.
#[repr(transparent)]
pub struct SkbSharedInfo {
    ptr: *const bindings::skb_shared_info,
}

impl SkbSharedInfo {
    pub fn tx_flags(&self) -> u8 {
        let ref_skb = self.deref_int();
        ref_skb.tx_flags
    }

    /// Constructs a new [`struct skb_shared_info`] wrapper.
    ///
    /// # Safety
    ///
    /// The pointer `ptr` must be non-null and valid for the lifetime of the object.
    pub unsafe fn from_ptr(ptr: *const bindings::skb_shared_info) -> Self {
        // INVARIANTS: the safety contract ensures the type invariant will hold.
        Self { ptr }
    }

    fn deref_int(&self) -> &bindings::skb_shared_info {
        // SAFETY: self.ptr is valid if self is valid
        unsafe { self.ptr.as_ref() }.unwrap()
    }
}
