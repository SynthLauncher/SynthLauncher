// #include <gtest/gtest.h>

// #include "include/json/manifest.hh"
// #include "include/config/app.hh"

// TEST(ManifestHH, LatestParsing) { 
//     rapidjson::Document doc = rapidjson_utils::fromJson("E:/OneDrive/Desktop/SynthLauncher/assets/version_manifest.json");
//     const rapidjson::Value &obj = doc["latest"];

//     Manifest::Latest latest = Manifest::Latest::fromJson(obj);

//     ASSERT_EQ(latest.release, "1.21.4");
//     ASSERT_EQ(latest.snapshot, "25w03a");
// }

// TEST(ManifestHH, VersionParsing) {
//     rapidjson::Document doc = rapidjson_utils::fromJson("E:/OneDrive/Desktop/SynthLauncher/assets/version_manifest.json");
//     const rapidjson::Value &obj = doc["versions"][0];

//     Manifest::Version version = Manifest::Version::fromJson(obj);

//     ASSERT_EQ(version.id, "25w03a");
//     ASSERT_EQ(version.type, "snapshot");
//     ASSERT_EQ(version.url, "https://piston-meta.mojang.com/v1/packages/355a00a8bd037d18e80110a4536d0e8b0ea73270/25w03a.json");
//     ASSERT_EQ(version.time, "2025-01-15T14:39:53+00:00");
//     ASSERT_EQ(version.releaseTime, "2025-01-15T14:28:04+00:00");
// }

// TEST(ManifestHH, ManifestParsing) {
//   App::AppConfig config = App::initAppConfig();
//   Manifest::PATH = "E:/OneDrive/Desktop/SynthLauncher/assets/version_manifest.json";

//   Manifest manifest = Manifest::fromJson();

//   ASSERT_EQ(manifest.latest.release, "1.21.4");
//   ASSERT_EQ(manifest.latest.snapshot, "25w03a");
//   ASSERT_EQ(manifest.versions.at(1).id, "25w02a");
//   ASSERT_EQ(manifest.versions.at(1).type, "snapshot");
//   ASSERT_EQ(manifest.versions.at(1).url,
//             "https://piston-meta.mojang.com/v1/packages/"
//             "02a2ae8e2c54cfc39402997bae1bbb2ccc956c84/25w02a.json");
//   ASSERT_EQ(manifest.versions.at(1).time, "2025-01-08T13:54:13+00:00");
//   ASSERT_EQ(manifest.versions.at(1).releaseTime, "2025-01-08T13:42:18+00:00");
// }