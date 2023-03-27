use core::marker::PhantomData;

use macros::VolatileBits;

use crate::xhc::transfer::event::event_ring_segment_table::SegmentTableAddr;

/// Ring Segment Base Address Hi And Lo
///
/// # Size
///
/// 64Bits(下位6Bitsは予約領域)
///
/// [Xhci Document] : 515 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[derive(VolatileBits)]
#[volatile_type(u64)]
pub struct EventRingAddressEntry(usize, PhantomData<SegmentTableAddr>);
