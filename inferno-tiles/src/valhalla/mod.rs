use std::{borrow::Borrow, ops::Deref};

use graph_id::{GraphEntityId, TileId};

pub(crate) mod access_restrictions;
pub(crate) mod admin;
pub(crate) mod directed_edge;
pub(crate) mod directed_edge_ext;
pub(crate) mod edge_info;
pub(crate) mod graph_id;
pub(crate) mod node_info;
pub(crate) mod node_transition;
pub(crate) mod sign;
pub(crate) mod tile_header;
pub(crate) mod transit_departure;
pub(crate) mod transit_route;
pub(crate) mod transit_schedule;
pub(crate) mod transit_stop;
pub(crate) mod transit_transfer;

pub trait HasEntityPointer<Inner> {
    fn get_entity(&self) -> GraphEntityId<Inner>;
}

pub trait HasEntityPointerInner<Inner> {
    fn get_unchecked(&self, tile_id: &TileId) -> GraphEntityId<Inner>;
}

#[derive(Debug)]
pub struct VEntity<Inner> {
    inner: Inner,
    tile_id: TileId,
}

impl<Inner> VEntity<Inner> {
    pub fn new(tile_id: TileId, inner: Inner) -> VEntity<Inner> {
        VEntity { inner, tile_id }
    }
}

impl<Inner> VEntity<Inner> {
    pub fn tile_id(&self) -> TileId {
        self.tile_id
    }

    pub fn inner<'a>(&'a self) -> &'a Inner {
        &self.inner
    }
}

impl<Inner> Borrow<Inner> for VEntity<Inner> {
    fn borrow(&self) -> &Inner {
        self.inner()
    }
}

impl<Inner> AsRef<Inner> for VEntity<Inner> {
    fn as_ref(&self) -> &Inner {
        &self.inner
    }
}

impl<Inner> Deref for VEntity<Inner> {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<Inner, T> HasEntityPointer<Inner> for VEntity<&T>
where
    T: HasEntityPointerInner<Inner>,
{
    fn get_entity(&self) -> GraphEntityId<Inner> {
        // This is safe because we're providing our own tile_id.
        self.inner.get_unchecked(&self.tile_id)
    }
}
