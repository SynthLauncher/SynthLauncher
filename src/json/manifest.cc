#include "include/json/manifest.hh"

fs::path Manifest::PATH = "";

Manifest::Latest
Manifest::Latest::parse_latest(simdjson::ondemand::object &obj) {
  Manifest::Latest latest;
  
  latest.release = std::string(obj["release"].get_string().value());
  latest.snapshot = std::string(obj["snapshot"].get_string().value());

  return latest;
}

Manifest::Version
Manifest::Version::parse_version(simdjson::ondemand::object &obj) {
  Manifest::Version version;
  version.id = std::string(obj["id"].get_string().value());
  version.time = std::string(obj["time"].get_string().value());
  version.type = std::string(obj["type"].get_string().value());
  version.url = std::string(obj["url"].get_string().value());
  version.releaseTime = std::string(obj["releaseTime"].get_string().value());

  return version;
}

Manifest Manifest::parse_maniftest(const std::string &path) {
  simdjson::ondemand::parser parser;
  simdjson::padded_string json = simdjson::padded_string::load(path);
  simdjson::ondemand::document doc = parser.iterate(json);

  Manifest manifest;

  simdjson::ondemand::object latest_obj = doc["latest"].get_object().value();
  manifest.latest = Manifest::Latest::parse_latest(latest_obj);

  simdjson::ondemand::array versions_arr = doc["versions"].get_array().value();

  for (simdjson::ondemand::object version_obj : versions_arr) {
    manifest.versions.push_back(Manifest::Version::parse_version(version_obj));
  }

  return manifest;
}