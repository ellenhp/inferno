use std::collections::HashMap;

use rstar::{
    primitives::{GeomWithData, Line},
    RTree,
};
use tracing::{debug, info, instrument, warn};

use crate::{
    geomath::lat_lng_to_cartesian,
    valhalla::graph_id::{GraphEntityId, TileId},
};

use super::InfernoTile;

pub struct InfernoTileGraph<'a> {
    tiles: Vec<InfernoTileLoaded<'a>>,
}

pub struct InfernoTileLoaded<'a> {
    tile: &'a InfernoTile,
    rtree: RTree<GeomWithData<Line<[f64; 3]>, GraphEntityId>>,
}

impl<'a> InfernoTileGraph<'a> {
    #[instrument(skip(tiles))]
    pub fn new(tiles: &'a [InfernoTile]) -> Self {
        let tiles: HashMap<TileId, &InfernoTile> = tiles
            .into_iter()
            .map(|tile| (tile.tile_id(), tile))
            .collect();
        debug!("Loading {} tiles...", tiles.len());
        let loaded_tiles: Vec<_> = tiles
            .values()
            .map(|tile| Self::load_tile(tile, &tiles))
            .collect();
        debug!("Loaded {} tiles", tiles.len());

        Self {
            tiles: loaded_tiles,
        }
    }

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
                            lat_lng_to_cartesian(start_node.position(&tile))
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
                    let end_position = lat_lng_to_cartesian(end_node.position(&end_node_tile));
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
}
