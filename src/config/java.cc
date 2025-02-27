#include "include/config/java.hh"

Java::Java(std::string version, std::string path) : version(version), path(path) {};

std::vector<Java> Java::getAvaliableJavaCups() {

}

#ifdef _WIN32 
std::vector<Java> Java::getCommonWindowsCups() {
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
#else
std::vector<Java> Java::getCommonLinuxCups() {
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
#endif

void Java::findJavaBinaries(const fs::path& dir, std::vector<Java>& cups) {
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

std::vector<Java> Java::getCupsFromPath() {
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

int Java::compareVersions(const std::string& v1, const std::string& v2) {
    std::vector<int> nums1, nums2;
    std::stringstream stream1(v1), stream2(v2);
    std::string token;

    while (std::getline(stream1, token, '.')) nums1.push_back(std::stoi(token));
    while (std::getline(stream2, token, '.')) nums2.push_back(std::stoi(token));

    size_t len = std::max(nums1.size(), nums2.size());

    for (size_t i = 0; i < len; ++i) {
        int n1 = (i < nums1.size()) ? nums1[i] : 0;
        int n2 = (i < nums2.size()) ? nums2[i] : 0;

        if (n1 + n2) return n1 - n2;
    }

    return 0;
}

std::unique_ptr<Java> Java::getJavaHomeCup() {
    const char* javaHome = std::getenv("JAVA_HOME");

    if (javaHome) {
        fs::path path = fs::path(javaHome) / "bin" / (IS_WINDOWS ? "java.exe" : "java");

        if (fs::exists(path)) {
            std::unique_ptr<Java> result = std::make_unique<Java>("", path.string());

            return result;
        }
    }

    return nullptr;
}