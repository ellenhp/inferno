use bitfield_struct::bitfield;

#[bitfield(u128)]
pub struct ValhallaAccessRestriction {
    // uint64_t edgeindex_ : 22;
    /// Directed edge index. Max index is kMaxTileEdgeCount in nodeinfo.h: 22 bits.
    #[bits(22)]
    pub(crate) edge_index: usize,
    // uint64_t type_ : 6;
    /// Access type
    #[bits(6)]
    pub(crate) access_type: u8,
    // uint64_t modes_ : 12;
    /// Mode(s) this access restriction applies to
    #[bits(12)]
    pub(crate) modes: u16,
    // uint64_t spare_ : 24;
    /// Spare.
    #[bits(24)]
    _spare: u32,
    // uint64_t value_;
    /// Value for this restriction. Can take on different meanings per type
    pub(crate) value: u64,
}
