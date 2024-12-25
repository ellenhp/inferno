pub mod checked_vec;
pub mod graph;

use checked_vec::CheckedVec;
use rkyv::Archive;
use tracing::{debug, instrument, trace, warn};
use zerocopy::FromBytes;

use crate::valhalla::{
    access_restrictions::ValhallaAccessRestriction,
    admin::ValhallaAdmin,
    directed_edge::ValhallaDirectedEdge,
    directed_edge_ext::ValhallaDirectedEdgeExt,
    graph_id::{GraphEntityId, TileId},
    node_info::ValhallaNodeInfo,
    node_transition::ValhallaNodeTransition,
    sign::ValhallaSign,
    tile_header::ValhallaTileHeader,
    transit_departure::ValhallaTransitDeparture,
    transit_route::ValhallaTransitRoute,
    transit_schedule::ValhallaTransitSchedule,
    transit_stop::ValhallaTransitStop,
    transit_transfer::ValhallaTransitTransfer,
};

#[derive(Clone, Debug, Archive)]
pub struct InfernoTile {
    tile_id: TileId,
    header: ValhallaTileHeader,
    nodes: CheckedVec<ValhallaNodeInfo>,
    node_transitions: CheckedVec<ValhallaNodeTransition>,
    directed_edges: CheckedVec<ValhallaDirectedEdge>,
    access_restrictions: CheckedVec<ValhallaAccessRestriction>,
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

        let tile_id = TileId::new(header.metadata.graphid());
        let mut nodes = CheckedVec::new(tile_id);
        let mut node_transitions = CheckedVec::new(tile_id);
        let mut directed_edges = CheckedVec::new(tile_id);
        let mut access_restrictions = CheckedVec::new(tile_id);

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
            nodes.push(node_info.clone());
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
            node_transitions.push(transition.clone());
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
            directed_edges.push(edge.clone());
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
            access_restrictions.push(edge.clone());
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
            tile_id: TileId::new(header.metadata.graphid()),
            header: header.clone(),
            nodes,
            node_transitions,
            directed_edges,
            access_restrictions,
        })
    }

    pub fn base_lat_lng(&self) -> (f32, f32) {
        (self.header.base_ll[0] as f32, self.header.base_ll[1] as f32)
    }

    pub fn tile_id(&self) -> TileId {
        self.tile_id
    }

    pub(crate) fn node_slice<'a>(
        &'a self,
        start: GraphEntityId,
        count: usize,
    ) -> &'a [ValhallaNodeInfo] {
        self.nodes.slice(start, count)
    }

    pub(crate) fn edge_slice<'a>(
        &'a self,
        start: GraphEntityId,
        count: usize,
    ) -> &'a [ValhallaDirectedEdge] {
        self.directed_edges.slice(start, count)
    }

    pub(crate) fn transition_slice<'a>(
        &'a self,
        start: GraphEntityId,
        count: usize,
    ) -> &'a [ValhallaNodeTransition] {
        self.node_transitions.slice(start, count)
    }
}
