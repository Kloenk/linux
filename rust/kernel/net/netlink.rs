// SPDX-License-Identifier: GPL-2.0
use crate::bindings;

pub const ETH_ALEN: u16 = bindings::ETH_ALEN as u16;

const NLA_HDRLEN: i32 = bindings::BINDINGS_NLA_HDRLEN;
const __IFLA_MAX: usize = bindings::__IFLA_MAX as usize;

#[repr(transparent)]
pub struct NlAttr(*const bindings::nlattr);

impl NlAttr {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn nla_len(&self) -> u16 {
        if self.is_null() {
            return 0;
        }

        // NO-PANIC: self is valid and not null
        // SAFETY: ptr is valid if self is valid
        let nlattr = unsafe { self.0.as_ref() }.unwrap();
        nlattr.nla_len - NLA_HDRLEN as u16
    }

    /// Constructs a new [`struct nlattr`] wrapper.
    ///
    /// # Safety
    ///
    /// The pointer `ptr` must be non-null and valid for the lifetime of the object.
    pub unsafe fn from_ptr(ptr: *const bindings::nlattr) -> Self {
        Self(ptr)
    }
    /*pub unsafe fn from_ptr(ptr: *const bindings::nlattr) -> &'static mut Self {
        (ptr as *mut NlAttr).as_mut().unwrap()
    }*/

    pub unsafe fn data(&self) -> *const i8 {
        ((self.0 as usize) + NLA_HDRLEN as usize) as *const i8
    }

    pub fn is_valid_ether_addr(&self) -> bool {
        // SAFETY: self.o is valid if self is valid
        unsafe {
            let data = self.data() as *const u8;
            super::is_valid_ether_addr(data)
        }
    }

    unsafe fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[repr(transparent)]
pub struct NlExtAck(*const bindings::netlink_ext_ack);

impl NlExtAck {
    /// Constructs a new [`struct netlink_ext_ack`] wrapper.
    ///
    /// # Safety
    ///
    /// The pointer `ptr` must be non-null and valid for the lifetime of the object.
    pub unsafe fn from_ptr(ptr: *const bindings::netlink_ext_ack) -> Self {
        Self(ptr)
    }
}

#[repr(transparent)]
pub struct NlAttrVec {
    ptr: *const *const bindings::nlattr,
}

impl NlAttrVec {
    pub fn get(&self, offset: u32) -> Option<NlAttr> {
        if offset > __IFLA_MAX as u32 {
            return None;
        }

        let vec = unsafe { &*(self.ptr as *const [NlAttr; __IFLA_MAX]) };
        let nlattr = &vec[offset as usize];
        if nlattr.is_null() {
            None
        } else {
            Some(unsafe { nlattr.clone() })
        }
    }

    pub unsafe fn from_ptr(ptr: *const *const bindings::nlattr) -> Self {
        /*let vec = *(ptr as *const [NlAttr; __IFLA_MAX]);
        Self(vec)*/
        Self { ptr }
    }
}

/*pub struct NlAttrVec<'a>(&'a mut [NlAttr]);

impl<'a> NlAttrVec<'a> {
    pub fn get(&self, offset: u32) -> Option<NlAttr> {
        if offset > bindings::__IFLA_MAX {
            return None;
        }

        let nlattr = &self.0[offset as usize];
        if nlattr.is_null() {
            None
        } else {
            Some(unsafe { nlattr.clone() })
        }
    }

    /// Constructs a new [`struct nlattr[]`] wrapper.
    ///
    /// # Safety
    ///
    /// The pointer `ptr` must be non-null and valid for the lifetime of the object.
    /// The pointer `ptr` must be valid for the size of `__IFLA_MAX` * `mem::size_of<NlAttr>`
    pub unsafe fn from_ptr(ptr: *const *const bindings::nlattr) -> Self {
        // TODO: is this correct?
        Self(core::slice::from_raw_parts_mut(ptr as *mut NlAttr, bindings::__IFLA_MAX as usize))
    }
}*/
