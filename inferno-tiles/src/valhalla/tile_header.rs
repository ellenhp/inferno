use bitfield_struct::bitfield;
use rkyv::Archive;
use zerocopy::{FromBytes, Immutable, KnownLayout};

#[repr(C)]
#[derive(Debug, Clone, Copy, Archive, FromBytes, KnownLayout, Immutable)]
pub struct ValhallaTileHeader {
    pub(crate) metadata: ValhallaTileHeaderMetadata,

    // std::pair<float, float> base_ll_ = {0, 0};
    pub(crate) base_ll: [f32; 2],

    // std::array<char, 16> version_ = {};
    pub(crate) version: [u8; 16],

    // uint64_t dataset_id_ = 0;
    pub(crate) dataset_id: u64,

    pub(crate) counts1: ValhallaTileHeaderCounts1,

    pub(crate) counts2: ValhallaTileHeaderCounts2,

    pub(crate) counts3: ValhallaTileHeaderCounts3,

    pub(crate) counts4: ValhallaTileHeaderCounts4,

    pub(crate) counts5: ValhallaTileHeaderCounts5,

    // uint64_t spareword0_ = 0;
    // uint64_t spareword1_ = 0;
    pub(crate) _spare_word1: u64,
    pub(crate) _spare_word2: u64,

    // uint32_t complex_restriction_forward_offset_ = 0;
    pub(crate) complex_restriction_forward_offset: u32,

    // uint32_t complex_restriction_reverse_offset_ = 0;
    pub(crate) complex_restriction_reverse_offset: u32,

    // uint32_t edgeinfo_offset_ = 0;
    pub(crate) edge_info_offset: u32,

    // uint32_t textlist_offset_ = 0;
    pub(crate) text_list_offset: u32,

    // uint32_t date_created_ = 0;
    pub(crate) date_created: u32,

    // std::array<uint32_t, kBinCount> bin_offsets_ = {};
    pub(crate) bin_offsets: [u32; 25],

    // uint32_t lane_connectivity_offset_ = 0;
    pub(crate) late_connectivity_offset: u32,

    // uint32_t predictedspeeds_offset_ = 0;
    pub(crate) predicted_speeds_offset: u32,

    // uint32_t tile_size_ = 0;
    pub(crate) tile_size: u32,

    // std::array<uint32_t, kEmptySlots> empty_slots_ = {};
    pub(crate) empty_slots: [u32; 11],
}

#[bitfield(u64)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub struct ValhallaTileHeaderMetadata {
    // uint64_t graphid_ : 46;
    #[bits(46)]
    pub(crate) graphid: u64,

    // uint64_t density_ : 4;
    #[bits(4)]
    pub(crate) density: u8,

    // uint64_t name_quality_ : 4;
    #[bits(4)]
    pub(crate) speed_quality: u8,

    // uint64_t speed_quality_ : 4;
    #[bits(4)]
    pub(crate) exit_quality: u8,

    // uint64_t exit_quality_ : 4;
    #[bits(4)]
    pub(crate) name_quality: u8,

    // uint64_t has_elevation_ : 1;
    #[bits(1)]
    pub(crate) has_elevation: bool,

    // uint64_t has_ext_directededge_ : 1;
    #[bits(1)]
    pub(crate) has_ext_directededge: bool,
}

#[bitfield(u64)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub struct ValhallaTileHeaderCounts1 {
    // uint64_t nodecount_ : 21;
    #[bits(21)]
    pub(crate) node_count: usize,

    // uint64_t directededgecount_ : 21;
    #[bits(21)]
    pub(crate) directed_edges_count: usize,

    // uint64_t predictedspeeds_count_ : 21;
    #[bits(21)]
    pub(crate) predicted_speeds_count: usize,

    // uint64_t spare1_ : 1;
    #[bits(1)]
    _spare1: u8,
}

#[bitfield(u64)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTileHeaderCounts2 {
    // uint32_t transitioncount_ : 22;
    #[bits(22)]
    pub(crate) transition_count: usize,

    // uint32_t spare3_ : 10;
    #[bits(10)]
    _spare3: u32,

    // uint32_t turnlane_count_ : 21;
    #[bits(21)]
    pub(crate) turn_lane_count: usize,

    // uint64_t spare4_ : 11;
    #[bits(11)]
    _spare4: u16,
}

#[bitfield(u64)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTileHeaderCounts3 {
    // uint64_t transfercount_ : 16;
    #[bits(16)]
    pub(crate) transfer_count: usize,

    // uint64_t spare2_ : 7;
    #[bits(7)]
    _spare2: u8,

    // uint64_t departurecount_ : 24;
    #[bits(24)]
    pub(crate) departure_count: usize,

    // uint64_t stopcount_ : 16;
    #[bits(16)]
    pub(crate) stop_count: usize,

    // uint64_t spare5_ : 1;
    #[bits(1)]
    _spare5: u8,
}

#[bitfield(u64)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTileHeaderCounts4 {
    // uint64_t routecount_ : 12;
    #[bits(12)]
    pub(crate) route_count: usize,

    // uint64_t schedulecount_ : 12;
    #[bits(12)]
    pub(crate) schedule_count: usize,

    // uint64_t signcount_ : 24;
    #[bits(24)]
    pub(crate) sign_count: usize,

    // uint64_t spare6_ : 16;
    #[bits(16)]
    _spare6: u16,
}

#[bitfield(u64)]
#[derive(Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaTileHeaderCounts5 {
    // uint64_t access_restriction_count_ : 24;
    #[bits(24)]
    pub(crate) access_restriction_count: usize,

    // uint64_t admincount_ : 16;
    #[bits(16)]
    pub(crate) admin_count: usize,

    // uint64_t spare7_ : 24;
    #[bits(24)]
    _spare7: u32,
}
