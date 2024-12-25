use bitfield_struct::bitfield;
use rkyv::Archive;
use zerocopy::{FromBytes, Immutable, KnownLayout};

#[derive(Debug, Clone, Copy, Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaEdgeInfo {
    // uint32_t wayid_ : 32;
    /// OSM way Id
    way_id: u32,
    bitfield1: ValhallaEdgeInfoBitfield1,
    bitfield2: ValhallaEdgeInfoBitfield2,
}

#[bitfield(u32)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaEdgeInfoBitfield1 {
    // uint32_t mean_elevation_ : 12;
    /// Mean elevation with 2 meter precision
    #[bits(12)]
    mean_elevation: u32,
    // uint32_t bike_network_ : 4;
    /// Mask of bicycle network types (see graphconstants.h)
    #[bits(4)]
    bike_network: u8,
    // uint32_t speed_limit_ : 8;
    /// Speed limit (kph)
    #[bits(8)]
    speed_limit: u8,
    // uint32_t extended_wayid0_ : 8;
    /// Next byte of the way id
    #[bits(8)]
    extended_wayid0: u8,
}

#[bitfield(u32)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaEdgeInfoBitfield2 {
    // uint32_t name_count_ : 4;
    /// How many name infos we expect
    #[bits(4)]
    name_count: usize,
    // uint32_t encoded_shape_size_ : 16;
    /// How many bytes long the encoded shape is
    #[bits(16)]
    encoded_shape_size: usize,
    // uint32_t extended_wayid1_ : 8;
    /// Next next byte of the way id
    #[bits(8)]
    extended_wayid1: u8,
    // uint32_t extended_wayid_size_ : 2;
    /// How many more bytes the way id is stored in
    #[bits(2)]
    extended_wayid_size: usize,
    // uint32_t has_elevation_ : 1;
    /// Does the edgeinfo have elevation?
    #[bits(1)]
    has_elevation: bool,
    // uint32_t spare0_ : 1;
    /// not used
    #[bits(1)]
    _spare: u8,
}
