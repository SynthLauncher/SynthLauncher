#pragma once

#include <string>
#include <vector>
#include <algorithm>
#include <filesystem>

#ifdef _WIN32
#include <windows.h>
#include <shlobj.h>
#else
#include <unistd.h>
#include <limits.h>
#endif 

namespace fs = std::filesystem;

#ifdef _WIN32
constexpr bool IS_WINDOWS = true;
#else
constexpr bool IS_WINDOWS = false;
#endif

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
    static void findJavaBinaries(const fs::path& dir, std::vector<Java>& cups);
    static std::vector<Java> getCupsPath();
    static std::vector<Java> getRegCups();
};