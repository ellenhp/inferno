use bitfield_struct::bitfield;
use rkyv::Archive;
use zerocopy::{FromBytes, Immutable, KnownLayout};

#[repr(C)]
#[derive(Debug, Clone, Copy, Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTransitRoute {
    route_color: u32,
    route_text_color: u32,

    data1: ValhallaTransitRouteData1,
    data2: ValhallaTransitRouteData2,
    data3: ValhallaTransitRouteData3,
    data4: ValhallaTransitRouteData4,
}

#[bitfield(u64)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTransitRouteData1 {
    // uint64_t route_type_ : 8;
    /// Internal route type
    #[bits(8)]
    pub(crate) route_type: u8,
    // uint64_t one_stop_offset_ : 24;
    #[bits(24)]
    pub(crate) one_stop_offset: u32,
    // onestop Id for this route.
    // uint64_t spare1_ : 32;
    #[bits(32)]
    _spare1: u32,
}

#[bitfield(u64)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTransitRouteData2 {
    // uint64_t op_by_onestop_id_offset_ : 24;
    /// operated by onestop id.
    #[bits(24)]
    pub(crate) op_by_onestop_id_offset: usize,
    // uint64_t op_by_name_offset_ : 24;
    #[bits(24)]
    pub(crate) op_by_name_offset: usize,
    /// operated by name.
    // uint64_t spare2_ : 16;
    #[bits(16)]
    _spare2: u16,
}

#[bitfield(u64)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTransitRouteData3 {
    // uint64_t op_by_website_offset_ : 24;
    /// operated by website.
    #[bits(24)]
    op_by_website_offset: usize,
    // uint64_t short_name_offset_ : 24;
    /// Short route name.
    #[bits(24)]
    short_name_offset: usize,
    // uint64_t spare3_ : 16;
    #[bits(16)]
    _spare3: u16,
}

#[bitfield(u64)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTransitRouteData4 {
    // uint64_t long_name_offset_ : 24;
    /// Long route name.
    #[bits(24)]
    long_name_offset: usize,
    // uint64_t desc_offset_ : 24;
    /// Route description.
    #[bits(24)]
    desc_offset: usize,
    // uint64_t spare4_ : 16;
    #[bits(16)]
    _spare4: u16,
}
