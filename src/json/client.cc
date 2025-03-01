#include "include/json/client.hh"

// Deserializes features
Client::Features
Client::Features::deserialize(simdjson::ondemand::object &obj) {
  return {.isDemoUser = simdjson_utils::get_with_default<bool>(
              obj, "is_demo_user", false),
          .hasCustomResolution = simdjson_utils::get_with_default<bool>(
              obj, "has_custom_resolution", false),
          .hasQuickPlaysSupport = simdjson_utils::get_with_default<bool>(
              obj, "has_quick_plays_support", false),
          .isQuickPlaySingleplayer = simdjson_utils::get_with_default<bool>(
              obj, "is_quick_play_singleplayer", false),
          .isQuickPlayMultiplayer = simdjson_utils::get_with_default<bool>(
              obj, "is_quick_play_multiplayer", false),
          .isQuickPlayRealms = simdjson_utils::get_with_default<bool>(
              obj, "is_quick_play_realms", false)};
}

Client::OSRules Client::OSRules::deserialize(simdjson::ondemand::object &obj) {
  std::optional<OperatingSystem::OS> name;
  if (auto name_obj = simdjson_utils::get_optional<std::string>(obj, "name"))
    name = OperatingSystem::os_from_string(*name_obj);

  std::optional<Architecture::Arch> arch;
  if (auto arch_obj = simdjson_utils::get_optional<std::string>(obj, "arch"))
    arch = Architecture::arch_from_string(*arch_obj);

  auto version =
      simdjson_utils::get_with_default<std::string>(obj, "version", "");

  return {.name = name, .arch = arch, .version = version};
}

Client::Rule Client::Rule::deserialize(simdjson::ondemand::object &obj) {
  std::string action =
      simdjson_utils::get_with_default<std::string>(obj, "action", "disallow");

  std::optional<OSRules> os;
  if (auto os_obj =
          simdjson_utils::get_optional<simdjson::ondemand::object>(obj, "os"))
    os = OSRules::deserialize(*os_obj);

  std::optional<Features> features;
  if (auto features_obj =
          simdjson_utils::get_optional<simdjson::ondemand::object>(obj,
                                                                   "features"))
    features = Features::deserialize(*features_obj);

  return {.action = action, .os = os, .features = features};
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
  for (auto& rule : rules) {
    if (!rule.osMatches(config))
      return false;
  }

  return true;
}

Client::Argument Client::Argument::deserialize(simdjson::ondemand::value& val) {  
  Argument arg;

  if (val.type() == simdjson::ondemand::json_type::string) {
    arg.value = std::string(val.get_string().value());
    return arg;
  }

  simdjson::ondemand::object obj = val.get_object().value();

  if (auto rules = simdjson_utils::get_optional<simdjson::ondemand::array>(obj, "rules")) {
    for (simdjson::ondemand::value rule_value : *rules) {
      auto rule_obj = rule_value.get_object().value();
      arg.rules.push_back(Rule::deserialize(rule_obj));
    }
  }

  if (auto value = obj["value"]; !value.error()) {
    if (value.type() == simdjson::ondemand::json_type::string) 
      arg.value = std::string(value.get_string().value());
    else if (value.type() == simdjson::ondemand::json_type::array) {
      for (simdjson::ondemand::value elem : value.get_array())   
        arg.values.push_back(std::string(elem.get_string().value()));
    }
  }

  return arg;
}

Client::Arguments Client::Arguments::deserialize(simdjson::ondemand::object &obj) {
  Arguments args;

  if (auto game = obj["game"].get_array(); !game.error()) {
    for (simdjson::ondemand::value arg : game)   
      args.game.push_back(Argument::deserialize(arg));
  }

  if (auto jvm = obj["jvm"].get_array(); !jvm.error()) {
    for (simdjson::ondemand::value arg : jvm)
      args.jvm.push_back(Argument::deserialize(arg));
  }

  return args;
}

Client::Download Client::Download::deserialize(simdjson::ondemand::object &obj) {
  auto sha1 = std::string(obj["sha1"].get_string().value());
  auto size = obj["size"].get_int64().value();
  auto url = std::string(obj["url"].get_string().value());

  auto id = simdjson_utils::get_with_default<std::string>(obj, "id", "");
  auto path = simdjson_utils::get_with_default<std::string>(obj, "path", "");
  auto totalSize = simdjson_utils::get_optional<int64_t>(obj, "totalSize");

  return {
    .id = id,
    .path = path,
    .sha1 = sha1,
    .size = size,
    .totalSize = totalSize,
    .url = url
  };
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

Client::ClientDownloads Client::ClientDownloads::deserialize(simdjson::ondemand::object &obj) {
  auto client = obj["client"].get_object().value();
  auto client_mappings = obj["client_mappings"].get_object().value();
  auto server = obj["server"].get_object().value();
  auto server_mappings = obj["server_mappings"].get_object().value();

  return {
    .client = Client::Download::deserialize(client),
    .client_mappings = Client::Download::deserialize(client_mappings),
    .server = Client::Download::deserialize(server),
    .server_mappings = Client::Download::deserialize(server_mappings)
  };
}

Client::JavaVersion Client::JavaVersion::deserialize(simdjson::ondemand::object &obj) {
  return { 
    .component = simdjson_utils::get<std::string>(obj, "component"),
    .majorVersion = simdjson_utils::get<int>(obj, "majorVersion") 
  };
}

std::vector<uint8_t> Client::LibraryDownloads::fetchArtifact() {
  return artifact.fetch();
}

fs::path Client::LibraryDownloads::artifactPath(AppConfig &config) {
  return config.LIBRARIES_DIR / artifact.path;
}

std::vector<uint8_t> Client::LibraryDownloads::fetchNative(std::string nativeIndex) {
  return classifiers.at(nativeIndex).fetch();
}

fs::path Client::LibraryDownloads::nativePath(AppConfig &config, std::string nativeIndex) {
  Download download = classifiers.at(nativeIndex);

  return config.NATIVES_DIR / download.path;
}