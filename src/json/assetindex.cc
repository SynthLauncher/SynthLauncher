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

void AssetIndex::AssetObject::fetch(App::AppConfig& config) {
  fs::path target_path = AssetIndex::AssetObject::path(config);

  if (!fs::exists(target_path)) {
      fs::create_directories(target_path.parent_path());

      httplib::Client cli("https://resources.download.minecraft.net");

      std::string path = "/" + hash.substr(0, 2) + "/" + hash;
      auto res = cli.Get(path.c_str());

      if (res && res->status == 200) {
          std::ofstream outFile(target_path, std::ios::binary);
          if (!outFile) {
              throw std::runtime_error("Failed to open file: " + target_path.string());
          }
          outFile.write(res->body.data(), res->body.size());
      } else {
          throw std::runtime_error("Failed to download asset: " + hash + 
                                  " - HTTP Code: " + 
                                  (res ? std::to_string(res->status) : "No response"));
      }
  }
}

AssetIndex::AssetObject
AssetIndex::AssetObject::fromJson(const rapidjson::Value &obj) {
  AssetIndex::AssetObject object;

  if (obj.HasMember("hash"))
    object.hash = obj["hash"].GetString();

  return object;
}

AssetIndex AssetIndex::fromJson(const rapidjson::Value& obj) {
  AssetIndex idx;
  if (obj.IsObject()) {
      for (auto it = obj.MemberBegin(); it != obj.MemberEnd(); ++it) {
          const auto& key = it->name.GetString();
          const auto& value = it->value;
          idx.objects[key] = AssetIndex::AssetObject::fromJson(value);
      }
  }
  return idx;
}