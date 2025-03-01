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
  auto name = simdjson_utils::get_optional<std::string>(obj, "name");
  auto arch = simdjson_utils::get_optional<std::string>(obj, "arch");
  auto version =
      simdjson_utils::get_with_default<std::string>(obj, "version", "");

  return {
      .name =
          (name != std::nullopt)
              ? std::optional<
                    OperatingSystem::OS>{OperatingSystem::os_from_string(*name)}
              : std::nullopt,
      .arch =
          (arch != std::nullopt)
              ? std::optional<
                    Architecture::Arch>{Architecture::arch_from_string(*arch)}
              : std::nullopt,
      .version = version};
}