#pragma once

#include <filesystem>
#include <stdexcept>
#include <string>
#include <vector>
#include "include/rapidjson/document.h"
#include "include/rapidjson/error/en.h"
#include "include/utils/rapidjson_utils.hh"

namespace fs = std::filesystem;

/// @brief Manifest for the game
class Manifest {
public:
  static fs::path PATH;

  /// @brief Latest version of the game
  class Latest {
  public:
    std::string release;
    std::string snapshot;
    static Latest fromJson(const rapidjson::Value &obj);
  };

  /// @brief Version of the game
  class Version {
  public:
    std::string id;
    std::string type;
    std::string url;
    std::string time;
    std::string releaseTime;
    static Version fromJson(const rapidjson::Value &obj);
  };

  Latest latest;
  std::vector<Version> versions;
  /// @brief Parses the manifest
  static Manifest fromJson();
};