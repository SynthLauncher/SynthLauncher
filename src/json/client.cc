#include "include/json/client.hh"

using namespace simdjson_utils;

bool Client::Rule::osMatches(AppConfig &config) { 
  bool match = true; 

  if (os != std::nullopt) {
    if (os.value().name != std::nullopt)
      match = (os.value().name == config.OS);

    if (os.value().arch != std::nullopt)
      match = (match && os.value().arch == config.ARCH);

    if (action == "allow") return match;

    if (action == "disallow") return !match;
  }
}

bool Client::Rule::osMatches(AppConfig &config, std::vector<Rule> rules) {
  for (Rule rule : rules) {
    if (!rule.osMatches(config))
      return false;
  }

  return true;
}

Client::Features Client::ArgumentDeserializer::deserialize_features(
    simdjson::ondemand::object &obj) 
{
    Client::Features features;

    features.isDemoUser = get_with_default<bool>(obj, "is_demo_user", false);
    features.hasCustomResolution = get_with_default<bool>(obj, "has_custom_resolution", false);
    features.hasQuickPlaysSupport = get_with_default<bool>(obj, "has_quick_plays_support", false);
    features.isQuickPlaySingleplayer = get_with_default<bool>(obj, "is_quick_play_singleplayer", false);
    features.isQuickPlayMultiplayer = get_with_default<bool>(obj, "is_quick_play_multiplayer", false);
    features.isQuickPlayRealms = get_with_default<bool>(obj, "is_quick_play_realms", false);

    return features;
}

Client::OSRules Client::ArgumentDeserializer::deserialize_os_rules(simdjson::ondemand::object &obj) {
    Client::OSRules rules;
    
    if (auto name = get_optional<std::string>(obj, "name")) rules.name = OperatingSystem::os_from_string(*name);
    
    if (auto arch = get_optional<std::string>(obj, "arch")) rules.arch = Architecture::arch_from_string(*arch);
    
    rules.version = get_with_default(obj, "version", "");
    
    return rules;
}

Client::Rule Client::ArgumentDeserializer::deserialize_rule(simdjson::ondemand::object &obj) {
  Client::Rule rule;

  if (auto features = get_optional<simdjson::ondemand::object>(obj, "features")) rule.features = deserialize_features(*features);

  if (auto os = get_optional<simdjson::ondemand::object>(obj, "os")) rule.os = deserialize_os_rules(*os);

  rule.action = get_with_default<std::string>(obj, "action", "disallow");

  return rule;
}