#include "include/json/client.hh"
#include "include/utils/rapidjson_utils.hh"
#include <gtest/gtest.h>

TEST(ClientHH, FeaturesParsing_1) {
  rapidjson::Document doc = rapidjson_utils::fromJson(
      "E:/OneDrive/Desktop/SynthLauncher/assets/features/features_1.json");
  const rapidjson::Value &obj = doc["features"];

  Client::Features features = Client::Features::fromJson(obj);

  ASSERT_EQ(features.isDemoUser, true);
  ASSERT_EQ(features.hasQuickPlaysSupport, true);
  ASSERT_EQ(features.hasCustomResolution, false);
  ASSERT_EQ(features.isQuickPlayMultiplayer, false);
  ASSERT_EQ(features.isQuickPlayRealms, false);
  ASSERT_EQ(features.isQuickPlaySingleplayer, false);
}

TEST(ClientHH, FeaturesParsing_2) {
  rapidjson::Document doc = rapidjson_utils::fromJson(
      "E:/OneDrive/Desktop/SynthLauncher/assets/features/features_2.json");
  const rapidjson::Value &obj = doc["features"];

  Client::Features features = Client::Features::fromJson(obj);

  ASSERT_EQ(features.isDemoUser, false);
  ASSERT_EQ(features.hasQuickPlaysSupport, true);
  ASSERT_EQ(features.hasCustomResolution, false);
  ASSERT_EQ(features.isQuickPlayMultiplayer, true);
  ASSERT_EQ(features.isQuickPlayRealms, true);
  ASSERT_EQ(features.isQuickPlaySingleplayer, false);
}

TEST(ClientHH, FeaturesParsing_3) {
  rapidjson::Document doc = rapidjson_utils::fromJson(
      "E:/OneDrive/Desktop/SynthLauncher/assets/features/features_3.json");
  const rapidjson::Value &obj = doc["features"];

  Client::Features features = Client::Features::fromJson(obj);

  ASSERT_EQ(features.isDemoUser, false);
  ASSERT_EQ(features.hasQuickPlaysSupport, true);
  ASSERT_EQ(features.hasCustomResolution, false);
  ASSERT_EQ(features.isQuickPlayMultiplayer, true);
  ASSERT_EQ(features.isQuickPlayRealms, false);
  ASSERT_EQ(features.isQuickPlaySingleplayer, false);
}

TEST(ClientHH, OSRulesParsing_1) {
  rapidjson::Document doc = rapidjson_utils::fromJson(
      "E:/OneDrive/Desktop/SynthLauncher/assets/os_rules/os_rules_1.json");
  const rapidjson::Value &obj = doc["os"];

  Client::OSRules rules = Client::OSRules::fromJson(obj);

  ASSERT_EQ(rules.name, OperatingSystem::OS::Windows);
  ASSERT_EQ(rules.arch, std::nullopt);
  ASSERT_EQ(rules.version, "");
}

TEST(ClientHH, OSRulesParsing_2) {
  rapidjson::Document doc = rapidjson_utils::fromJson(
      "E:/OneDrive/Desktop/SynthLauncher/assets/os_rules/os_rules_2.json");
  const rapidjson::Value &obj = doc["os"];

  Client::OSRules rules = Client::OSRules::fromJson(obj);

  ASSERT_EQ(rules.arch, Architecture::Arch::X86);
  ASSERT_EQ(rules.version, "1.21");
  ASSERT_EQ(rules.name, std::nullopt);
}

TEST(ClientHH, OSRulesParsing_3) {
  rapidjson::Document doc = rapidjson_utils::fromJson(
      "E:/OneDrive/Desktop/SynthLauncher/assets/os_rules/os_rules_3.json");
  const rapidjson::Value &obj = doc["os"];

  Client::OSRules rules = Client::OSRules::fromJson(obj);

  ASSERT_EQ(rules.name, OperatingSystem::OS::Linux);
  ASSERT_EQ(rules.arch, Architecture::Arch::Arm);
  ASSERT_EQ(rules.version, "");
}

TEST(ClientHH, RuleParsing_1) {
  rapidjson::Document doc = rapidjson_utils::fromJson(
      "E:/OneDrive/Desktop/SynthLauncher/assets/rule/rule_1.json");

  const rapidjson::Value &arr = doc["rules"];

  /*
    !!! Important: "rules" has only 1 rule object and that is why we do it in
    this hard coded way
  */
  for (const auto &obj : arr.GetArray()) {
    Client::Rule rule = Client::Rule::fromJson(obj);

    ASSERT_EQ(rule.action, "allow");
    ASSERT_EQ(rule.os->name, OperatingSystem::OS::OSX);
    ASSERT_EQ(rule.os->arch, Architecture::Arch::X86_64);
    ASSERT_EQ(rule.features, std::nullopt);
  }
}

TEST(ClientHH, RuleParsing_2) {
  rapidjson::Document doc = rapidjson_utils::fromJson(
      "E:/OneDrive/Desktop/SynthLauncher/assets/rule/rule_2.json");

  const rapidjson::Value &arr = doc["rules"];

  /*
    !!! Important: "rules" has only 1 rule object and that is why we do it in
    this hard coded way
  */
  for (const auto &obj : arr.GetArray()) {
    Client::Rule rule = Client::Rule::fromJson(obj);

    ASSERT_EQ(rule.action, "disallow");
    ASSERT_EQ(rule.os->name, OperatingSystem::OS::Windows);
    ASSERT_EQ(rule.os->version, "1.21");
    ASSERT_EQ(rule.os->arch, std::nullopt);
    ASSERT_EQ(rule.features, std::nullopt);
  }
}

TEST(ClientHH, RuleParsing_3) {
  rapidjson::Document doc = rapidjson_utils::fromJson(
      "E:/OneDrive/Desktop/SynthLauncher/assets/rule/rule_3.json");

  const rapidjson::Value &arr = doc["rules"];

  /*
    !!! Important: "rules" has only 1 rule object and that is why we do it in
    this hard coded way
  */
  for (const auto &obj : arr.GetArray()) {
    Client::Rule rule = Client::Rule::fromJson(obj);

    ASSERT_EQ(rule.action, "allow");
    ASSERT_EQ(rule.features->isDemoUser, true);
    ASSERT_EQ(rule.features->hasCustomResolution, true);
    ASSERT_EQ(rule.features->hasQuickPlaysSupport, false);
    ASSERT_EQ(rule.features->isQuickPlayMultiplayer, false);
    ASSERT_EQ(rule.features->isQuickPlayRealms, false);
    ASSERT_EQ(rule.features->isQuickPlaySingleplayer, false);
    ASSERT_EQ(rule.os, std::nullopt);
  }
}

TEST(ClientHH, ArgumentParsing_1) {
  rapidjson::Document doc = rapidjson_utils::fromJson(
      "E:/OneDrive/Desktop/SynthLauncher/assets/argument/argument_1.json");

  const rapidjson::Value &arr = doc;

  std::vector<Client::Argument> arguments;
  for (const auto &obj : arr.GetArray()) {
    arguments.push_back(Client::Argument::fromJson(obj));
  }

  ASSERT_EQ(arguments[0].value, "${version_type}");
}

TEST(ClientHH, ArgumentParsing_2) {
  rapidjson::Document doc = rapidjson_utils::fromJson(
    "E:/OneDrive/Desktop/SynthLauncher/assets/argument/argument_2.json");

  const rapidjson::Value &arr = doc;

  std::vector<Client::Argument> arguments;
  for (const auto &obj : arr.GetArray()) {
    arguments.push_back(Client::Argument::fromJson(obj));
  }

  ASSERT_EQ(arguments[0].rules[0].action, "allow");
  ASSERT_EQ(arguments[0].rules[0].features->isDemoUser, true);
  ASSERT_EQ(arguments[0].value, "--demo");
}

TEST(ClientHH, ArgumentParsing_3) {
  rapidjson::Document doc = rapidjson_utils::fromJson(
    "E:/OneDrive/Desktop/SynthLauncher/assets/argument/argument_3.json");

  const rapidjson::Value &arr = doc;

  std::vector<Client::Argument> arguments;
  for (const auto &obj : arr.GetArray()) {
    arguments.push_back(Client::Argument::fromJson(obj));
  }

  ASSERT_EQ(arguments[0].values[0], "--width");
  ASSERT_EQ(arguments[0].values[1], "${resolution_width}");
  ASSERT_EQ(arguments[0].values[2], "--height");
  ASSERT_EQ(arguments[0].values[3], "${resolution_height}");
  ASSERT_EQ(arguments[0].rules[0].action, "allow");
  ASSERT_EQ(arguments[0].rules[0].features->hasCustomResolution, true);
}

TEST(ClientHH, ArgumentsParsing) {
    rapidjson::Document doc =
    rapidjson_utils::fromJson("E:/OneDrive/Desktop/SynthLauncher/assets/arguments/arguments.json");

    const rapidjson::Value& obj = doc["arguments"];

    Client::Arguments args = Client::Arguments::fromJson(obj);

    ASSERT_EQ(args.game[0].value, "--username");
    ASSERT_EQ(args.game[1].value, "${auth_player_name}");
    ASSERT_EQ(args.game[2].value, "--version");
    ASSERT_EQ(args.game[3].value, "${version_name}");
    ASSERT_EQ(args.jvm[0].rules[0].action, "allow");
    ASSERT_EQ(args.jvm[0].rules[0].os->name, OperatingSystem::OS::Windows);
    ASSERT_EQ(args.jvm[0].value, "-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump");
    ASSERT_EQ(args.jvm[1].value, "-Djava.library.path=${natives_directory}");
}

TEST(ClientHH, DownloadParsing_1) {
    rapidjson::Document doc =
    rapidjson_utils::fromJson("E:/OneDrive/Desktop/SynthLauncher/assets/download/download_1.json");
    const rapidjson::Value &obj = doc["download"];

    Client::Download download = Client::Download::fromJson(obj);

    ASSERT_EQ(download.path, "org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-windows-arm64.jar");
    ASSERT_EQ(download.sha1, "e9aca8c5479b520a2a7f0d542a118140e812c5e8");
    ASSERT_EQ(download.size, 133378);
    ASSERT_EQ(download.url, "https://libraries.minecraft.net/org/lwjgl/lwjgl/3.3.3/lwjgl-3.3.3-natives-windows-arm64.jar");
    ASSERT_EQ(download.id, "");
    ASSERT_EQ(download.totalSize, 0);
}

TEST(ClientHH, DownloadParsing_2) {
  rapidjson::Document doc =
  rapidjson_utils::fromJson("E:/OneDrive/Desktop/SynthLauncher/assets/download/download_2.json");
  const rapidjson::Value &obj = doc["download"];

  Client::Download download = Client::Download::fromJson(obj);

  ASSERT_EQ(download.sha1, "e003d151668a0eff64c1191972707655e341f8f5");
  ASSERT_EQ(download.size, 57017689);
  ASSERT_EQ(download.url, "https://piston-data.mojang.com/v1/objects/e003d151668a0eff64c1191972707655e341f8f5/server.jar");
  ASSERT_EQ(download.id, "123k");
  ASSERT_EQ(download.totalSize, 0);
  ASSERT_EQ(download.path, "");
}

TEST(ClientHH, DownloadParsing_3) {
  rapidjson::Document doc =
  rapidjson_utils::fromJson("E:/OneDrive/Desktop/SynthLauncher/assets/download/download_3.json");
  const rapidjson::Value &obj = doc["download"];

  Client::Download download = Client::Download::fromJson(obj);

  ASSERT_EQ(
    download.path,
      "com/fasterxml/jackson/core/jackson-core/2.13.4/jackson-core-2.13.4.jar");
  ASSERT_EQ(download.sha1, "0cf934c681294b97ef6d80082faeefbe1edadf56");
  ASSERT_EQ(download.totalSize, 29323);
  ASSERT_EQ(download.id, "");
  ASSERT_EQ(download.size, 0);
  ASSERT_EQ(download.url, "");
}

TEST(ClientHH, ClientDownloadsParsing) {
  rapidjson::Document doc = rapidjson_utils::fromJson("E:/OneDrive/Desktop/SynthLauncher/assets/client_download/client_download.json");
  rapidjson::Value &obj = doc["downloads"];

  Client::ClientDownloads downloads = Client::ClientDownloads::fromJson(obj);

  ASSERT_EQ(downloads.client.sha1, "9acca901e3564a91250b941cd2c55a55d0b71bca");
  ASSERT_EQ(downloads.client.size, 28534222);
  ASSERT_EQ(downloads.client.url, "https://piston-data.mojang.com/v1/objects/9acca901e3564a91250b941cd2c55a55d0b71bca/client.jar");
  
  ASSERT_EQ(downloads.client_mappings.sha1, "94b753018a4683ec7c25a33c9048d46fbf9a5db0");
  ASSERT_EQ(downloads.client_mappings.size, 10413606);
  ASSERT_EQ(downloads.client_mappings.url, "https://piston-data.mojang.com/v1/objects/94b753018a4683ec7c25a33c9048d46fbf9a5db0/client.txt");

  ASSERT_EQ(downloads.server.sha1, "e003d151668a0eff64c1191972707655e341f8f5");
  ASSERT_EQ(downloads.server.size, 57017689);
  ASSERT_EQ(downloads.server.url, "https://piston-data.mojang.com/v1/objects/e003d151668a0eff64c1191972707655e341f8f5/server.jar");

  ASSERT_EQ(downloads.server_mappings.sha1, "ad7bb6cf9bdb85fd561981e2c4634a9d3292592d");
  ASSERT_EQ(downloads.server_mappings.size, 7824495);
  ASSERT_EQ(downloads.server_mappings.url,  "https://piston-data.mojang.com/v1/objects/ad7bb6cf9bdb85fd561981e2c4634a9d3292592d/server.txt");
}

TEST(ClientHH, JavaVersionParsing) {
    rapidjson::Document doc =
    rapidjson_utils::fromJson("E:/OneDrive/Desktop/SynthLauncher/assets/java_version/java_version.json");
    const rapidjson::Value &obj = doc["javaVersion"];

    Client::JavaVersion version = Client::JavaVersion::fromJson(obj);

    ASSERT_EQ(version.component, "java-runtime-delta");
    ASSERT_EQ(version.majorVersion, 21);
}


