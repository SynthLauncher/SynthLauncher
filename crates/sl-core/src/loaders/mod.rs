use serde::Deserialize;
use sl_meta::minecraft::loaders::{
    fabric::profile::FabricLoaderProfile, forge::ForgeLoaderProfile,
    neoforge::NeoForgeLoaderProfile, quilt::profiles::QuiltLoaderProfile,
};

pub mod fabric;
pub mod forge;
pub mod neoforge;
pub mod quilt;

#[derive(Debug, Deserialize)]
pub enum Loaders {
    Fabric(FabricLoaderProfile),
    Quilt(QuiltLoaderProfile),
    Forge(ForgeLoaderProfile),
    NeoForge(NeoForgeLoaderProfile),
}
