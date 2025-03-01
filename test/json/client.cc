#include <gtest/gtest.h>
#include "include/json/client.hh"

TEST(ClientCC, DeserializeFeaturesTest) { 
    simdjson::ondemand::parser parser;
    simdjson::padded_string json = simdjson::padded_string::load("E:/OneDrive/Desktop/SynthLauncher/assets/features.json");
    simdjson::ondemand::document doc = parser.iterate(json);

    simdjson::ondemand::object obj = doc["features"].get_object().value();



    Client::Features features;
    features = features.deserialize(obj);

    ASSERT_EQ(features.isDemoUser, true);
    ASSERT_EQ(features.hasQuickPlaysSupport, true);
    ASSERT_EQ(features.hasCustomResolution, false);
}