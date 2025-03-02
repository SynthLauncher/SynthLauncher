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

TEST(ClientCC, DeserializeArgumentsTest) { 
    simdjson::ondemand::parser parser;
    simdjson::padded_string json = simdjson::padded_string::load("E:/OneDrive/Desktop/SynthLauncher/assets/argument.json");
    simdjson::ondemand::document doc = parser.iterate(json);

    simdjson::ondemand::array arr = doc.get_array().value();

    std::vector<Client::Argument> arguments;
    for (auto val : arr) {
      arguments.push_back(Client::Argument::deserialize(val.value()));
    }
}

TEST(ClientCC, DeserializeGameAndJvmArgsTest) { 
    simdjson::ondemand::parser parser;
    simdjson::padded_string json = simdjson::padded_string::load("E:/OneDrive/Desktop/SynthLauncher/assets/25w03a.json");
    simdjson::ondemand::document doc = parser.iterate(json);

    simdjson::ondemand::object obj = doc["arguments"].get_object().value();

    Client::Arguments args = Client::Arguments::deserialize(obj);
}

TEST(ClientCC, DeserializeDownloadTest) { 
    simdjson::ondemand::parser parser; 
    simdjson::padded_string json = simdjson::padded_string::load("E:/OneDrive/Desktop/SynthLauncher/assets/download.json");
    simdjson::ondemand::document doc = parser.iterate(json);

    simdjson::ondemand::object obj = doc["download"].get_object().value();

    Client::Download download = Client::Download::deserialize(obj);

    ASSERT_EQ(download.id, "");
    ASSERT_EQ(download.path, "org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-windows-arm64.jar");
    ASSERT_EQ(download.sha1, "e9aca8c5479b520a2a7f0d542a118140e812c5e8");
    ASSERT_EQ(download.size, 133378);
    ASSERT_EQ(download.url, "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-windows-arm64.jar");
    ASSERT_EQ(download.totalSize, std::nullopt);
}