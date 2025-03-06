#pragma once

#define CPPHTTPLIB_OPENSSL_SUPPORT

#include "include/entities/arch.hh"
#include "include/entities/os.hh"
#include "include/httplib.h"
#include "include/json/manifest.hh"
#include <filesystem>
#include <string>

namespace fs = std::filesystem;


/// @brief Configuration for the application (folder structure, OS,
/// architecture)
struct AppConfig {
  std::string DIR;
  fs::path ASSETS_DIR;
  fs::path LIBRARIES_DIR;
  fs::path NATIVES_DIR;
  OperatingSystem::OS OS;
  Architecture::Arch ARCH;
};

/// @brief Initializes the application configuration
AppConfig initAppConfig();
/// @brief Initializes the launcher directory
void initLauncherDir(AppConfig &config);
