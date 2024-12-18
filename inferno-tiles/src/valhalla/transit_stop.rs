use bitfield_struct::bitfield;
use rkyv::Archive;
use zerocopy::{FromBytes, Immutable, KnownLayout};

#[bitfield(u64)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTransitStop {
    // uint64_t one_stop_offset_ : 24;
    /// one stop Id offset.
    #[bits(24)]
    pub(crate) one_stop_offset: u32,
    // uint64_t name_offset_ : 24;
    /// Stop name offset in the text/name list.
    #[bits(24)]
    pub(crate) name_offset: u32,
    // uint64_t generated_ : 1;
    /// Whether this stop is generated or not.
    #[bits(1)]
    pub(crate) generated: bool,
    // uint64_t traversability_ : 2;
    /// Traversability of the stop.
    #[bits(2)]
    pub(crate) traversability: u8,
    // uint64_t spare_ : 13;
    /// Spare bits for future use.
    #[bits(13)]
    _spare: u16,
}
