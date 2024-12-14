use bitfield_struct::bitfield;

#[bitfield(u64)]
pub struct ValhallaNodeTransition {
    // uint64_t endnode_ : 46;
    #[bits(46)]
    end_node: u64,

    // uint64_t up_ : 1;
    #[bits(1)]
    up: bool,

    // uint64_t spare_ : 17;
    #[bits(17)]
    _spare: u32,
}
