#include "include/json/client.hh"

template<typename T>
T Client::ArgumentDeserializer::check_and_return(simdjson::ondemand::object &obj, std::string_view field_name, T default_value) {
    auto field = obj[field_name];
    
    if (field.error()) return default_value;
    
    
    T result;
    auto error = field.get(result);
    
    if (error == simdjson::SUCCESS) return result;
    
    return default_value;
}

Client::Features Client::ArgumentDeserializer::deserialize_features(simdjson::ondemand::object &obj) {
  Client::Features features;

  features.isDemoUser = check_and_return(obj, "is_demo_user", false);
  features.hasQuickPlaysSupport = check_and_return(obj, "has_quick_plays_support", false);
  features.hasCutomResolution = check_and_return(obj, "has_custom_resolution", false);
  features.isQuickPlayMultiplayer = check_and_return(obj, "is_quick_play_multiplayer", false);
  features.isQuickPlayRealms = check_and_return(obj, "is_quick_play_realms", false);
  features.isQuickPlaySingleplayer = check_and_return(obj, "is_quick_play_singleplayer", false);

  return features;
}

Client::OSRules Client::ArgumentDeserializer::deserialize_os_rules(
  simdjson::ondemand::object &obj) {
  Client::OSRules rules;
  
  if (auto name = obj["name"].get_string(); !name.error()) {
    std::string str_name = std::string(name.value_unsafe());
    rules.name = OperatingSystem::os_from_string(str_name);
  }
  
  if (auto arch = obj["arch"].get_string(); !arch.error()) {
    std::string str_arch = std::string(arch.value_unsafe());
    rules.arch = Architecture::arch_from_string(str_arch);
  }
  
  if (auto version = obj["version"].get_string(); !version.error()) {
      rules.version = std::string(version.value_unsafe());
  }
  
  return rules;
}

Client::Rule Client::ArgumentDeserializer::deserialize_rule(
  simdjson::ondemand::object &obj) {
  Client::Rule rule;
  
  if (auto features = obj["features"].get_object(); !features.error()) {
      rule.features = deserialize_features(features.value_unsafe());
  }
  
  if (auto os = obj["os"].get_object(); !os.error()) {
      rule.os = deserialize_os_rules(os.value_unsafe());
  }
  
  if (auto action = obj["action"].get_string(); !action.error()) {
      rule.action = std::string(action.value_unsafe());
  }
  
  return rule;
}

