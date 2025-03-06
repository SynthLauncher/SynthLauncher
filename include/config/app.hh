#pragma once

#include "include/entities/arch.hh"
#include "include/entities/os.hh"
#include "include/httplib.h"
#include "include/json/manifest.hh"
#include <filesystem>
#include <string>

namespace fs = std::filesystem;

/// @brief Contains the struct for the application config and functions to initialize the app
namespace App {
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
void initLauncherDir(const AppConfig &config);
} // namespace App