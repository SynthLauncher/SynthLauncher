#pragma once
#define CPPHTTPLIB_OPENSSL_SUPPORT

#include <filesystem>
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
  inline static fs::path PARENT_DIR;
  inline static fs::path INSTANCE_FILE;

  std::string name;
  std::string version;
  fs::path icon;

  Instance();
  Instance(const std::string &name, const std::string &version);

  fs::path dir();
  void initDir();
  static void init(const App::AppConfig &config);
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
  
  static Instance fromJson(const rapidjson::Value &obj);
  std::string toJson() const;
};