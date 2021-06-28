// SPDX-License-Identifier: GPL-2.0

//! Inspired by the intel e1000 driver
//! Intel e1000 COpyright (c) 1999 - 2006 Intel Corporation.

#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::pci::PciDeviceId;
use kernel::net::device;
use kernel::net::prelude::*;
use kernel::net::rtnl;
use kernel::Error;
use kernel::{
    net::netlink::{NlAttrVec, NlExtAck},
    prelude::*,
};

const fn e1000_ethernet_device(device: u32) -> PciDeviceId {
    PciDeviceId::new(0x8086, device)
}

#[no_mangle]
pub static __test: u32 = 1;

#[no_mangle]
pub static __test_ptr: &u32 = &__test;


#[no_mangle]
//pub static __mod_pci__e1000_device_table: [PciDeviceId; 38] = [
pub static e1000_pci_tbl: [PciDeviceId; 38] = [
    e1000_ethernet_device(0x1000),
    e1000_ethernet_device(0x1001),
    e1000_ethernet_device(0x1004),
    e1000_ethernet_device(0x1008),
    e1000_ethernet_device(0x1009),
    e1000_ethernet_device(0x100C),
    e1000_ethernet_device(0x100D),
    e1000_ethernet_device(0x100E),
    e1000_ethernet_device(0x100F),
    e1000_ethernet_device(0x1010),
    e1000_ethernet_device(0x1011),
    e1000_ethernet_device(0x1012),
    e1000_ethernet_device(0x1013),
    e1000_ethernet_device(0x1014),
    e1000_ethernet_device(0x1015),
    e1000_ethernet_device(0x1016),
    e1000_ethernet_device(0x1017),
    e1000_ethernet_device(0x1018),
    e1000_ethernet_device(0x1019),
    e1000_ethernet_device(0x101A),
    e1000_ethernet_device(0x101D),
    e1000_ethernet_device(0x101E),
    e1000_ethernet_device(0x1026),
    e1000_ethernet_device(0x1027),
    e1000_ethernet_device(0x1028),
    e1000_ethernet_device(0x1075),
    e1000_ethernet_device(0x1076),
    e1000_ethernet_device(0x1077),
    e1000_ethernet_device(0x1078),
    e1000_ethernet_device(0x1079),
    e1000_ethernet_device(0x107A),
    e1000_ethernet_device(0x107B),
    e1000_ethernet_device(0x107C),
    e1000_ethernet_device(0x108A),
    e1000_ethernet_device(0x1099),
    e1000_ethernet_device(0x10B5),
    e1000_ethernet_device(0x2E6E),
    // required last entry
    PciDeviceId::null()
];


global_asm!(
    "alias {},_123_test",
    //"export _123_test"
    sym(e1000_pci_tbl)
);


module! {
    type: E100rs,
    name: b"e1000rs",
    author: b"Finn Behrens",
    description: b"Intel(R) PRO/1000 Network Driver rust",
    license: b"GPL v2",
    /*params: {
        numdummies: usize {
            default: 0,
            permissions: 0,
            description: b"Number of dummy_rs pseudo devices",
        },
    },*/
}

struct E100rs {

}

impl KernelModule for E100rs {
    fn init() -> Result<Self> {
        // INIT pcie

        Ok(E100rs {

        })
    }
}

impl Drop for E100rs {
    fn drop(&mut self) {
        // remove from pcie
    }
}
