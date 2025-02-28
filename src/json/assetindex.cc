#include "include/json/assetindex.hh"

std::string AssetIndex::AssetObject::id() {
  if (hash.empty() || hash.size() < 2) throw std::runtime_error("Invalid hash: too short to generate ID.");

  return hash.substr(0, 2);
}

std::string AssetIndex::AssetObject::url() {
  return "https://resources.download.minecraft.net/" + this->id() + "/" + hash;
}

fs::path AssetIndex::AssetObject::path(AppConfig &config) {
  return config.ASSETS_DIR / "objects" / this->id() / hash;
}

void AssetIndex::AssetObject::fetch(AppConfig &config) {
  fs::path asset_path = this->path(config);

  if (!fs::exists(asset_path)) {
    httplib::Client cli(this->url());

    auto res = cli.Get("/");
    if (res && res->status == 200) {
      fs::create_directories(asset_path.parent_path());

      std::ofstream outFile(asset_path, std::ios::binary);
      
      if (!outFile) throw std::runtime_error("Failed to open file for writing.");

      outFile.write(res->body.data(), res->body.size());
    } else {
      throw std::runtime_error("Failed to download asset: " + hash);
    }
  }
}