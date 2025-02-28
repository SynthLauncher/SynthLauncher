#include <gtest/gtest.h>
#include <simdjson.h>
#include <optional>

#include "include/entities/os.hh"
#include "include/entities/os.hh"
#include "include/json/client.hh"

/*
!!! WARNING I HAVEN'T IMPLEMENTED THE RELATIVE PATHS YET!!!
*/

TEST(ClientCC, DeserializingFeaturesTest) { 
    simdjson::ondemand::parser parser;
    simdjson::padded_string json = simdjson::padded_string::load("E:/OneDrive/Desktop/SynthLauncher/assets/features.json");
    simdjson::ondemand::document doc = parser.iterate(json);

    Client::Features features;
    Client::ArgumentDeserializer argDeserializer;
    
    simdjson::ondemand::object obj = doc["features"].get_object().value();

    features = argDeserializer.deserialize_features(obj);

    ASSERT_EQ(features.isDemoUser, true);
    ASSERT_EQ(features.hasQuickPlaysSupport, true);
}

TEST(ClientCC, DeserializingOsRulesTest) { 
    simdjson::ondemand::parser parser;
    simdjson::padded_string json = simdjson::padded_string::load("E:/OneDrive/Desktop/SynthLauncher/assets/osrule.json");
    simdjson::ondemand::document doc = parser.iterate(json);

    Client::OSRules osRules;
    Client::ArgumentDeserializer argDeserializer;

    simdjson::ondemand::object obj = doc["os"].get_object().value();

    osRules = argDeserializer.deserialize_os_rules(obj);

    ASSERT_EQ(osRules.name, OperatingSystem::OS::OSX);
    ASSERT_EQ(osRules.arch, Architecture::Arch::X86);
    ASSERT_EQ(osRules.version, std::nullopt);
}