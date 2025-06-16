use serde::Deserialize;
use sl_meta::minecraft::loaders::{fabric::profile::FabricLoaderProfile, forge::ForgeLoaderProfile, quilt::profiles::QuiltLoaderProfile};

pub mod fabric;
pub mod quilt;
pub mod forge;

#[derive(Debug, Deserialize)]
pub enum Loaders {
    Fabric(FabricLoaderProfile),
    Quilt(QuiltLoaderProfile),
    Forge(ForgeLoaderProfile),
}
