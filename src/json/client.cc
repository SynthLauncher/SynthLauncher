#include "include/json/client.hh"

Client::Features Client::ArgumentDeserializer::deserialize_features(simdjson::ondemand::object &obj) {
  Client::Features features;

  features.isDemoUser = obj["is_demo_user"].get_bool().value();
  features.hasCutomResolution = obj["has_custom_resolution"].get_bool().value();
  features.hasQuickPlaysSupport = obj["has_quick_plays_support"].get_bool().value();
  features.isQuickPlaySingleplayer = obj["is_quick_play_singleplayer"].get_bool().value();
  features.isQuickPlayMultiplayer = obj["is_quick_play_multiplayer"].get_bool().value();
  features.isQuickPlayRealms = obj["is_quick_play_realms"].get_bool().value();

  return features;
}

Client::OSRules Client::ArgumentDeserializer::deserialize_os_rules(simdjson::ondemand::object &obj) {
  Client::OSRules rules;

  std::string_view name = obj["name"];
  
}

Client::Rule Client::ArgumentDeserializer::deserialize_rule(simdjson::ondemand::object &obj) {

}