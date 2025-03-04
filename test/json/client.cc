#include "include/json/client.hh"
#include "include/utils/rapidjson_utils.hh"
#include <gtest/gtest.h>


TEST(ClientHH, FeaturesParsing) {
  rapidjson::Document doc =
      parse_json_file("E:/OneDrive/Desktop/SynthLauncher/assets/features.json");
  const rapidjson::Value &obj = doc["features"];

  Client::Features features = Client::Features::parse(obj);

  ASSERT_EQ(features.isDemoUser, true);
  ASSERT_EQ(features.hasQuickPlaysSupport, true);
  ASSERT_EQ(features.hasCustomResolution, false);
}

TEST(ClientHH, OSRulesParsing) {
  rapidjson::Document doc =
      parse_json_file("E:/OneDrive/Desktop/SynthLauncher/assets/os_rules.json");
  const rapidjson::Value &obj = doc["os"];

  Client::OSRules rules = Client::OSRules::parse(obj);

  ASSERT_EQ(rules.name, OperatingSystem::OS::Windows);
  ASSERT_EQ(rules.arch, std::nullopt);
  ASSERT_EQ(rules.version, "");
}

TEST(ClientHH, RuleParsing) {
    rapidjson::Document doc =
        parse_json_file("E:/OneDrive/Desktop/SynthLauncher/assets/rule.json");
    
    const rapidjson::Value &obj = doc["rule"];

    Client::Rule rule = Client::Rule::parse(obj);

    ASSERT_EQ(rule.action, "allow");
    ASSERT_EQ(rule.os->name, OperatingSystem::OS::OSX);
    ASSERT_EQ(rule.os->arch, Architecture::Arch::X86_64);
    ASSERT_EQ(rule.features, std::nullopt);
}

TEST(ClientHH, MultipleArgumentParsing) {
    rapidjson::Document doc = parse_json_file("E:/OneDrive/Desktop/SynthLauncher/assets/argument.json");

    const rapidjson::Value &arr = doc.GetArray();

    std::vector<Client::Argument> arguments;
    for (rapidjson::Value::ConstValueIterator itr = arr.Begin(); itr != arr.End(); ++itr) {
        arguments.push_back(Client::Argument::parse(*itr));
    }

    ASSERT_EQ(arguments.at(1).value, "--demo");
}

TEST(ClientHH, ArgumentsParsing) {
    rapidjson::Document doc = parse_json_file("E:/OneDrive/Desktop/SynthLauncher/assets/25w03a.json");

    auto obj = doc["arguments"].GetObject();

    Client::Arguments args = Client::Arguments::parse(obj);

    ASSERT_EQ(args.game.at(0).value, "--username");
}

TEST(ClientHH, DownloadParsing) {
    rapidjson::Document doc = parse_json_file("E:/OneDrive/Desktop/SynthLauncher/assets/download.json");
    const rapidjson::Value &obj = doc["download"];
   
    Client::Download download = Client::Download::parse(obj);

    ASSERT_EQ(download.id, "");
    ASSERT_EQ(download.path,
    "org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-windows-arm64.jar");
    ASSERT_EQ(download.sha1, "e9aca8c5479b520a2a7f0d542a118140e812c5e8");
    ASSERT_EQ(download.size, 133378);
    ASSERT_EQ(download.url,
    "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-windows-arm64.jar");
    ASSERT_EQ(download.totalSize, std::nullopt);
}

TEST(ClientHH, ClientDownloadParsing) {
    rapidjson::Document doc = parse_json_file("E:/OneDrive/Desktop/SynthLauncher/assets/client_download.json");
    const rapidjson::Value &obj = doc["downloads"];

    Client::ClientDownloads downloads = Client::ClientDownloads::parse(obj);

    ASSERT_EQ(downloads.client.sha1,
    "9acca901e3564a91250b941cd2c55a55d0b71bca");
    ASSERT_EQ(downloads.client_mappings.sha1,
    "94b753018a4683ec7c25a33c9048d46fbf9a5db0");
    ASSERT_EQ(downloads.server.sha1,
    "e003d151668a0eff64c1191972707655e341f8f5");
    ASSERT_EQ(downloads.server_mappings.sha1,
    "ad7bb6cf9bdb85fd561981e2c4634a9d3292592d");
}

TEST(ClientHH, JavaVersionParsing) {
    rapidjson::Document doc = parse_json_file("E:/OneDrive/Desktop/SynthLauncher/assets/java_version.json");
    const rapidjson::Value &obj = doc["javaVersion"];

    Client::JavaVersion version = Client::JavaVersion::parse(obj);

    ASSERT_EQ(version.component, "java-runtime-delta");
    ASSERT_EQ(version.majorVersion, 21);
}