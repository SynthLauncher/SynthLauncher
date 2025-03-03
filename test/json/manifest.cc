#include <gtest/gtest.h>

#include "include/json/manifest.hh"
#include "include/config/app.hh"

TEST(ManifestHH, LatestParsing) { 
    simdjson::ondemand::parser parser;
    simdjson::padded_string json = simdjson::padded_string::load("E:/OneDrive/Desktop/SynthLauncher/assets/version_manifest.json");
    simdjson::ondemand::document doc = parser.iterate(json);

    simdjson::ondemand::object obj = doc["latest"].get_object().value();

    Manifest::Latest latest = Manifest::Latest::parse(obj);

    ASSERT_EQ(latest.release, "1.21.4");
    ASSERT_EQ(latest.snapshot, "25w03a");
}

TEST(ManifestHH, VersionParsing) {
    simdjson::ondemand::parser parser;
    simdjson::padded_string json = simdjson::padded_string::load("E:/OneDrive/Desktop/SynthLauncher/assets/version_manifest.json");
    simdjson::ondemand::document doc = parser.iterate(json);

    simdjson::ondemand::array arr = doc["versions"].get_array().value();
    simdjson::ondemand::object obj = arr.at(0).get_object().value();

    Manifest::Version version = Manifest::Version::parse(obj);

    ASSERT_EQ(version.id, "25w03a");
    ASSERT_EQ(version.type, "snapshot");
    ASSERT_EQ(version.url, "https://piston-meta.mojang.com/v1/packages/355a00a8bd037d18e80110a4536d0e8b0ea73270/25w03a.json");
    ASSERT_EQ(version.time, "2025-01-15T14:39:53+00:00");
    ASSERT_EQ(version.releaseTime, "2025-01-15T14:28:04+00:00");
}

/* 
This test has to be rewritten since I reconfigured how Manifest::parse() works!
*/
TEST(ManifestHH, ManifestParsing) {
  AppConfig config = initializeAppConfig();
  Manifest::PATH =
      "E:/OneDrive/Desktop/SynthLauncher/assets/version_manifest.json";

  Manifest manifest = Manifest::parse();

  ASSERT_EQ(manifest.latest.release, "1.21.4");
  ASSERT_EQ(manifest.latest.snapshot, "25w03a");
  ASSERT_EQ(manifest.versions.at(1).id, "25w02a");
  ASSERT_EQ(manifest.versions.at(1).type, "snapshot");
  ASSERT_EQ(manifest.versions.at(1).url,
            "https://piston-meta.mojang.com/v1/packages/"
            "02a2ae8e2c54cfc39402997bae1bbb2ccc956c84/25w02a.json");
  ASSERT_EQ(manifest.versions.at(1).time, "2025-01-08T13:54:13+00:00");
  ASSERT_EQ(manifest.versions.at(1).releaseTime, "2025-01-08T13:42:18+00:00");
}