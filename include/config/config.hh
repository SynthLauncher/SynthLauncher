#pragma once
#define CPPHTTPLIB_OPENSSL_SUPPORT

#include <cstdint>
#include <vector>
#include <stdexcept>
#include <filesystem>
#include "include/rapidjson/document.h"
#include "include/config/java.hh"
#include "include/utils/rapidjson_utils.hh"
#include "include/entities/instance.hh"

#ifdef _WIN32
#include <winsock2.h>
#include <windows.h>
#elif defined(__APPLE__)
#include <sys/sysctl.h>
#else
#include <sys/sysinfo.h>
#endif

namespace fs = std::filesystem;

class Instance;

class Config {
public:
  inline static fs::path MAIN_PATH = "";
  
  Config(const fs::path& path);
  Config(const Java &java, const fs::path& path, const uint64_t& min_ram, const uint64_t& max_ram);
  std::string toJson();
  static Config parse(const rapidjson::Value &obj);

  uint64_t getMinRam() const;
  uint64_t getMaxRam() const;
  Java getJava() const;
  void setMinRam(uint64_t min_ram);
  void setMaxRam(uint64_t max_ram);
  void setJava(Java java);
  static Config getConfig(fs::path);
  Config readMainConfig();
  void launch(App::AppConfig &config, Instance &instance);

private:
  fs::path path;
  uint64_t min_ram;
  uint64_t max_ram;
  Java java;

  void writeConfig(); 
  static uint64_t getTotalPhysicalMemory();
  Config();
};