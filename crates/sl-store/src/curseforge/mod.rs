use serde::Deserialize;

pub mod api;
pub mod modpack;

#[repr(u8)]
pub enum CurseforgeModLoader {
    Forge = 1,
    Fabric = 4,
    Quilt = 5,
    NeoForge = 6,
}

#[derive(Debug, Deserialize)]
pub struct CurseforgeResponse<T> {
    data: T,
}
