use bitfield_struct::bitfield;
use rkyv::Archive;

#[bitfield(u64)]
#[derive(Archive)]
pub struct ValhallaNodeTransition {
    // uint64_t endnode_ : 46;
    #[bits(46)]
    pub(crate) end_node: u64,

    // uint64_t up_ : 1;
    #[bits(1)]
    pub(crate) up: bool,

    // uint64_t spare_ : 17;
    #[bits(17)]
    _spare: u32,
}
