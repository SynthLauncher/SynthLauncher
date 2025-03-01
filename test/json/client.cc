#include <gtest/gtest.h>
#include "include/json/client.hh"

TEST(ClientCC, DeserializeFeaturesTest) { 
    simdjson::ondemand::parser parser;
    simdjson::padded_string json = simdjson::padded_string::load("E:/OneDrive/Desktop/SynthLauncher/assets/features.json");
    simdjson::ondemand::document doc = parser.iterate(json);

    simdjson::ondemand::object obj = doc["features"].get_object().value();

    Client::Features features = Client::Features::deserialize(obj);

    ASSERT_EQ(features.isDemoUser, true);
    ASSERT_EQ(features.hasQuickPlaysSupport, true);
    ASSERT_EQ(features.hasCustomResolution, false);
}

TEST(ClientCC, DeserializeOSRulesTest) { 
    simdjson::ondemand::parser parser;
    simdjson::padded_string json = simdjson::padded_string::load("E:/OneDrive/Desktop/SynthLauncher/assets/osrules.json");
    simdjson::ondemand::document doc = parser.iterate(json);

    simdjson::ondemand::object obj = doc["os"].get_object().value();

    Client::OSRules rules = Client::OSRules::deserialize(obj);

    ASSERT_EQ(rules.name, OperatingSystem::OS::Windows);
    ASSERT_EQ(rules.arch, Architecture::Arch::X86);
    ASSERT_EQ(rules.version, "");
}

TEST(ClientCC, DeserializeRuleTest) { 
    simdjson::ondemand::parser parser;
    simdjson::padded_string json = simdjson::padded_string::load("E:/OneDrive/Desktop/SynthLauncher/assets/rule.json");
    simdjson::ondemand::document doc = parser.iterate(json);

    simdjson::ondemand::object obj = doc["rule"].get_object().value();

    Client::Rule rule = Client::Rule::deserialize(obj);

    ASSERT_EQ(rule.action, "allow");
    ASSERT_EQ(rule.os->name, OperatingSystem::OS::OSX);
    ASSERT_EQ(rule.os->arch, Architecture::Arch::X86_64);
    ASSERT_EQ(rule.features, std::nullopt);
}

TEST(ClientCC, DeserializeArguments) { 
    simdjson::ondemand::parser parser;
    simdjson::padded_string json = simdjson::padded_string::load("E:/OneDrive/Desktop/SynthLauncher/assets/argument.json");
    simdjson::ondemand::document doc = parser.iterate(json);

    simdjson::ondemand::array arr = doc.get_array().value();

    std::vector<Client::Argument> arguments;
    for (auto val : arr) {
      arguments.push_back(Client::Argument::deserialize(val.value()));
    }
}