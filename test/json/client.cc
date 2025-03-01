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

TEST(ClientCC, DeserializeOSRulesTest) { 
    simdjson::ondemand::parser parser;
    simdjson::padded_string json = simdjson::padded_string::load("E:/OneDrive/Desktop/SynthLauncher/assets/osrules.json");
    simdjson::ondemand::document doc = parser.iterate(json);

    simdjson::ondemand::object obj = doc["os"].get_object().value();

    Client::OSRules rules;
    rules = rules.deserialize(obj);

    ASSERT_EQ(rules.name, OperatingSystem::OS::Windows);
    ASSERT_EQ(rules.arch, Architecture::Arch::X86);
    ASSERT_EQ(rules.version, "");
}