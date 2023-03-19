use crate::error::OperationReason::{FailedAllocate, NotReflectedValue};
use crate::error::{OperationReason, PciError, PciResult};
use crate::xhci::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhci::registers::capability_registers::structural_parameters1::number_of_device_slots::NumberOfDeviceSlots;
use crate::xhci::registers::capability_registers::CapabilityRegisters;
use crate::xhci::registers::memory_mapped_addr::MemoryMappedAddr;
use crate::xhci::registers::operational_registers::config_register::max_device_slots_enabled::MaxDeviceSlotsEnabled;
use crate::xhci::registers::operational_registers::device_context_base_address_array_pointer::DeviceContextBaseAddressArrayPointer;
use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use crate::xhci::registers::operational_registers::usb_command_register::run_stop::RunStop;
use crate::xhci::registers::operational_registers::OperationalRegisters;
use crate::VolatileAccessible;

pub mod capability_registers;
pub mod doorbell_registers;
pub mod memory_mapped_addr;
pub mod operational_registers;
pub mod runtime_registers;

#[derive(Debug)]
pub struct Registers {
    pub capability_registers: CapabilityRegisters,
    pub operational_registers: OperationalRegisters,
}

impl Registers {
    pub fn new(mmio_addr: MemoryMappedAddr) -> PciResult<Self> {
        let capability_registers = CapabilityRegisters::new(mmio_addr)?;
        let operational_registers = OperationalRegisters::new(OperationalRegistersOffset::new(
            mmio_addr,
            capability_registers.cap_length(),
        ))?;
        Ok(Self {
            capability_registers,
            operational_registers,
        })
    }

    ///
    /// 1. xhcのリセット
    /// 2. デバイスコンテキストの設定
    pub fn init(&self) -> PciResult {
        // reset_controller()?;
        // set_device_context()?;
        // allocate_device_context_array()?
        // USBCOMMAND RUN
        self.operational_registers.reset_host_controller();
        self.set_device_context()?;
        Ok(())
    }

    pub fn set_device_context(&self) -> PciResult {
        set_device_context(
            self.operational_registers.usb_command().run_stop(),
            self.capability_registers.hcs_params1().max_slots(),
            self.operational_registers.config().max_slots_en(),
        )
    }

    pub unsafe fn allocate_device_context_array(
        &self,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult {
        allocate_device_context_array(
            self.operational_registers.dcbaap(),
            self.operational_registers.config().max_slots_en(),
            allocator,
        )
    }
}

/// 接続できるデバイス数を取得して、コンフィグレジスタに設定します。
fn set_device_context(
    run_stop: &RunStop,
    max_slots: &NumberOfDeviceSlots,
    max_slots_en: &MaxDeviceSlotsEnabled,
) -> PciResult {
    if run_stop.read_flag_volatile() {
        return Err(PciError::FailedOperateToRegister(
            OperationReason::XhcRunning,
        ));
    }
    let enable_slots = max_slots.read_volatile();
    max_slots_en.write_volatile(enable_slots);

    if max_slots.read_volatile() == enable_slots {
        Ok(())
    } else {
        Err(PciError::FailedOperateToRegister(
            OperationReason::NotReflectedValue {
                value: enable_slots as usize,
            },
        ))
    }
}

unsafe fn allocate_device_context_array(
    dcbaap: &DeviceContextBaseAddressArrayPointer,
    max_slots_en: &MaxDeviceSlotsEnabled,
    allocator: &mut impl MemoryAllocatable,
) -> PciResult {
    const DEVICE_CONTEXT_SIZE: usize = 1024;

    let alloc_size = DEVICE_CONTEXT_SIZE * (max_slots_en.read_volatile() + 1) as usize;
    let device_context_array_addr = allocator
        .alloc(alloc_size)
        .ok_or(PciError::FailedOperateToRegister(FailedAllocate))?;
    dcbaap.write_volatile(device_context_array_addr as u64);

    let addr = dcbaap.read_volatile();
    if addr == device_context_array_addr as u64 {
        Ok(())
    } else {
        Err(PciError::FailedOperateToRegister(NotReflectedValue {
            value: addr as usize,
        }))
    }
}
