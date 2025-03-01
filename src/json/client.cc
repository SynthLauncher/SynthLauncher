#include "include/json/client.hh"

// Deserializes features
Client::Features Client::Features::deserialize(simdjson::ondemand::object& obj) {
  return {
    .isDemoUser = simdjson_utils::get_with_default<bool>(obj, "is_demo_user", false),
    .hasCustomResolution = simdjson_utils::get_with_default<bool>(obj, "has_custom_resolution", false),
    .hasQuickPlaysSupport = simdjson_utils::get_with_default<bool>(obj, "has_quick_plays_support", false),
    .isQuickPlaySingleplayer = simdjson_utils::get_with_default<bool>(obj, "is_quick_play_singleplayer", false),
    .isQuickPlayMultiplayer = simdjson_utils::get_with_default<bool>(obj, "is_quick_play_multiplayer", false),
    .isQuickPlayRealms = simdjson_utils::get_with_default<bool>(obj, "is_quick_play_realms", false)
  };
}