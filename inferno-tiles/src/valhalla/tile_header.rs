use bitfield_struct::bitfield;

#[repr(C)]
pub struct ValhallaTileHeader {
    metadata: ValhallaTileHeaderMetadata,

    // std::array<char, 16> version_ = {};
    version: [u8; 16],

    // uint64_t dataset_id_ = 0;
    dataset_id: u64,

    counts1: ValhallaTileHeaderCounts1,

    // std::pair<float, float> base_ll_ = {0, 0};
    base_ll: [f32; 2],

    counts2: ValhallaTileHeaderCounts2,

    counts3: ValhallaTileHeaderCounts3,

    // uint64_t spareword0_ = 0;
    // uint64_t spareword1_ = 0;
    _spare_words: [u64; 2],

    // uint32_t complex_restriction_forward_offset_ = 0;
    complex_restriction_forward_offset: u32,

    // uint32_t complex_restriction_reverse_offset_ = 0;
    complex_restriction_reverse_offset: u32,

    // uint32_t edgeinfo_offset_ = 0;
    edge_info_offset: u32,

    // uint32_t textlist_offset_ = 0;
    text_list_offset: u32,

    // uint32_t date_created_ = 0;
    date_created: u32,

    // std::array<uint32_t, kBinCount> bin_offsets_ = {};
    bin_offsets: [u32; 25],

    // uint32_t lane_connectivity_offset_ = 0;
    late_connectivity_offset: u32,

    // uint32_t predictedspeeds_offset_ = 0;
    predicted_speeds_offset: u32,

    // uint32_t tile_size_ = 0;
    tile_size: u32,

    // std::array<uint32_t, kEmptySlots> empty_slots_ = {};
    empty_slots: [u32; 11],
}

#[bitfield(u64)]
pub struct ValhallaTileHeaderMetadata {
    // uint64_t graphid_ : 46;
    #[bits(46)]
    graphid: u64,

    // uint64_t density_ : 4;
    #[bits(4)]
    density: u8,

    // uint64_t name_quality_ : 4;
    #[bits(4)]
    speed_quality: u8,

    // uint64_t speed_quality_ : 4;
    #[bits(4)]
    exit_quality: u8,

    // uint64_t exit_quality_ : 4;
    #[bits(4)]
    name_quality: u8,

    // uint64_t has_elevation_ : 1;
    #[bits(1)]
    has_elevation: bool,

    // uint64_t has_ext_directededge_ : 1;
    #[bits(1)]
    has_ext_directededge: bool,
}

#[bitfield(u128)]
pub struct ValhallaTileHeaderCounts1 {
    // uint64_t nodecount_ : 21;
    #[bits(21)]
    node_count: u32,

    // uint64_t directededgecount_ : 21;
    #[bits(21)]
    directed_edges_count: u32,

    // uint64_t predictedspeeds_count_ : 21;
    #[bits(21)]
    predicted_speeds_count: u32,

    // uint64_t spare1_ : 1;
    #[bits(1)]
    _spare1: u8,

    // uint32_t transitioncount_ : 22;
    #[bits(22)]
    transition_count: u32,

    // uint32_t spare3_ : 10;
    #[bits(10)]
    _spare3: u32,

    // uint32_t turnlane_count_ : 21;
    #[bits(21)]
    turn_lane_count: u32,

    // uint64_t spare4_ : 11;
    #[bits(11)]
    _spare4: u16,
}

#[bitfield(u128)]
pub struct ValhallaTileHeaderCounts2 {
    // uint64_t transfercount_ : 16;
    #[bits(16)]
    transfer_count: u32,

    // uint64_t spare2_ : 7;
    #[bits(7)]
    _spare2: u8,

    // uint64_t departurecount_ : 24;
    #[bits(24)]
    departure_count: u32,

    // uint64_t stopcount_ : 16;
    #[bits(16)]
    stop_count: u16,

    // uint64_t spare5_ : 1;
    #[bits(1)]
    _spare5: u8,

    // uint64_t routecount_ : 12;
    #[bits(12)]
    route_count: u16,

    // uint64_t schedulecount_ : 12;
    #[bits(12)]
    schedule_count: u16,

    // uint64_t signcount_ : 24;
    #[bits(24)]
    sign_count: u32,

    // uint64_t spare6_ : 16;
    #[bits(16)]
    spare6: u16,
}

#[bitfield(u64)]
struct ValhallaTileHeaderCounts3 {
    // uint64_t access_restriction_count_ : 24;
    #[bits(24)]
    access_restriction_count: u32,

    // uint64_t admincount_ : 16;
    #[bits(16)]
    admin_count: u32,

    // uint64_t spare7_ : 24;
    #[bits(24)]
    _spare7: u32,
}
