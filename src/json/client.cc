#include "include/json/client.hh"

Client::Features
Client::Features::parse(const rapidjson::Value &obj) {
  Features features;

  features.isDemoUser = obj.HasMember("is_demo_user") && obj["is_demo_user"].GetBool();
  features.hasCustomResolution = obj.HasMember("has_custom_resolution") && obj["has_custom_resolution"].GetBool();
  features.hasQuickPlaysSupport = obj.HasMember("has_quick_plays_support") && obj["has_quick_plays_support"].GetBool();
  features.isQuickPlaySingleplayer = obj.HasMember("is_quick_play_singleplayer") && obj["is_quick_play_singleplayer"].GetBool();
  features.isQuickPlayMultiplayer = obj.HasMember("is_quick_play_multiplayer") && obj["is_quick_play_multiplayer"].GetBool();
  features.isQuickPlayRealms = obj.HasMember("is_quick_play_realms") && obj["is_quick_play_realms"].GetBool();

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

// bool Client::Rule::osMatches(AppConfig &config) { 
//   bool match = true; 

//   if (os != std::nullopt) {
//     if (os->name != std::nullopt) 
//       match = (os->name == config.OS);
    
//     if (os->arch != std::nullopt)
//       match = match && os->arch == config.ARCH;

//     if (action == "allow")
//       return match;

//     if (action == "disallow")
//       return !match;
//   }

//   return match;
// }

// bool Client::Rule::osMatches(AppConfig &config, std::vector<Rule> rules) {
//   for (auto& rule : rules) {
//     if (!rule.osMatches(config))
//       return false;
//   }

//   return true;
// }

Client::Argument Client::Argument::parse(const rapidjson::Value &val) {  
  Argument arg;

  if (val.IsString()) {
    arg.value = val.GetString();
    return arg;
  }

  auto obj = val.GetObject();
 
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

  return arg;
}

// Client::Arguments Client::Arguments::parse(simdjson::ondemand::object &obj) {
//   Arguments args;

//   if (auto game = obj["game"].get_array(); !game.error()) {
//     for (simdjson::ondemand::value arg : game)   
//       args.game.push_back(Argument::parse(arg));
//   }

//   if (auto jvm = obj["jvm"].get_array(); !jvm.error()) {
//     for (simdjson::ondemand::value arg : jvm)
//       args.jvm.push_back(Argument::parse(arg));
//   }

//   return args;
// }

// /*
//   This function works properly I tested it in test/client.cc!
//   The reason that this didn't work with the ClientDownloads parsing is because
//   json is deformed for some reason
// */
// Client::Download Client::Download::parse(simdjson::ondemand::object &obj) {
//   auto sha1 = obj["sha1"].get_string().value();
//   auto size = obj["size"].get_int64().value();
//   auto url = obj["url"].get_string().value();

//   auto id = simdjson_utils::get_with_default<std::string>(obj, "id", "");
//   auto path = simdjson_utils::get_with_default<std::string>(obj, "path", "");
//   auto totalSize = simdjson_utils::get_optional<int64_t>(obj, "totalSize");

//   return {
//     .id = id,
//     .path = path,
//     .sha1 = std::string(sha1),
//     .size = size,
//     .totalSize = totalSize,
//     .url = std::string(url)
//   };
// }

// /*
//   Debugging purposes for now
// */
// Client::ClientDownloads Client::ClientDownloads::parse(simdjson::ondemand::object &obj) {
//   Client::ClientDownloads download;

//   auto client_obj = obj["client"].get_object().value(); 
//   auto client_mappings_obj = obj["client_mappings"].get_object().value();
//   auto server_obj = obj["server"].get_object().value();
//   auto server_mappings_obj = obj["server_mappings"].get_object().value();
  
//   /*
//     Problem arrises here, for some reason the simdjson parser says that the json is deformed in some way
//     which causes it to not be able to parse
//   */
//   /*
//     Another weird notice: on top if I don't do std::cout then this fails entirely 
//   */
//   auto client = Client::Download::parse(client_obj);
//   auto client_mappings = Client::Download::parse(client_mappings_obj);
//   auto server = Client::Download::parse(server_obj);
//   auto server_mappings = Client::Download::parse(server_mappings_obj);

//   download.client = client;
//   download.client_mappings = client_mappings;
//   download.server = server;
//   download.server_mappings = server_mappings;

//   return download;
// }

// std::vector<std::uint8_t> Client::Download::fetch() {
//   auto [host, path] = httplib_utils::extractHostAndPath(this->url);

//   httplib::Client cli(host);

//   auto res = cli.Get(path);
//   if (!res || res->status != 200) 
//     throw std::runtime_error("Failed to download " + url);

//   const auto &body = res->body;
//   return std::vector<uint8_t>(body.begin(), body.end());
// }

// Client::JavaVersion Client::JavaVersion::parse(simdjson::ondemand::object &obj) {
//   return { 
//     .component = simdjson_utils::get<std::string>(obj, "component"),
//     .majorVersion = simdjson_utils::get<int>(obj, "majorVersion") 
//   };
// }

// std::vector<uint8_t> Client::LibraryDownloads::fetchArtifact() {
//   return artifact.fetch();
// }

// fs::path Client::LibraryDownloads::artifactPath(AppConfig &config) {
//   return config.LIBRARIES_DIR / artifact.path;
// }

// std::vector<uint8_t> Client::LibraryDownloads::fetchNative(std::string nativeIndex) {
//   return classifiers.at(nativeIndex).fetch();
// }

// fs::path Client::LibraryDownloads::nativePath(AppConfig &config, std::string nativeIndex) {
//   Download download = classifiers.at(nativeIndex);

//   return config.NATIVES_DIR / download.path;
// }