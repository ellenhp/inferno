use rkyv::Archive;
use zerocopy::{FromBytes, Immutable, KnownLayout};

#[repr(C)]
#[derive(Debug, Clone, Archive, FromBytes, KnownLayout, Immutable)]

pub(crate) struct ValhallaDirectedEdgeExt {
    _spare: u64,
}
