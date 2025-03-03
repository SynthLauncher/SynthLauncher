#include "include/json/manifest.hh"

fs::path Manifest::PATH = "";

Manifest::Latest
Manifest::Latest::parse(simdjson::ondemand::object &obj) {
  return {
    .release = simdjson_utils::get<std::string>(obj, "release"),
          .snapshot = simdjson_utils::get<std::string>(obj, "snapshot")};
}

Manifest::Version
Manifest::Version::parse(simdjson::ondemand::object &obj) {
  return {.id = simdjson_utils::get<std::string>(obj, "id"),
          .type = simdjson_utils::get<std::string>(obj, "type"),
          .url = simdjson_utils::get<std::string>(obj, "url"),
          .time = simdjson_utils::get<std::string>(obj, "time"),
          .releaseTime = simdjson_utils::get<std::string>(obj, "releaseTime")};
}

Manifest Manifest::parse() {
  simdjson::ondemand::parser parser;
  simdjson::padded_string json = simdjson::padded_string::load(PATH.string());
  simdjson::ondemand::document doc = parser.iterate(json);

  simdjson::ondemand::object latest_obj = doc["latest"].get_object().value();
  simdjson::ondemand::array versions_arr = doc["versions"].get_array().value();

  std::vector<Version> versions;
  for (simdjson::ondemand::object version_obj : versions_arr)
    versions.push_back(Manifest::Version::parse(version_obj));

  return {.latest = Manifest::Latest::parse(latest_obj),
          .versions = versions};
}