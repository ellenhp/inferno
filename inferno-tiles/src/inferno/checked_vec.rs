use rkyv::Archive;

use crate::valhalla::{
    graph_id::{GraphEntityId, TileId},
    VEntity,
};

#[derive(Debug, Clone, Archive)]
pub struct CheckedVec<Inner: Archive> {
    graph_id: TileId,
    inner: Vec<Inner>,
}

impl<Inner: Archive> CheckedVec<Inner> {
    pub fn new(graph_id: TileId) -> Self {
        Self {
            graph_id,
            inner: Vec::new(),
        }
    }

    pub fn push(&mut self, value: Inner) {
        self.inner.push(value);
    }

    pub fn get<'a>(&'a self, index: &GraphEntityId<Inner>) -> Option<VEntity<&'a Inner>> {
        if index.tile_id() != self.graph_id {
            if tracing::enabled!(tracing::Level::WARN) {
                tracing::warn!(
                    "Attempted to access element at incorrect tile ID. Expected {}, got {}",
                    self.graph_id,
                    index.tile_id()
                );
            }
        }
        let index = index.graph_index();
        if index < self.inner.len() {
            Some(VEntity::new(self.graph_id, &self.inner[index]))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Inner> {
        self.inner.iter()
    }

    pub fn into_inner(self) -> Vec<Inner> {
        self.inner
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn pop(&mut self) -> Option<Inner> {
        self.inner.pop()
    }

    pub fn insert(&mut self, index: usize, value: Inner) -> Result<(), &'static str> {
        if index <= self.inner.len() {
            self.inner.insert(index, value);
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<Inner> {
        if index < self.inner.len() {
            Some(self.inner.remove(index))
        } else {
            None
        }
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    pub fn slice(&self, start: GraphEntityId<Inner>, count: usize) -> &[Inner] {
        &self.inner[start.graph_index()..start.graph_index() + count]
    }
}
