#include "include/json/manifest.hh"

fs::path Manifest::PATH = "";

Manifest::Latest
Manifest::Latest::parse_latest(simdjson::ondemand::object &obj) {
  return {
    .release = std::string(obj["release"].get_string().value()),
          .snapshot = std::string(obj["snapshot"].get_string().value())};
}

Manifest::Version
Manifest::Version::parse_version(simdjson::ondemand::object &obj) {
  return {.id = std::string(obj["id"].get_string().value()),
          .type = std::string(obj["type"].get_string().value()),
          .url = std::string(obj["type"].get_string().value()),
          .time = std::string(obj["time"].get_string().value()),
          .releaseTime = std::string(obj["releaseTime"].get_string().value())};
}

Manifest Manifest::parse_maniftest(const std::string &path) {
  simdjson::ondemand::parser parser;
  simdjson::padded_string json = simdjson::padded_string::load(path);
  simdjson::ondemand::document doc = parser.iterate(json);

  simdjson::ondemand::object latest_obj = doc["latest"].get_object().value();
  simdjson::ondemand::array versions_arr = doc["versions"].get_array().value();

  std::vector<Version> versions;
  for (simdjson::ondemand::object version_obj : versions_arr)
    versions.push_back(Manifest::Version::parse_version(version_obj));

  return {.latest = Manifest::Latest::parse_latest(latest_obj),
          .versions = versions};
}