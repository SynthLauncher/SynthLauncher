#include "include/config/app.hh"

std::ostream &operator<<(std::ostream &os, const AppConfig &config) {
  os << "DIR: " << config.DIR << '\n'
     << "ASSETS_DIR: " << config.ASSETS_DIR << '\n'
     << "LIBRARIES_DIR: " << config.LIBRARIES_DIR << '\n'
     << "NATIVES_DIR: " << config.NATIVES_DIR << '\n'
     << "OS: " << OperatingSystem::os_to_string(config.OS) << '\n'
     << "ARCH: " << Architecture::arch_to_string(config.ARCH) << '\n';
  return os;
}

AppConfig initializeAppConfig() {
  AppConfig config;

// OS Configuration
#ifdef _WIN32
  config.OS = OperatingSystem::OS::Windows;
  const char *appData = std::getenv("APPDATA");
  if (appData) {
    config.DIR = std::string(appData) + "\\SynthLauncher";
  } else {
    config.DIR = "C:\\SynthLauncher";
  }
#elif __linux__
  config.OS = OperatingSystem::OS::Linux;
  const char *home = std::getenv("HOME");
  if (home) {
    config.DIR = std::string(home) + "/.synthlauncher";
  } else {
    config.DIR = "/usr/local/synthlauncher";
  }
#elif __APPLE__
  config.OS = OperatingSystem::OS::OSX;
  const char *home = std::getenv("HOME");
  if (home) {
    config.DIR =
        std::string(home) + "/Library/Application Support/SynthLauncher";
  } else {
    config.DIR = "/usr/local/synthlauncher";
  }
#endif

// Architecture Configuration
#if defined(__x86_64__) || defined(_M_X64)
  config.ARCH = Architecture::Arch::X86_64;
#elif defined(__i386) || defined(_M_IX86)
  config.ARCH = Architecture::Arch::X86;
#elif defined(__aarch64__)
  config.ARCH = Architecture::Arch::Arm64;
#elif defined(__arm__)
  config.ARCH = Architecture::Arch::Arm;
#endif

  config.ASSETS_DIR = fs::path(config.DIR) / "assets";
  config.LIBRARIES_DIR = fs::path(config.DIR) / "libraries";
  config.NATIVES_DIR = fs::path(config.DIR) / "natives";

  return config;
}