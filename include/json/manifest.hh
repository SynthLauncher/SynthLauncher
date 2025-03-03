#pragma once

#include <filesystem>
#include <stdexcept>
#include <string>
#include <vector>
#include <simdjson.h>

#include "include/utils/simdjson_utils.hh"

namespace fs = std::filesystem;

class Manifest {
public:
  static fs::path PATH;

  class Latest {
  public:
    std::string release;
    std::string snapshot;
    static Latest parse(simdjson::ondemand::object &obj);
  };

  class Version {
  public:
    std::string id;
    std::string type;
    std::string url;
    std::string time;
    std::string releaseTime;
    static Version parse(simdjson::ondemand::object &obj);
  };

  Latest latest;
  std::vector<Version> versions;
  static Manifest parse();
};