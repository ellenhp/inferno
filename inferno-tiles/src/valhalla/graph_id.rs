use std::{fmt::Display, marker::PhantomData};

use rkyv::Archive;
use zerocopy::{FromBytes, Immutable, KnownLayout};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Archive, KnownLayout, FromBytes, Immutable)]
pub struct TileId {
    pub(crate) id: u64,
}

impl TileId {
    pub(crate) fn new(id: u64) -> Self {
        TileId { id }
    }
}

impl Display for TileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.id)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GraphEntityId<Inner> {
    pub(crate) graph_entity_id: u64,
    _phantom: PhantomData<Inner>,
}

impl<Inner> GraphEntityId<Inner> {
    #[inline]
    pub fn new(graph_entity_id: u64) -> Self {
        Self {
            graph_entity_id,
            _phantom: PhantomData,
        }
    }

    pub fn from_tile_index(tile: &TileId, index: usize) -> GraphEntityId<Inner> {
        Self {
            graph_entity_id: tile.id | ((index as u64) << 25),
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn hierarchy_level(&self) -> u8 {
        (self.graph_entity_id & 0x7) as u8
    }

    #[inline]
    pub fn tile_id(&self) -> TileId {
        TileId::new(self.graph_entity_id & 0x1ffffff)
    }

    #[inline]
    pub(crate) fn graph_index(&self) -> usize {
        (self.graph_entity_id >> 25) as usize
    }
}

impl<Inner> Display for GraphEntityId<Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.graph_entity_id)
    }
}
