use bitfield_struct::bitfield;

#[repr(C)]
pub struct ValhallaNodeInfo {
    position_info: ValhallaNodeInfoPositionInfo,

    data: ValhallaNodeInfoData,

    /// For not transit levels its the headings of up to kMaxLocalEdgeIndex+1 local edges (rounded to
    /// nearest 2 degrees)for all other levels.
    /// Sadly we need to keep this for now because its used in map matching, otherwise we could remove it
    /// Also for transit levels (while building data only) it can be used for either the connecting way
    /// id for matching the connection point of the station to the edge or an encoded lon lat pair for
    /// the exact connection point. If the highest bit is set its a lon lat otherwise its a way id
    /// uint64_t headings_;
    headings: u64,
}

#[bitfield(u64)]
pub struct ValhallaNodeInfoPositionInfo {
    // // 26 bits for lat,lon offset allows 7 digits of precision even in 4 degree tiles
    // // to stay backwards compatible we have to break 6 digits and the 7th digit into two parts
    // uint64_t lat_offset_ : 22;
    /// Latitude offset from tile base latitude in int 6 digit precision
    #[bits(22)]
    lat_offset: u32,
    // uint64_t lat_offset7_ : 4;
    /// Latitude offset 7th digit of precision
    #[bits(4)]
    lat_offset7: u8,

    // uint64_t lon_offset_ : 22;
    /// Longitude offset from tile base longitude in int 6 digit precision
    #[bits(22)]
    lon_offset: u32,
    // uint64_t lon_offset7_ : 4;
    /// Longitude offset 7th digit of precision
    #[bits(4)]
    lon_offset7: u8,

    // uint64_t access_ : 12;
    /// Access through the node - bit field
    #[bits(12)]
    access: u16,
}

#[bitfield(u128)]
pub struct ValhallaNodeInfoData {
    // uint64_t edge_index_ : 21;
    /// Index within the node's tile of its first outbound directed edge
    #[bits(21)]
    edge_index: u32,

    // uint64_t edge_count_ : 7;
    /// Number of outbound edges (on this level)
    #[bits(7)]
    edge_count: u32,

    // uint64_t admin_index_ : 12;
    /// Index into this tile's administrative information list
    #[bits(12)]
    admin_index: u16,

    // uint64_t timezone_ : 9;
    /// Time zone
    #[bits(9)]
    timezone: u32,

    // uint64_t intersection_ : 4;
    /// Intersection type (see graphconstants.h)
    #[bits(4)]
    intersection: u32,

    // uint64_t type_ : 4;
    /// NodeType (see graphconstants.h)
    #[bits(4)]
    node_type: u32,

    // uint64_t density_ : 4;
    /// Relative road density
    #[bits(4)]
    density: u32,

    // uint64_t traffic_signal_ : 1;
    /// Traffic signal
    #[bits(1)]
    traffic_signal: bool,

    // uint64_t mode_change_ : 1;
    /// Mode change allowed? Also used for aggregation of edges at filter stage
    #[bits(1)]
    mode_change: bool,

    // uint64_t named_ : 1;
    /// Is this a named intersection?
    #[bits(1)]
    is_named: bool,

    // uint64_t transition_index_ : 21;
    /// Index into the node transitions to the first transition (used to store transit stop index for transit level)
    #[bits(21)]
    transition_index: u32,

    // uint64_t transition_count_ : 3;
    /// Number of transitions from this node
    #[bits(3)]
    transition_count: u32,
    // uint64_t local_driveability_ : 16;
    /// Driveability for regular edges (up to kMaxLocalEdgeIndex+1 edges)
    #[bits(16)]
    local_driveability: u32,

    // uint64_t local_edge_count_ : 3;
    /// # of regular edges across all levels (up to kMaxLocalEdgeIndex+1)
    #[bits(3)]
    local_edge_count: u32,

    // uint64_t drive_on_right_ : 1;
    /// Driving side. Right if true (false=left)
    #[bits(1)]
    drive_on_right: bool,

    // uint64_t tagged_access_ : 1;
    /// Was access initially tagged?
    #[bits(1)]
    tagged_access: bool,

    // uint64_t private_access_ : 1;
    /// Is the access private?
    #[bits(1)]
    private_access: bool,
    // uint64_t cash_only_toll_ : 1;
    /// Is this toll cash only?
    #[bits(1)]
    cash_only_toll: bool,
    // uint64_t elevation_ : 15;
    /// Encoded elevation (meters)
    #[bits(15)]
    elevation: u16,
    // uint64_t timezone_ext_1_ : 1;
    /// To keep compatibility when new timezones are added
    /// uncomment a new timezone ever gets created from a previously new
    /// timezone (reference release is 2023c)
    /// uint64_t timezone_ext_2_ : 1;
    #[bits(1)]
    timezone_ext_1: bool,

    // uint64_t spare2_ : 1;
    #[bits(1)]
    spare2: bool,
}