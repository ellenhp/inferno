use bitfield_struct::bitfield;

#[repr(C)]
pub struct ValhallaDirectedEdge {
    /// Mostly data related to edge restrictions.
    restrictions: ValhallaDirectedEdgeRestrictions,
    data1: ValhallaDirectedEdgeData1,
    data2: ValhallaDirectedEdgeData2,

    // Stop impact among edges
    // struct StopImpact {
    //   uint32_t stopimpact : 24;
    //   Stop impact between edges
    //   uint32_t edge_to_right : 8;
    //   Is there an edge to the right (between "from edge" and this edge)
    // };

    // Store either the stop impact or the transit line identifier. Since
    // transit lines are schedule based they have no need for edge transition
    // logic so we can safely share this field.
    // union StopOrLine {
    //   StopImpact s;
    //   uint32_t lineid;
    // };
    stop_impact_union_line_id: u64,
    // // 6th 8-byte word (this union plus the next uint32_t bitfield)
    // StopOrLine stopimpact_;

    // This is the "next uint32_t" bitfield referred to previously.
    data3: ValhallaDirectedEdgeData3,
}

#[bitfield(u128)]
pub struct ValhallaDirectedEdgeRestrictions {
    // // 1st 8-byte word
    // uint64_t endnode_ : 46;
    /// End node of the directed edge
    #[bits(46)]
    end_node: u64,

    // uint64_t restrictions_ : 8;
    /// Restrictions - mask of local edge indexes at the end node
    #[bits(8)]
    restrictions: u64,

    // uint64_t opp_index_ : 7;
    /// Opposing directed edge index
    #[bits(7)]
    opp_index: u64,
    // uint64_t forward_ : 1;
    /// Is the edge info forward or reverse
    #[bits(1)]
    is_forward: bool,
    // uint64_t leaves_tile_ : 1;
    /// Does directed edge end in a different tile?
    #[bits(1)]
    leaves_tile: bool,
    // uint64_t ctry_crossing_ : 1;
    /// Does the edge cross into new country
    #[bits(1)]
    country_crossing: bool,

    // // 2nd 8 byte word
    // uint64_t edgeinfo_offset_ : 25;
    /// Offset to edge data
    #[bits(25)]
    edge_info_offset: u32,
    // uint64_t access_restriction_ : 12;
    /// General restriction or access condition (per mode)
    #[bits(12)]
    access_restriction: u64,
    // uint64_t start_restriction_ : 12;
    /// Complex restriction (per mode) starts on this directed edge
    #[bits(12)]
    start_restriction: u64,
    // uint64_t end_restriction_ : 12;
    /// Complex restriction (per mode) ends on this directed edge
    #[bits(12)]
    end_restriction: u64,
    // uint64_t complex_restriction_ : 1;
    /// Edge is part of a complex restriction
    #[bits(1)]
    complex_restriction: bool,
    // uint64_t dest_only_ : 1;
    /// Access allowed to destination only (e.g., private)
    #[bits(1)]
    destination_only: bool,
    // uint64_t not_thru_ : 1;
    /// Edge leads to "no-through" region
    #[bits(1)]
    not_thru: bool,
}

#[bitfield(u128)]
pub struct ValhallaDirectedEdgeData1 {
    // // 3rd 8-byte word. Note: speed values above 250 for special cases (closures, construction)
    // uint64_t speed_ : 8;
    /// Speed (kph)
    #[bits(8)]
    speed: u8,
    // uint64_t free_flow_speed_ : 8;
    // Speed when there is no traffic(kph)
    #[bits(8)]
    free_flow_speed: u8,
    // uint64_t constrained_flow_speed_ : 8;
    // Speed when there is traffic(kph)
    #[bits(8)]
    constrained_flow_speed: u8,
    // uint64_t truck_speed_ : 8;
    // Truck speed (kph)
    #[bits(8)]
    truck_speed: u8,
    // uint64_t name_consistency_ : 8;
    // Name consistency at start node with other local edges
    #[bits(8)]
    name_consistency: u8,
    // uint64_t use_ : 6;
    // Specific use types
    #[bits(6)]
    use_type: u8,
    // uint64_t lanecount_ : 4;
    // Number of lanes
    #[bits(4)]
    lane_count: u8,
    // uint64_t density_ : 4;
    // Relative road density along the edge
    #[bits(4)]
    density: u8,
    // uint64_t classification_ : 3;
    // Classification/importance of the road/path
    #[bits(3)]
    classification: u8,
    // uint64_t surface_ : 3;
    // Representation of smoothness
    #[bits(3)]
    surface: u8,
    // uint64_t toll_ : 1;
    // Edge is part of a toll road
    #[bits(1)]
    toll: bool,
    // uint64_t roundabout_ : 1;
    // Edge is part of a roundabout
    #[bits(1)]
    roundabout: bool,
    // uint64_t truck_route_ : 1;
    // Edge that is part of a truck route/network
    #[bits(1)]
    truck_route: bool,
    // uint64_t has_predicted_speed_ : 1;
    // Does this edge have a predicted speed records?
    #[bits(1)]
    predicted_speed: bool,

    // // 4th 8-byte word
    // uint64_t forwardaccess_ : 12;
    // Access (bit mask) in forward direction (see graphconstants.h)
    #[bits(12)]
    forward_access_mask: u16,
    // uint64_t reverseaccess_ : 12;
    // Access (bit mask) in reverse direction (see graphconstants.h)
    #[bits(12)]
    reverse_access_mask: u16,
    // uint64_t max_up_slope_ : 5;
    // Maximum upward slope
    #[bits(5)]
    max_up_slope: u16,
    // uint64_t max_down_slope_ : 5;
    // Maximum downward slope
    #[bits(5)]
    max_down_slope: u16,
    // uint64_t sac_scale_ : 3;
    // Is this edge for hiking and if so how difficult is the hike?
    #[bits(3)]
    sac_scale: u8,
    // uint64_t cycle_lane_ : 2;
    // Does this edge have bicycle lanes?
    #[bits(2)]
    cycle_lane: u8,
    // uint64_t bike_network_ : 1;
    // Edge that is part of a bicycle network
    #[bits(1)]
    bike_network: bool,
    // uint64_t use_sidepath_ : 1;
    // Is there a cycling path to the side that should be preferred?
    #[bits(1)]
    use_sidepath: bool,
    // uint64_t dismount_ : 1;
    // Do you need to dismount when biking on this edge?
    #[bits(1)]
    dismount: bool,
    // uint64_t sidewalk_left_ : 1;
    // Sidewalk to the left of the edge
    #[bits(1)]
    sidewalk_left: bool,
    // uint64_t sidewalk_right_ : 1;
    // Sidewalk to the right of the edge
    #[bits(1)]
    sidewalk_right: bool,
    // uint64_t shoulder_ : 1;
    // Does the edge have a shoulder?
    #[bits(1)]
    shoulder: bool,
    // uint64_t lane_conn_ : 1;
    // 1 if has lane connectivity, 0 otherwise
    #[bits(1)]
    lane_connectivity: bool,
    // uint64_t turnlanes_ : 1;
    // Does this edge have turn lanes (end of edge)
    #[bits(1)]
    turn_lanes: bool,
    // uint64_t sign_ : 1;
    // Exit signs exist for this edge
    #[bits(1)]
    has_signs: bool,
    // uint64_t internal_ : 1;
    // Edge that is internal to an intersection
    #[bits(1)]
    internal: bool,
    // uint64_t tunnel_ : 1;
    // Is this edge part of a tunnel
    #[bits(1)]
    tunnel: bool,
    // uint64_t bridge_ : 1;
    // Is this edge part of a bridge?
    #[bits(1)]
    bridge: bool,
    // uint64_t traffic_signal_ : 1;
    // Traffic signal at end of the directed edge
    #[bits(1)]
    traffic_signal: bool,
    // uint64_t seasonal_ : 1;
    // Seasonal access (ex. no access in winter)
    #[bits(1)]
    seasonal: bool,
    // uint64_t deadend_ : 1;
    // Leads to a dead-end (no other drivable roads) TODO
    #[bits(1)]
    deadend: bool,
    // uint64_t bss_connection_ : 1;
    // Does this lead to(come out from) a bike share station?
    #[bits(1)]
    bss_connection: bool,
    // uint64_t stop_sign_ : 1;
    // Stop sign at end of the directed edge
    #[bits(1)]
    stop_sign: bool,
    // uint64_t yield_sign_ : 1;
    // Yield/give way sign at end of the directed edge
    #[bits(1)]
    yield_sign: bool,
    // uint64_t hov_type_ : 1;
    // if (is_hov_only()==true), this means (HOV2=0, HOV3=1)
    #[bits(1)]
    hov_type: bool,
    // uint64_t indoor_ : 1;
    // Is this edge indoor
    #[bits(1)]
    indoor: bool,
    // uint64_t lit_ : 1;
    // Is the edge lit?
    #[bits(1)]
    is_lit: bool,
    // uint64_t dest_only_hgv_ : 1;
    // destonly for HGV specifically
    #[bits(1)]
    dest_only_hgv: bool,
    // uint64_t spare4_ : 3;
    #[bits(3)]
    _spare4: u8,
}

#[bitfield(u64)]
pub struct ValhallaDirectedEdgeData2 {
    // // 5th 8-byte word
    // uint64_t turntype_ : 24;
    #[bits(24)]
    turn_type: u32,
    // Turn type (see graphconstants.h)
    // uint64_t edge_to_left_ : 8;
    // Is there an edge to the left (between the "from edge" and this edge)
    #[bits(8)]
    edge_to_left: u8,
    // uint64_t length_ : 24;
    // Length in meters
    #[bits(24)]
    length_meters: u32,
    // uint64_t weighted_grade_ : 4;
    // Weighted estimate of grade
    #[bits(4)]
    grade: u8,
    // uint64_t curvature_ : 4;
    // Curvature factor
    #[bits(4)]
    curvature: u8,
}

#[bitfield(u32)]
pub struct ValhallaDirectedEdgeData3 {
    // uint32_t localedgeidx_ : 7;
    // Index of the edge on the local level
    #[bits(7)]
    local_edge_index: u8,
    // uint32_t opp_local_idx_ : 7;
    // Opposing local edge index (for costing and Uturn detection)
    #[bits(7)]
    opposing_local_edge_index: u8,
    // uint32_t shortcut_ : 7;
    // Shortcut edge (mask)
    #[bits(7)]
    shortcut_mask: u8,
    // uint32_t superseded_ : 7;
    // Edge is superseded by a shortcut (mask)
    #[bits(7)]
    superceded: u8,
    // uint32_t is_shortcut_ : 1;
    // True if this edge is a shortcut
    #[bits(1)]
    is_shortcut: bool,
    // uint32_t speed_type_ : 1;
    // Speed type (used in setting default speeds)
    #[bits(1)]
    speed_type: u8,
    // uint32_t named_ : 1;
    // 1 if this edge has names, 0 if unnamed
    #[bits(1)]
    is_named: bool,
    // uint32_t link_ : 1;
    // *link tag - Ramp or turn channel. Used in costing.
    #[bits(1)]
    link: bool,
}
