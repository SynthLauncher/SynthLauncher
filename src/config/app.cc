#include "include/config/app.hh"

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

void initializeLauncherDir(AppConfig &config) {
  if (!fs::exists(config.DIR))
    fs::create_directories(config.DIR);
  if (!fs::exists(config.ASSETS_DIR))
    fs::create_directories(config.ASSETS_DIR);
  if (!fs::exists(config.LIBRARIES_DIR))
    fs::create_directories(config.LIBRARIES_DIR);
  if (!fs::exists(config.NATIVES_DIR))
    fs::create_directories(config.NATIVES_DIR);

  httplib::Client cli("https://launchermeta.mojang.com");

  Manifest::PATH = config.DIR + "/manifest.json";

  auto res = cli.Get("/mc/game/version_manifest.json");

  if (res && res->status == 200) {
    std::ofstream out(Manifest::PATH, std::ios::binary);
    out.write(res->body.c_str(), res->body.size());
    out.close();
  } else {
    if (!std::ifstream(Manifest::PATH)) {
      throw std::runtime_error(
          "Failed to fetch Manifest.json; Response code: " +
          std::to_string(res ? res->status : -1));
    }
  }
}