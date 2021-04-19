// SPDX-License-Identifier: GPL-2.0

//! Net Device Operations.
//!
//! C header: [`include/linux/netdevice.h`](../../../../include/linux/netdevice.h)

use core::{marker, ops::Deref, ops::DerefMut};

use crate::bindings;
use crate::c_from_kernel_result;
use crate::c_types;
use crate::error::{Error, KernelResult};

use super::device::{NetDevice, NetDeviceAdapter};

unsafe extern "C" fn get_drvinfo_callback<T: NetDeviceAdapter>(
    dev: *mut bindings::net_device,
    info: *mut bindings::ethtool_drvinfo,
) {
    T::EthOps::get_drvinfo(
        &NetDevice::<T>::from_ptr(dev),
        &mut EthtoolDrvinfo::from_ptr(info),
    );
}

unsafe extern "C" fn get_ts_info_callback<T: NetDeviceAdapter>(
    dev: *mut bindings::net_device,
    info: *mut bindings::ethtool_ts_info,
) -> c_types::c_int {
    c_from_kernel_result! {
        T::EthOps::get_ts_info(
            &NetDevice::<T>::from_ptr(dev),
            &mut EthToolTsInfo::from_ptr(info)
        )?;
        Ok(0)
    }
}

pub(crate) struct EthToolOperationsVtable<T: NetDeviceAdapter>(marker::PhantomData<T>);

impl<T: NetDeviceAdapter> EthToolOperationsVtable<T> {
    const VTABLE: bindings::ethtool_ops = bindings::ethtool_ops {
        _bitfield_align_1: [],
        _bitfield_1: bindings::__BindgenBitfieldUnit::<[u8; 1usize]>::new([0u8; 1usize]),
        supported_coalesce_params: 0,
        get_drvinfo: if T::EthOps::TO_USE.get_drvinfo {
            Some(get_drvinfo_callback::<T>)
        } else {
            None
        },
        get_regs_len: None,
        get_regs: None,
        get_wol: None,
        set_wol: None,
        get_msglevel: None,
        set_msglevel: None,
        nway_reset: None,
        get_link: None,
        get_link_ext_state: None,
        get_eeprom_len: None,
        get_eeprom: None,
        set_eeprom: None,
        get_coalesce: None,
        set_coalesce: None,
        get_ringparam: None,
        set_ringparam: None,
        get_pause_stats: None,
        get_pauseparam: None,
        set_pauseparam: None,
        self_test: None,
        get_strings: None,
        set_phys_id: None,
        get_ethtool_stats: None,
        begin: None,
        complete: None,
        get_priv_flags: None,
        set_priv_flags: None,
        get_sset_count: None,
        get_rxnfc: None,
        set_rxnfc: None,
        flash_device: None,
        reset: None,
        get_rxfh_key_size: None,
        get_rxfh_indir_size: None,
        get_rxfh: None,
        set_rxfh: None,
        get_rxfh_context: None,
        set_rxfh_context: None,
        get_channels: None,
        set_channels: None,
        get_dump_flag: None,
        get_dump_data: None,
        set_dump: None,
        get_ts_info: if T::EthOps::TO_USE.get_ts_info {
            Some(get_ts_info_callback::<T>)
        } else {
            None
        },
        get_module_info: None,
        get_module_eeprom: None,
        get_eee: None,
        set_eee: None,
        get_tunable: None,
        set_tunable: None,
        get_per_queue_coalesce: None,
        set_per_queue_coalesce: None,
        get_link_ksettings: None,
        set_link_ksettings: None,
        get_fecparam: None,
        set_fecparam: None,
        get_ethtool_phy_stats: None,
        get_phy_tunable: None,
        set_phy_tunable: None,
    };

    /// Builds an instance of [`struct ethtool_ops`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the adapter is compatible with the way the device is registered.
    pub(crate) const unsafe fn build() -> &'static bindings::ethtool_ops {
        &Self::VTABLE
    }
}

/// Represents which fields of [`struct ethtool_ops`] should pe populated with pointers.
pub struct EthToolToUse {
    /// The `get_drvinfo` field of [`struct ethtool_ops`].
    pub get_drvinfo: bool,

    pub get_ts_info: bool,
}

pub const ETH_TOOL_USE_NONE: EthToolToUse = EthToolToUse {
    get_drvinfo: false,
    get_ts_info: false,
};

/// Defines the [`EthToolOps::TO_USE`] field based on a list of fields to be populated.
#[macro_export]
macro_rules! declare_eth_tool_ops {
    () => {
        const TO_USE: $crate::net::ethtool::EthToolToUse = $crate::net::ethtool::ETH_TOOL_USE_NONE;
    };
    ($($i:ident),+) => {
        const TO_USE: kernel::net::ethtool::EthToolToUse =
            $crate::net::ethtool::EthToolToUse {
                $($i: true),+ ,
                ..$crate::net::ethtool::ETH_TOOL_USE_NONE
            };
    };
}

pub trait EthToolOps<T: NetDeviceAdapter>: Send + Sync + Sized {
    const TO_USE: EthToolToUse;

    fn get_drvinfo(_dev: &NetDevice<T>, _info: &mut EthtoolDrvinfo) {}

    fn get_ts_info(_dev: &NetDevice<T>, _info: &mut EthToolTsInfo) -> KernelResult<()> {
        Err(Error::EINVAL)
    }
}

#[repr(transparent)]
pub struct EthToolTsInfo {
    ptr: *const bindings::ethtool_ts_info,
}

impl EthToolTsInfo {
    /// Constructs a new [`struct ethtool_ts_info`] wrapper.
    ///
    /// # Safety
    ///
    /// The pointer `ptr` must be non-null and valid for the lifetime of the object.
    pub unsafe fn from_ptr(ptr: *const bindings::ethtool_ts_info) -> Self {
        // INVARIANTS: the safety contract ensures the type invariant will hold.
        Self { ptr }
    }

    pub unsafe fn get_ptr(&self) -> *const bindings::ethtool_ts_info {
        self.ptr
    }
}

impl Deref for EthToolTsInfo {
    type Target = bindings::ethtool_ts_info;

    fn deref(&self) -> &Self::Target {
        // SAFETY: ptr is valid
        unsafe { self.ptr.as_ref() }.unwrap()
    }
}

impl DerefMut for EthToolTsInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: ptr is valid
        unsafe { (self.ptr as *mut bindings::ethtool_ts_info).as_mut() }.unwrap()
    }
}

pub struct EthtoolDrvinfo {
    ptr: *const bindings::ethtool_drvinfo,
}

impl EthtoolDrvinfo {
    /// Constructs a new [`struct ethtool_drvinfo`] wrapper.
    ///
    /// # Safety
    ///
    /// The pointer `ptr` must be non-null and valid for the lifetime of the object.
    pub unsafe fn from_ptr(ptr: *const bindings::ethtool_drvinfo) -> Self {
        // INVARIANTS: the safety contract ensures the type invariant will hold.
        Self { ptr }
    }

    pub unsafe fn get_ptr(&self) -> *const bindings::ethtool_drvinfo {
        self.ptr
    }
}

impl Deref for EthtoolDrvinfo {
    type Target = bindings::ethtool_drvinfo;

    fn deref(&self) -> &Self::Target {
        // SAFETY: ptr is valid
        unsafe { self.ptr.as_ref() }.unwrap()
    }
}

impl DerefMut for EthtoolDrvinfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: ptr is valid
        unsafe { (self.ptr as *mut bindings::ethtool_drvinfo).as_mut() }.unwrap()
    }
}

pub mod helpers {
    use super::*;

    pub fn ethtool_op_get_ts_info<T: NetDeviceAdapter>(
        dev: &NetDevice<T>,
        info: &mut EthToolTsInfo,
    ) -> KernelResult<()> {
        // SAFETY: dev.ptr is valid if dev is valid
        unsafe {
            bindings::ethtool_op_get_ts_info(
                dev.get_ptr() as *mut bindings::net_device,
                info.get_ptr() as *mut bindings::ethtool_ts_info,
            )
        };
        Ok(())
    }
}
