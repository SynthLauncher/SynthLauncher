use serde::Deserialize;
use sl_meta::minecraft::loaders::{
    fabric::profile::FabricLoaderProfile, forge::ForgeLoaderProfile,
    neoforge::NeoForgeLoaderProfile, quilt::profiles::QuiltLoaderProfile, vanilla::Client,
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
    Vanilla,
}

impl Loaders {
    pub fn concat(self, client: Client) -> Client {
        match self {
            Loaders::Fabric(profile) => profile.join_client(client),
            Loaders::Quilt(profile) => profile.join_client(client),
            Loaders::Forge(profile) => profile.join_client(client),
            Loaders::NeoForge(profile) => profile.join_client(client),
            Loaders::Vanilla => client,
        }
    }
}
