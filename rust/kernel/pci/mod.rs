
use crate::bindings;

pub const PCI_ANY_ID: u32 = !0;

#[repr(transparent)]
pub struct PciDeviceId {
    inner: bindings::pci_device_id,
}

impl PciDeviceId {
    pub const fn new(vendor: u32, device: u32) -> Self {
        Self {
            inner: bindings::pci_device_id {
                vendor,
                device,
                subvendor: PCI_ANY_ID,
                subdevice: PCI_ANY_ID,
                class: 0,
                class_mask: 0,
                driver_data: 0,
            }
        }
    }

    pub const fn null() -> Self {
        Self {
            inner: bindings::pci_device_id {
                vendor: 0,
                device: 0,
                subvendor: 0,
                subdevice: 0,
                class: 0,
                class_mask: 0,
                driver_data: 0,
            }
        }
    }
}