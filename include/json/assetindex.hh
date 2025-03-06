#pragma once

#include "include/config/app.hh"
#include "include/httplib.h"
#include "include/rapidjson/document.h"
#include <filesystem>
#include <map>
#include <string>

namespace fs = std::filesystem;

/// @brief Asset Index for the game
class AssetIndex {
public:
  /// @brief Asset Object
  class AssetObject {
  public:
    std::string hash;

    /// @brief Returns the ID of the asset
    std::string id();

    /// @brief Returns the URL of the asset
    std::string url();

    /// @brief Returns the path of the asset
    fs::path path(App::AppConfig &config);

    /// @brief Fetches the asset
    void fetch(App::AppConfig &config);

    static AssetObject fromJson(const rapidjson::Value &obj);
  };

  static AssetIndex fromJson(const rapidjson::Value &obj);
  std::map<std::string, AssetObject> objects;
};