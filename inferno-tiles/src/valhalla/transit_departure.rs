use bitfield_struct::bitfield;
use rkyv::Archive;
use zerocopy::{FromBytes, Immutable, KnownLayout};

#[repr(C)]
#[derive(Debug, Clone, Copy, Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTransitDeparture {
    pub(crate) data1: ValhallaTransitDepartureBitfield1,
    pub(crate) data2: ValhallaTransitDepartureBitfield2,

    pub(crate) data3_union: u64,
}

#[bitfield(u64)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTransitDepartureBitfield1 {
    // uint64_t lineid_ : 20;
    /// Line Id - lookup departures by unique line id (which indicates a unique departure / arrival stop pair.
    #[bits(20)]
    line_id: u64,
    // uint64_t routeindex_ : 12;
    /// Route index.
    #[bits(12)]
    route_index: u64,
    // uint64_t tripid_ : 32;
    /// TripId (internal).
    #[bits(32)]
    trip_id: u64,
    // uint64_t blockid_ : 20;
}

#[bitfield(u64)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTransitDepartureBitfield2 {
    /// Block Id
    #[bits(20)]
    block_id: u64,
    // uint64_t schedule_index_ : 12;
    /// Schedule validity index
    #[bits(12)]
    schedule_index: u64,
    // uint64_t headsign_offset_ : 24;
    /// Headsign offset into the names/text list.
    #[bits(24)]
    headsign_offset: u64,
    // uint64_t type_ : 2;
    /// Departure type (fixed, frequency)
    #[bits(2)]
    departure_type: u64,
    // uint64_t wheelchair_accessible_ : 1;
    /// Is the vehicle wheelchair accessible?
    #[bits(1)]
    wheelchair_accessible: bool,
    // uint64_t bicycle_accessible_ : 1;
    /// Is the vehicle bicycle accessible?
    #[bits(1)]
    bicycle_accessible: bool,
    // uint64_t spare_ : 4;
    #[bits(4)]
    spare: u64,
}
