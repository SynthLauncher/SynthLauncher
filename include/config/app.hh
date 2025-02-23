#include <filesystem>
#include <string>
#include "include/entities/arch.hh"
#include "include/entities/os.hh"

namespace fs = std::filesystem;

struct AppConfig {
    std::string DIR;
    fs::path ASSETS_DIR;
    fs::path LIBRARIES_DIR;
    fs::path NATIVES_DIR;
    OperatingSystem::OS OS;
    Architecture::Arch ARCH;
};

AppConfig initializeAppConfig();