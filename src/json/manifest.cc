#include "include/json/manifest.hh"

fs::path Manifest::PATH = "";

Manifest::Latest Manifest::Latest::fromJson(const rapidjson::Value &obj) {
  Manifest::Latest latest;

  latest.release = obj.HasMember("release") ? obj["release"].GetString() : "";
  latest.snapshot =
      obj.HasMember("snapshot") ? obj["snapshot"].GetString() : "";

  return latest;
}

Manifest::Version Manifest::Version::fromJson(const rapidjson::Value &obj) {
  Manifest::Version version;

  version.id = obj.HasMember("id") ? obj["id"].GetString() : "";
  version.type = obj.HasMember("type") ? obj["type"].GetString() : "";
  version.url = obj.HasMember("url") ? obj["url"].GetString() : "";
  version.time = obj.HasMember("time") ? obj["time"].GetString() : "";
  version.releaseTime =
      obj.HasMember("releaseTime") ? obj["releaseTime"].GetString() : "";

  return version;
}

Manifest Manifest::fromJson() {
  Manifest manifest;

  rapidjson::Document doc = rapidjson_utils::fromJson(PATH);

  const rapidjson::Value &latest_obj = doc["latest"];
  manifest.latest = Manifest::Latest::fromJson(latest_obj);

  const rapidjson::Value &versions_arr = doc["versions"];
  for (const auto &version_obj : versions_arr.GetArray())
    manifest.versions.push_back(Manifest::Version::fromJson(version_obj));

  return manifest;
}