#include "include/json/manifest.hh"

fs::path Manifest::PATH = "";

Manifest::Latest
Manifest::Latest::parse(const rapidjson::Value &obj) {
  Manifest::Latest latest;

  latest.release = obj.HasMember("release") ? obj["release"].GetString() : "";
  latest.snapshot = obj.HasMember("snapshot") ? obj["snapshot"].GetString() : "";

  return latest;
}

Manifest::Version
Manifest::Version::parse(const rapidjson::Value &obj) {
  Manifest::Version version;

  version.id = obj.HasMember("id") ? obj["id"].GetString() : "";
  version.type = obj.HasMember("type") ? obj["type"].GetString() : "";
  version.url = obj.HasMember("url") ? obj["url"].GetString() : "";
  version.time = obj.HasMember("time") ? obj["time"].GetString() : "";
  version.releaseTime = obj.HasMember("releaseTime") ? obj["releaseTime"].GetString() : "";
  
  return version;
}

Manifest Manifest::parse() {
  Manifest manifest;

  rapidjson::Document doc = parse_json_file(PATH.string());

  const rapidjson::Value &latest_obj = doc["latest"];
  const rapidjson::Value &versions_arr = doc["versions"];

  manifest.latest = Manifest::Latest::parse(latest_obj);
  for (const auto& version_obj : versions_arr.GetArray())
    manifest.versions.push_back(Manifest::Version::parse(version_obj));

  return manifest;
}