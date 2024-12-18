use bitfield_struct::bitfield;
use rkyv::Archive;
use zerocopy::{FromBytes, Immutable, KnownLayout};

#[repr(C)]
#[derive(Debug, Clone, Copy, Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTransitSchedule {
    // uint64_t days_;
    /// Days this departure is active relative to the tile's creation date. Stores bit field with 1's meaning the departure applies to the day.
    pub(crate) days: u64,

    pub(crate) data1: ValhallaTransitScheduleData1,
}

#[bitfield(u64)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTransitScheduleData1 {
    // uint64_t days_of_week_ : 7;
    /// Days of the week. Bit mask.
    #[bits(7)]
    pub(crate) days_of_week: u64,
    // uint64_t end_day_ : 6;
    /// End day (what is our end day in the days_).
    #[bits(6)]
    pub(crate) end_day: u64,
    // uint64_t spare_ : 51;
    /// Spare bits.
    #[bits(51)]
    _spare: u64,
}
