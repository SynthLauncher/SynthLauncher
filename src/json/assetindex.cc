#include "include/json/assetindex.hh"

std::string AssetIndex::AssetObject::id() { 
    return std::string { hash[0], hash[1] }; 
}

std::string AssetIndex::AssetObject::url() { 
    return "https://resources.download.minecraft.net/" + this->id() + "/" + hash;
}

fs::path AssetIndex::AssetObject::path() { 
    return fs::path(); 
}

void AssetIndex::AssetObject::fetch() {

}
