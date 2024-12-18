use bitfield_struct::bitfield;
use rkyv::Archive;

#[bitfield(u64)]
#[derive(Archive)]
pub struct ValhallaSign {
    // kMaxTileEdgeCount in nodeinfo.h: 22 bits
    // uint32_t index_ : 22;
    #[bits(22)]
    index: u32,
    // uint32_t type_ : 8;
    #[bits(8)]
    sign_type: u8,
    // uint32_t route_num_type_ : 1;
    #[bits(1)]
    route_num_type: u8,
    // uint32_t tagged_ : 1;
    #[bits(1)]
    tagged: bool,
    // uint32_t text_offset_;
    #[bits(32)]
    text_offset: u32,
}
