use rkyv::Archive;
use tracing::{debug, instrument, trace, warn};
use zerocopy::FromBytes;

use crate::valhalla::{
    access_restrictions::ValhallaAccessRestriction, admin::ValhallaAdmin,
    directed_edge::ValhallaDirectedEdge, directed_edge_ext::ValhallaDirectedEdgeExt,
    node_info::ValhallaNodeInfo, node_transition::ValhallaNodeTransition, sign::ValhallaSign,
    tile_header::ValhallaTileHeader, transit_departure::ValhallaTransitDeparture,
    transit_route::ValhallaTransitRoute, transit_schedule::ValhallaTransitSchedule,
    transit_stop::ValhallaTransitStop, transit_transfer::ValhallaTransitTransfer,
};

#[derive(Clone, Debug, Archive)]
pub struct InfernoTile {
    nodes: Vec<ValhallaNodeInfo>,
    node_transitions: Vec<ValhallaNodeTransition>,
    directed_edges: Vec<ValhallaDirectedEdge>,
    access_restrictions: Vec<ValhallaAccessRestriction>,
}

const HEADER_SIZE: usize = size_of::<ValhallaTileHeader>();
const NODE_INFO_SIZE: usize = size_of::<ValhallaNodeInfo>();
const NODE_TRANSITION_SIZE: usize = size_of::<ValhallaNodeTransition>();
const DIRECTED_EDGE_SIZE: usize = size_of::<ValhallaDirectedEdge>();
const DIRECTED_EDGE_EXT_SIZE: usize = size_of::<ValhallaDirectedEdgeExt>();
const ACCESS_RESTRICTION_SIZE: usize = size_of::<ValhallaAccessRestriction>();
const TRANSIT_DEPARTURE_SIZE: usize = size_of::<ValhallaTransitDeparture>();
const TRANSIT_STOP_SIZE: usize = size_of::<ValhallaTransitStop>();
const TRANSIT_ROUTE_SIZE: usize = size_of::<ValhallaTransitRoute>();
const TRANSIT_SCHEDULE_SIZE: usize = size_of::<ValhallaTransitSchedule>();
const TRANSIT_TRANSFER_SIZE: usize = size_of::<ValhallaTransitTransfer>();
const SIGN_SIZE: usize = size_of::<ValhallaSign>();
const ADMIN_SIZE: usize = size_of::<ValhallaAdmin>();

impl InfernoTile {
    #[instrument(skip(bytes))]
    pub fn from_valhalla(bytes: &[u8]) -> Result<InfernoTile, anyhow::Error> {
        if bytes.len() < HEADER_SIZE {
            return Err(anyhow::anyhow!("Invalid tile header"));
        }
        let header = ValhallaTileHeader::ref_from_bytes(&bytes[0..HEADER_SIZE])
            .map_err(|err| anyhow::anyhow!("Failed ValhallaTileHeader cast: {:?}", err))?;

        let mut nodes = Vec::new();
        let mut node_transitions = Vec::new();
        let mut directed_edges = Vec::new();
        let mut access_restrictions = Vec::new();

        let mut ptr = HEADER_SIZE;

        if header.tile_size as usize != bytes.len() {
            warn!(
                tile_size = header.tile_size,
                actual_size = bytes.len(),
                "Tile size mismatch"
            );
            return Err(anyhow::anyhow!(
                "Invalid tile size. Expected {} and found {}",
                header.tile_size,
                bytes.len()
            ));
        }

        nodes.reserve_exact(header.counts1.node_count());
        for _ in 0..header.counts1.node_count() {
            if ptr + NODE_INFO_SIZE >= bytes.len() {
                return Err(anyhow::anyhow!(
                    "Invalid tile: not enough bytes for specified node count"
                ));
            }
            let node_info = ValhallaNodeInfo::ref_from_bytes(&bytes[ptr..ptr + NODE_INFO_SIZE])
                .map_err(|err| anyhow::anyhow!("Failed ValhallaTileHeader cast: {:?}", err))?;
            nodes.push(node_info);
            ptr += NODE_INFO_SIZE;
        }
        trace!(
            "Parsed {} nodes, ptr: 0x{:x}",
            header.counts1.node_count(),
            ptr
        );
        node_transitions.reserve_exact(header.counts2.transition_count());
        for _ in 0..header.counts2.transition_count() {
            if ptr + NODE_TRANSITION_SIZE >= bytes.len() {
                return Err(anyhow::anyhow!(
                    "Invalid tile: not enough bytes for specified transition count"
                ));
            }
            let transition =
                ValhallaNodeTransition::ref_from_bytes(&bytes[ptr..ptr + NODE_TRANSITION_SIZE])
                    .map_err(|err| anyhow::anyhow!("Failed ValhallaTileHeader cast: {:?}", err))?;
            node_transitions.push(transition);
            ptr += NODE_TRANSITION_SIZE;
        }
        trace!(
            "Parsed {} transitions, ptr: 0x{:x}",
            header.counts2.transition_count(),
            ptr
        );
        directed_edges.reserve_exact(header.counts1.directed_edges_count());
        for _ in 0..header.counts1.directed_edges_count() {
            if ptr + DIRECTED_EDGE_SIZE >= bytes.len() {
                return Err(anyhow::anyhow!(
                    "Invalid tile: not enough bytes for specified directed edge count"
                ));
            }
            let edge = ValhallaDirectedEdge::ref_from_bytes(&bytes[ptr..ptr + DIRECTED_EDGE_SIZE])
                .map_err(|err| anyhow::anyhow!("Failed ValhallaTileHeader cast: {:?}", err))?;
            directed_edges.push(edge);
            ptr += DIRECTED_EDGE_SIZE;
        }
        trace!(
            "Parsed {} directed edges, ptr: 0x{:x}",
            header.counts1.directed_edges_count(),
            ptr
        );
        if header.metadata.has_ext_directededge() {
            ptr += DIRECTED_EDGE_EXT_SIZE * header.counts1.directed_edges_count();
            trace!(
                "Parsed {} directed edge extensions, ptr: 0x{:x}",
                header.counts1.directed_edges_count(),
                ptr
            );
        } else {
            trace!("No directed edge extensions found");
        }
        for _ in 0..header.counts5.access_restriction_count() {
            if ptr + ACCESS_RESTRICTION_SIZE >= bytes.len() {
                return Err(anyhow::anyhow!(
                    "Invalid tile: not enough bytes for specified access restriction count"
                ));
            }
            let edge = ValhallaAccessRestriction::ref_from_bytes(
                &bytes[ptr..ptr + ACCESS_RESTRICTION_SIZE],
            )
            .map_err(|err| anyhow::anyhow!("Failed ValhallaTileHeader cast: {:?}", err))?;
            access_restrictions.push(edge);
            ptr += ACCESS_RESTRICTION_SIZE;
        }
        trace!(
            "Parsed {} access restrictions, ptr: 0x{:x}",
            header.counts5.access_restriction_count(),
            ptr
        );

        ptr += header.counts3.departure_count() * TRANSIT_DEPARTURE_SIZE;
        ptr += header.counts3.stop_count() * TRANSIT_STOP_SIZE;
        ptr += header.counts4.route_count() * TRANSIT_ROUTE_SIZE;
        ptr += header.counts4.schedule_count() * TRANSIT_SCHEDULE_SIZE;
        ptr += header.counts3.transfer_count() * TRANSIT_TRANSFER_SIZE;
        ptr += header.counts4.sign_count() * SIGN_SIZE;
        ptr += header.counts5.admin_count() * ADMIN_SIZE;

        debug!(
            "Valhalla tile parsed successfully. ptr: 0x{:x}, len: 0x{:x}",
            ptr,
            bytes.len()
        );

        Ok(InfernoTile {
            nodes: nodes.into_iter().cloned().collect(),
            node_transitions: node_transitions.into_iter().cloned().collect(),
            directed_edges: directed_edges.into_iter().cloned().collect(),
            access_restrictions: access_restrictions.into_iter().cloned().collect(),
        })
    }
}
