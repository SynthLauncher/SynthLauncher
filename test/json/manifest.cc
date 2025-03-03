#include <gtest/gtest.h>

#include "include/json/manifest.hh"

TEST(ManifestHH, LatestParsingTest) { 
    simdjson::ondemand::parser parser;
    simdjson::padded_string json = simdjson::padded_string::load("E:/OneDrive/Desktop/SynthLauncher/assets/version_manifest.json");
    simdjson::ondemand::document doc = parser.iterate(json);

    simdjson::ondemand::object obj = doc["latest"].get_object().value();

    Manifest::Latest latest = Manifest::Latest::parse_latest(obj);

    ASSERT_EQ(latest.release, "1.21.4");
    ASSERT_EQ(latest.snapshot, "25w03a");
}