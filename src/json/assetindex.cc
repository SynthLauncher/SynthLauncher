#include "include/json/assetindex.hh"

std::string AssetIndex::AssetObject::id() {
  if (hash.size() < 2)
    throw std::runtime_error("Invalid hash: too short to generate ID.");

  return hash.substr(0, 2);
}

std::string AssetIndex::AssetObject::url() {
  return "https://resources.download.minecraft.net/" + id() + "/" + hash;
}

fs::path AssetIndex::AssetObject::path(App::AppConfig &config) {
  return config.ASSETS_DIR / "objects" /  id() / hash;
}

void AssetIndex::AssetObject::fetch(App::AppConfig &config) {
  fs::path asset_path =  path(config);

  if (!fs::exists(asset_path)) {
    httplib::Client cli(  url());

    auto res = cli.Get("/");
    if (res && res->status == 200) {
      fs::create_directories(asset_path.parent_path());

      std::ofstream outFile(asset_path, std::ios::binary);

      if (!outFile)
        throw std::runtime_error("Failed to open file for writing.");

      outFile.write(res->body.data(), res->body.size());
    } else
      throw std::runtime_error("Failed to download asset: " + hash);
  }
}

AssetIndex::AssetObject
AssetIndex::AssetObject::fromJson(const rapidjson::Value &obj) {
  AssetIndex::AssetObject object;

  if (obj.HasMember("hash"))
    object.hash = obj["hash"].GetString();

  return object;
}

AssetIndex AssetIndex::fromJson(const rapidjson::Value &obj) {
  AssetIndex idx;

  if (obj.IsObject()) {
    AssetIndex::AssetObject assetObject;
    assetObject.hash = obj["hash"].GetString();
    idx.objects[obj.GetString()] = assetObject;
  }

  return idx;
}