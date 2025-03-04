#include "include/json/client.hh"

Client::Features Client::Features::parse(const rapidjson::Value &obj) {
  Features features;

  features.isDemoUser =
      obj.HasMember("is_demo_user") && obj["is_demo_user"].GetBool();
  features.hasCustomResolution = obj.HasMember("has_custom_resolution") &&
                                 obj["has_custom_resolution"].GetBool();
  features.hasQuickPlaysSupport = obj.HasMember("has_quick_plays_support") &&
                                  obj["has_quick_plays_support"].GetBool();
  features.isQuickPlaySingleplayer =
      obj.HasMember("is_quick_play_singleplayer") &&
      obj["is_quick_play_singleplayer"].GetBool();
  features.isQuickPlayMultiplayer =
      obj.HasMember("is_quick_play_multiplayer") &&
      obj["is_quick_play_multiplayer"].GetBool();
  features.isQuickPlayRealms = obj.HasMember("is_quick_play_realms") &&
                               obj["is_quick_play_realms"].GetBool();

  return features;
}

Client::OSRules Client::OSRules::parse(const rapidjson::Value &obj) {
  OSRules rules;

  if (obj.HasMember("name"))
    rules.name = OperatingSystem::os_from_string(obj["name"].GetString());
  else
    rules.name = std::nullopt;

  if (obj.HasMember("arch"))
    rules.arch = Architecture::arch_from_string(obj["arch"].GetString());
  else
    rules.arch = std::nullopt;

  if (obj.HasMember("version"))
    rules.version = obj["version"].GetString();
  else
    rules.version = "";

  return rules;
}

Client::Rule Client::Rule::parse(const rapidjson::Value &obj) {
  Client::Rule rule;

  if (obj.HasMember("action"))
    rule.action = obj["action"].GetString();
  else
    rule.action = "disallow";

  if (obj.HasMember("os"))
    rule.os = OSRules::parse(obj["os"]);
  else
    rule.os = std::nullopt;

  if (obj.HasMember("features"))
    rule.features = Features::parse(obj["features"]);
  else
    rule.features = std::nullopt;

  return rule;
}

bool Client::Rule::osMatches(AppConfig &config) {
  bool match = true;

  if (os != std::nullopt) {
    if (os->name != std::nullopt)
      match = (os->name == config.OS);

    if (os->arch != std::nullopt)
      match = match && os->arch == config.ARCH;

    if (action == "allow")
      return match;

    if (action == "disallow")
      return !match;
  }

  return match;
}

bool Client::Rule::osMatches(AppConfig &config, std::vector<Rule> rules) {
  for (auto &rule : rules) {
    if (!rule.osMatches(config))
      return false;
  }

  return true;
}

Client::Argument Client::Argument::parse(const rapidjson::Value &val) {
  Argument arg;

  if (val.IsString()) {
    arg.value = val.GetString();
    return arg;
  }

  if (val.IsObject()) {
    const auto &obj = val;

    if (obj.HasMember("rules")) {
      for (const auto &rule : obj["rules"].GetArray()) {
        arg.rules.push_back(Rule::parse(rule));
      }
    }

    if (obj.HasMember("value")) {
      if (obj["value"].IsString()) {
        arg.value = obj["value"].GetString();
      } else if (obj["value"].IsArray()) {
        for (const auto &val : obj["value"].GetArray()) {
          arg.values.push_back(val.GetString());
        }
      }
    }
  }

  return arg;
}

Client::Arguments Client::Arguments::parse(const rapidjson::Value &obj) {
  Arguments args;

  if (obj.HasMember("game")) {
    for (const auto &arg : obj["game"].GetArray()) {
      args.game.push_back(Argument::parse(arg));
    }
  }

  if (obj.HasMember("jvm")) {
    for (const auto &arg : obj["jvm"].GetArray()) {
      args.jvm.push_back(Argument::parse(arg));
    }
  }

  return args;
}

Client::Download Client::Download::parse(const rapidjson::Value &obj) {
  Client::Download download;

  if (obj.HasMember("id"))
    download.id = obj["id"].GetString();
  else
    download.id = "";

  if (obj.HasMember("path"))
    download.path = obj["path"].GetString();
  else
    download.path = "";

  if (obj.HasMember("totalSize"))
    download.totalSize = obj["totalSize"].GetInt64();
  else
    download.totalSize = std::nullopt;

  if (obj.HasMember("sha1"))
    download.sha1 = obj["sha1"].GetString();
  else
    download.sha1 = "";

  if (obj.HasMember("size"))
    download.size = obj["size"].GetInt64();
  else
    download.size = 0;

  if (obj.HasMember("url"))
    download.url = obj["url"].GetString();
  else
    download.url = "";

  return download;
}

Client::ClientDownloads
Client::ClientDownloads::parse(const rapidjson::Value &obj) {
  Client::ClientDownloads download;

  if (obj.HasMember("client"))
    download.client = Download::parse(obj["client"]);
  else
    download.client = Download();

  if (obj.HasMember("client_mappings"))
    download.client_mappings = Download::parse(obj["client_mappings"]);
  else
    download.client_mappings = Download();

  if (obj.HasMember("server"))
    download.server = Download::parse(obj["server"]);
  else
    download.server = Download();

  if (obj.HasMember("server_mappings"))
    download.server_mappings = Download::parse(obj["server_mappings"]);
  else
    download.server_mappings = Download();

  return download;
}

Client::JavaVersion Client::JavaVersion::parse(const rapidjson::Value &obj) {
  Client::JavaVersion version;

  if (obj.HasMember("component"))
    version.component = obj["component"].GetString();
  else
    version.component = "";

  if (obj.HasMember("majorVersion"))
    version.majorVersion = obj["majorVersion"].GetInt();
  else
    version.majorVersion = 0;

  return version;
}

std::vector<std::uint8_t> Client::Download::fetch() {
  auto [host, path] = httplib_utils::extractHostAndPath(this->url);

  httplib::Client cli(host);

  auto res = cli.Get(path);
  if (!res || res->status != 200)
    throw std::runtime_error("Failed to download " + url);

  const auto &body = res->body;
  return std::vector<uint8_t>(body.begin(), body.end());
}

std::vector<uint8_t> Client::LibraryDownloads::fetchArtifact() {
  return artifact.fetch();
}

fs::path Client::LibraryDownloads::artifactPath(AppConfig &config) {
  return config.LIBRARIES_DIR / artifact.path;
}

std::vector<uint8_t>
Client::LibraryDownloads::fetchNative(std::string nativeIndex) {
  return classifiers.at(nativeIndex).fetch();
}

fs::path Client::LibraryDownloads::nativePath(AppConfig &config,
                                              std::string nativeIndex) {
  Download download = classifiers.at(nativeIndex);

  return config.NATIVES_DIR / download.path;
}

Client::LibraryDownloads Client::LibraryDownloads::parse(const rapidjson::Value &obj) {
  LibraryDownloads downloads;

  if (obj.HasMember("downloads")) {
    if (obj["downloads"].HasMember("artifact")) {
      downloads.artifact = Download::parse(obj["downloads"]["artifact"]);
    }
  }

  /* 
    This field doesn't always show up in the JSON, so we need to check if it exists before parsing it.
  */
  if (obj.HasMember("classifiers")) {
    for (auto itr = obj["classifiers"].MemberBegin(); itr != obj["classifiers"].MemberEnd(); ++itr) {
      auto& key = itr->name;
      auto& val = itr->value;
      downloads.classifiers[key.GetString()] = Download::parse(val);
    }
  }

  return downloads;
}