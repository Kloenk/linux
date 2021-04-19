use core::mem;

pub mod device;
pub mod ethtool;
pub mod netlink;
pub mod rtnl;
pub mod skbuff;

#[doc(inline)]
pub use module::rtnl_link_ops;

#[cfg(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS)]
pub unsafe fn is_multicast_ether_addr(addr: *const u8) -> bool {
    let a: u32 = *(addr as *const u32);

    if cfg!(target_endian = "big") {
        (0x01 & (a >> (((mem::size_of::<u32>() as u32) * 8) - 8))) != 0
    } else {
        (0x01 & a) != 0
    }
}

#[cfg(not(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))]
pub unsafe fn is_multicast_ether_addr(addr: *const u8) -> bool {
    let a: u16 = *(addr as *const u16);

    if cfg!(target_endian = "big") {
        (0x01 & (a >> (((mem::size_of::<u16>() as u16) * 8) - 8))) != 0
    } else {
        (0x01 & a) != 0
    }
}

#[cfg(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS)]
pub unsafe fn is_zero_ether_addr(addr: *const u8) -> bool {
    *(addr as *const u32) | (*((addr as usize + 4) as *const u16) as u32) == 0
}

#[cfg(not(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))]
pub unsafe fn is_zero_ether_addr(addr: *const u8) -> bool {
    *(addr as *const u16)
        | *((addr as usize + 2) as *const u16)
        | *((addr as usize + 4) as *const u16)
        == 0
}

pub unsafe fn is_valid_ether_addr(addr: *const u8) -> bool {
    !is_multicast_ether_addr(addr) && !is_zero_ether_addr(addr)
}

pub mod prelude {
    pub use super::rtnl_link_ops;
    pub use super::{
        device::{NetDevice, NetDeviceAdapter, NetDeviceOps},
        ethtool::{self, EthToolOps},
        rtnl::{RtnlLinkOps, RtnlLock},
        skbuff::SkBuff,
    };
}
