#pragma once

#include <filesystem>
#include <simdjson.h>
#include <stdexcept>
#include <string>
#include <vector>

namespace fs = std::filesystem;

class Manifest {
public:
  static fs::path PATH;

  class Latest {
  public:
    std::string release;
    std::string snapshot;
    static Latest parse_latest(simdjson::ondemand::object &obj);
  };

  class Version {
  public:
    std::string id;
    std::string type;
    std::string url;
    std::string time;
    std::string releaseTime;
    static Version parse_version(simdjson::ondemand::object &obj);
  };

  Latest latest;
  std::vector<Version> versions;
  static Manifest parse_maniftest(const std::string &path);
};