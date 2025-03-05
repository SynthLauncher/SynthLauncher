#pragma once

#include <cstdint>
#include <stdexcept>
#include <filesystem>
#include "include/config/java.hh"

#ifdef _WIN32
#include <windows.h>
#elif defined(__APPLE__)
#include <sys/sysctl.h>
#else
#include <sys/sysinfo.h>
#endif

namespace fs = std::filesystem;

class Config {
public:
  static fs::path MAIN_PATH;

  uint64_t getMinRam() const;
  uint64_t getMaxRam() const;
  Java getJava() const;
  void setMinRam(uint64_t min_ram);
  void setMaxRam(uint64_t max_ram);
  void setJava(Java java);
  Config(fs::path path);

private:
  std::string path;
  uint64_t min_ram;
  uint64_t max_ram;
  Java java;

  static uint64_t getTotalPhysicalMemory();
  Config();
};