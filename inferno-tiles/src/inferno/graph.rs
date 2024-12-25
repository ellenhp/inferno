use std::collections::HashMap;

use rstar::{
    primitives::{GeomWithData, Line},
    RTree,
};
use tracing::{debug, instrument, warn};

use crate::{
    geomath::{lat_lng_to_cartesian, LatLng},
    valhalla::{
        directed_edge::ValhallaDirectedEdge,
        edge_info::ValhallaEdgeInfo,
        graph_id::{GraphEntityId, TileId},
        HasEntityPointer, VEntity,
    },
};

use super::InfernoTile;

pub struct InfernoTileGraph<'a> {
    tiles: HashMap<TileId, InfernoTileLoaded<'a>>,
}

impl<'a> InfernoTileGraph<'a> {
    #[instrument(skip(tiles))]
    pub fn new(tiles: &'a [InfernoTile]) -> Self {
        let tiles: HashMap<TileId, &InfernoTile> = tiles
            .into_iter()
            .map(|tile| (tile.tile_id(), tile))
            .collect();
        debug!("Loading {} tiles...", tiles.len());
        let loaded_tiles: HashMap<_, _> = tiles
            .values()
            .map(|tile| (tile.tile_id(), Self::load_tile(tile, &tiles)))
            .collect();
        debug!("Loaded {} tiles", tiles.len());

        Self {
            tiles: loaded_tiles,
        }
    }

    #[instrument(skip(tile, tiles))]
    fn load_tile(
        tile: &'a InfernoTile,
        tiles: &HashMap<TileId, &InfernoTile>,
    ) -> InfernoTileLoaded<'a> {
        let mut elements = Vec::new();
        for (edge_index, edge) in tile.directed_edges.iter().enumerate() {
            debug!("Trying edge index: {}", edge_index);
            let end_node_idx = edge.end_node();
            let end_node_tile_id = end_node_idx.tile_id();
            let end_node_tile = if let Some(end_node_tile) = tiles.get(&end_node_tile_id) {
                debug!("Found tileset id: {}", end_node_tile_id);
                end_node_tile
            } else {
                warn!("Tileset missing end node tile {}", end_node_tile_id);
                continue;
            };
            let (start_position, end_position) =
                if let Some(end_node) = end_node_tile.nodes.get(&end_node_idx) {
                    let opposing_edge = &end_node.edges(&end_node_tile)[edge.opposing_edge_index()];
                    let start_position =
                        if let Some(start_node) = tile.nodes.get(&opposing_edge.end_node()) {
                            lat_lng_to_cartesian(&start_node.position(&tile))
                        } else {
                            warn!(
                                "Tile {} missing start node {} for edge {} in tile {}",
                                end_node_tile_id,
                                opposing_edge.end_node(),
                                edge_index,
                                tile.tile_id()
                            );
                            continue;
                        };
                    let end_position = lat_lng_to_cartesian(&end_node.position(&end_node_tile));
                    (start_position, end_position)
                } else {
                    dbg!(tile.tile_id(), end_node_tile_id);
                    warn!(
                        "Tile {} missing end node {} for edge {} in tile {}",
                        end_node_tile_id,
                        end_node_idx,
                        edge_index,
                        tile.tile_id()
                    );
                    continue;
                };
            elements.push(GeomWithData::new(
                Line::new(start_position, end_position),
                GraphEntityId::from_tile_index(&tile.tile_id(), edge_index),
            ));
        }
        let rtree = RTree::bulk_load(elements);
        InfernoTileLoaded { tile, rtree }
    }

    #[instrument(skip(self))]
    pub fn edges_for_point(
        &self,
        point: &LatLng,
        max_distance_meters: f64,
        max_edges: usize,
    ) -> Vec<(GraphEntityId<ValhallaDirectedEdge>, f64)> {
        let mut edges = Vec::new();
        for (_tile_id, tile) in &self.tiles {
            edges.extend(tile.edges_for_point(point, max_distance_meters, max_edges))
        }
        for (edge, _dist) in &edges {
            let edge = self.directed_edge(edge).unwrap();
            let edge_info: GraphEntityId<ValhallaEdgeInfo> = edge.get_entity();
            let edge_info = self.edge_info(&edge_info).unwrap();
            dbg!(edge_info);
        }
        edges
    }

    pub(crate) fn directed_edge(
        &'a self,
        index: &GraphEntityId<ValhallaDirectedEdge>,
    ) -> Result<VEntity<&'a ValhallaDirectedEdge>, anyhow::Error> {
        let tile_id = index.tile_id();
        if let Some(tile) = self.tiles.get(&tile_id) {
            Ok(tile
                .tile
                .directed_edges
                .get(index)
                .expect("Missing directed edge"))
        } else {
            Err(anyhow::anyhow!("Missing tile {}", tile_id))
        }
    }

    pub(crate) fn edge_info(
        &'a self,
        index: &GraphEntityId<ValhallaEdgeInfo>,
    ) -> Result<&'a ValhallaEdgeInfo, anyhow::Error> {
        let tile_id = index.tile_id();
        if let Some(tile) = self.tiles.get(&tile_id) {
            Ok(&tile.tile.edge_infos.get(index).expect("Missing edge info"))
        } else {
            Err(anyhow::anyhow!("Missing tile {}", tile_id))
        }
    }
}

pub struct InfernoTileLoaded<'a> {
    tile: &'a InfernoTile,
    rtree: RTree<GeomWithData<Line<[f64; 3]>, GraphEntityId<ValhallaDirectedEdge>>>,
}

impl<'a> InfernoTileLoaded<'a> {
    fn edges_for_point(
        &self,
        point: &LatLng,
        max_distance_meters: f64,
        max_edges: usize,
    ) -> Vec<(GraphEntityId<ValhallaDirectedEdge>, f64)> {
        let mut edges = Vec::new();
        for (edge, distance_sq) in self
            .rtree
            .nearest_neighbor_iter_with_distance_2(&lat_lng_to_cartesian(point))
        {
            let distance = distance_sq.sqrt();
            if distance > max_distance_meters {
                break;
            }
            edges.push((edge.data.clone(), distance));
            if edges.len() >= max_edges {
                break;
            }
        }
        edges
    }
}
