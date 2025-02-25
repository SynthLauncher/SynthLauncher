#pragma once

#include <string>
#include <vector>
#include <algorithm>
#include <filesystem>

namespace fs = std::filesystem;

class Java {
public:
    std::string version;
    std::string path;

    Java(std::string version, std::string path);

    static std::vector<Java> getAvaliableJavaCups();
    static void sortCups(std::vector<Java>& cups);
    static Java getJavaHomeCups();

private:
    static int compareVersions(std::string& version1, std::string& version2);
    static std::vector<Java> getCommonLinuxCups();
    static std::vector<Java> getCommonWindowsCups();
    static std::vector<Java> getCupsInDirs(std::vector<fs::path> directories);
    static void findJavaBinaries(fs::path dir, std::vector<Java>& cups);
    static std::vector<Java> getCupsPath();
    static std::vector<Java> getRegCups();
};