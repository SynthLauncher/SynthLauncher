#include "include/config/java.hh"

Java::Java(std::string version, std::string path) : version(version), path(path) {};

std::vector<Java> Java::getAvaliableJavaCups() {

}

void Java::sortCups(std::vector<Java>& cups) {
    std::sort(cups.begin(), cups.end(), [](Java& cup1, Java& cup2) {
        return compareVersions(cup1.version, cup2.version) < 0;
    });
}

int Java::compareVersions(std::string& version1, std::string& version2) {
    std::istringstream v1Stream(version1), v2Stream(version2);
    std::string v1Part, v2Part;

    while (std::getline(v1Stream, v1Part, '.') || std::getline(v2Stream, v2Part,'.')) {
        int num1 = v1Part.empty() ? 0 : std::stoi(v1Part);
        int num2 = v2Part.empty() ? 0 : std::stoi(v2Part);

        if (num1 != num2) {
            return num1 - num2;
        }

        v1Part.clear();
        v2Part.clear();
    }

    return 0;
}

std::vector<Java> Java::getCommonLinuxCups() {
    std::vector<Java> cups;
    std::vector<fs::path> directories = {
        "/usr/lib/jvm",
        "/usr/lib64/jvm",
        "/usr/lib32/jvm"
    };

    for (const auto& dir : directories) {
        if (fs::exists(dir)) {
            findJavaBinaries(dir, cups);
        }
    }

    return cups;
}

std::vector<Java> Java::getCommonWindowsCups() {
    std::vector<Java> cups;
    const char* systemDrive = std::getenv("SystemDrive");
    
    std::vector<fs::path> directories = {
        fs::path(systemDrive) / "/Program Files" / "Java",
        fs::path(systemDrive) / "/Program Files (x86)" / "Java"
    };

    for (const auto& dir : directories) {
        if (fs::exists(dir)) {
            findJavaBinaries(dir, cups);
        }
    }

    return cups;
}

void Java::findJavaBinaries(const fs::path& dir, 
std::vector<Java>& cups) {
    try {
        for (const auto& entry : fs::directory_iterator(dir)) {
            if (entry.is_directory()) {
                fs::path javaPath = entry.path() / "bin" / (IS_WINDOWS ? "java.exe" : "java");
                
                if (fs::exists(javaPath)) {
                    cups.emplace_back("", javaPath.string());
                }
                else {
                    findJavaBinaries(entry.path(), cups);
                }
            }
        }
    }
    catch (const fs::filesystem_error&) {
        // !!! Handle the error
    }
}

Java Java::getJavaHomeCups() {

}

std::vector<Java> Java::getCupsInDirs(std::vector<fs::path> directories) {

}

std::vector<Java> Java::getCupsPath() {

}

std::vector<Java> Java::getRegCups() {

}