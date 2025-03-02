#pragma once

#define CPPHTTPLIB_OPENSSL_SUPPORT

#include "include/config/app.hh"
#include "include/httplib.h"
#include <filesystem>
#include <map>
#include <string>

namespace fs = std::filesystem;

class AssetIndex {
public:
  class AssetObject {
  public:
    std::string hash;

    std::string id();
    std::string url();
    fs::path path(AppConfig &config);
    void fetch(AppConfig &config);
  };

  std::map<std::string, AssetObject> objects;
};