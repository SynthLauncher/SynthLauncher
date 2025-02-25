#include "include/config/java.hh"

Java::Java(std::string version, std::string path) {
    this->version = version;
    this->path = path;
}

std::vector<Java> Java::getAvaliableJavaCups() {}

void Java::sortCups(std::vector<Java>& cups) {
    std::sort(cups.begin(), cups.end(), [](Java& cup1, Java& cup2) {
        return compareVersions(cup1.version, cup2.version) < 0;
    });
}

Java Java::getJavaHomeCups() {}

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

std::vector<Java> Java::getCommonLinuxCups() {}
std::vector<Java> Java::getCommonWindowsCups() {}
std::vector<Java> Java::getCupsInDirs(std::vector<fs::path> directories) {}
void Java::findJavaBinaries(fs::path dir, std::vector<Java>& cups) {}
std::vector<Java> Java::getCupsPath() {}
std::vector<Java> Java::getRegCups() {}