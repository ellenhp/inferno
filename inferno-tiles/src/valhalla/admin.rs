use rkyv::Archive;
use zerocopy::{FromBytes, Immutable, KnownLayout};

#[repr(C)]
#[derive(Debug, Clone, Copy, Archive, FromBytes, KnownLayout, Immutable)]
pub(crate) struct ValhallaAdmin {
    // uint32_t country_offset_;
    // country name offset
    pub(crate) country_offset: u32,
    // uint32_t state_offset_;
    // state name offset
    pub(crate) state_offset: u32,
    // std::array<char, kCountryIso> country_iso_{};
    // country ISO3166-1
    pub(crate) country_iso: [u8; 2],
    // std::array<char, kStateIso> state_iso_{};
    // state ISO3166-2
    pub(crate) state_iso: [u8; 3],
    // char spare_[3]{};
    // spare for byte alignment
    _spare: [u8; 3],
}
