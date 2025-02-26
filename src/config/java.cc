#include "include/config/java.hh"

Java::Java(std::string version, std::string path) : version(version), path(path) {};

std::vector<Java> getAvaliableJavaCups() {

}

#ifdef _WIN32 
std::vector<Java> getCommonWindowsCups() {
    std::vector<Java> cups;
    fs::path systemDrive(std::getenv("SystemDrive"));

    std::vector<fs::path> paths = {
        fs::path(systemDrive) / "Program Files" / "Java",
        fs::path(systemDrive) / "Program Files (x86)" / "Java"
    };

    for (const auto& path : paths) {
        if (fs::exists(path)) {
            findJavaBinaries(path, cups);
        }
    }

    return cups;
}

std::vector<Java> getRegCups() {
    // !!! Someone save me from this 
}

#else
std::vector<Java> getCommonLinuxCups() {
    std::vector<Java> cups;
    std::vector<fs::path> paths = {
        "/usr/lib/jvm",
        "/usr/lib64/jvm",
        "/usr/lib32/jvm"
    };

    for (const auto& path : paths) {
        if (fs::exists(path)) {
            findJavaBinaries(path, cups);
        }
    }

    return cups;
}

// Does nothing
std::vector<Java> getRegCups() { return {}; }
#endif

void findJavaBinaries(const fs::path& dir, std::vector<Java>& cups) {
    try {
        for (const auto& entry : fs::directory_iterator(dir)) {
            fs::path path = entry.path() / "bin" / (IS_WINDOWS ? "java.exe" : "java");

            if (fs::exists(path)) {
                cups.emplace_back("", path.string());
            }
            else {
                findJavaBinaries(entry.path(), cups);
            }
        }
    }
    catch (const fs::filesystem_error& error) {
        std::cerr << error.what();
    }
}

std::vector<Java> getCupsFromPath() {
    std::vector<Java> cups;
    std::string env = std::getenv("PATH");

    std::istringstream s1(env);
    std::string dir;

    while (std::getline(s1, dir, IS_WINDOWS ? ';' : ':')) {
        fs::path path = fs::path(dir) / (IS_WINDOWS ? "java.exe" : "java");

        if (fs::exists(path)) {
            cups.emplace_back("", path.string());
        }
    }

    return cups;
}