#pragma once

#include <filesystem>
#include <stdexcept>
#include <string>
#include <vector>
#include "include/rapidjson/document.h"
#include "include/rapidjson/error/en.h"
#include "include/utils/rapidjson_utils.hh"

namespace fs = std::filesystem;

class Manifest {
public:
  static fs::path PATH;

  class Latest {
  public:
    std::string release;
    std::string snapshot;
    static Latest parse(const rapidjson::Value &obj);
  };

  class Version {
  public:
    std::string id;
    std::string type;
    std::string url;
    std::string time;
    std::string releaseTime;
    static Version parse(const rapidjson::Value &obj);
  };

  Latest latest;
  std::vector<Version> versions;
  static Manifest parse();
};