#pragma once

#include <filesystem>
#include <stdexcept>
#include <string>
#include <vector>
#include "include/rapidjson/document.h"
#include "include/rapidjson/error/en.h"
#include "include/json/manifest.hh"
#include "include/config/app.hh"
#include "include/httplib.h"
#include "include/utils/httplib_utils.hh"
#include "include/config/config.hh"
#include "include/json/client.hh"

namespace fs = std::filesystem;

class Config;

class Instance {
public:
  static fs::path PARENT_DIR;
  static fs::path INSTANCE_FILE;

  std::string name;
  std::string version;
  fs::path icon;

  Instance(const std::string &name, const std::string &version);
  fs::path dir();
  void initDir();
  void init(App::AppConfig &config);
  static Instance createInstance(const std::string &name,
                                 const std::string &version);
  static std::vector<Instance> readInstances();
  static Instance getInstance(const std::string &name);
  static void writeInstance(Instance& instance);
  static void updateInstance(Instance& instance);
  static void addInstance(Instance& instance);
  Config getConfig();
  Client readClient();
  void install(App::AppConfig &config);
  void launch(App::AppConfig &config);
  static Instance parse(const rapidjson::Value &obj);
  static std::string toJson(Instance &instance);
};