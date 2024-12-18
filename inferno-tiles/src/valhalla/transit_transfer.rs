use bitfield_struct::bitfield;
use rkyv::Archive;
use zerocopy::{FromBytes, Immutable, KnownLayout};

#[repr(C)]
#[derive(Debug, Clone, Copy, Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTransitTransfer {
    // uint32_t from_stopid_;
    /// From stop Id (internal)
    pub(crate) from_stopid: u32,

    // uint32_t to_stopid_;
    /// To stop Id (internal)
    pub(crate) to_stopid: u32,

    pub(crate) data: ValhallaTransitTransferData,
}

#[bitfield(u32)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTransitTransferData {
    // uint32_t type_ : 4;
    /// Transfer type
    #[bits(4)]
    pub(crate) transfer_type: u8,
    // uint32_t mintime_ : 16;
    /// Minimum transfer time (seconds)
    #[bits(16)]
    pub(crate) min_transfer_time: u16,
    // uint32_t spare_ : 12;
    /// Spare bits
    #[bits(12)]
    _spare: u16,
}
