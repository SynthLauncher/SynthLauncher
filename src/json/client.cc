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