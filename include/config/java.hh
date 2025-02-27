#pragma once

#include <iostream>
#include <vector>
#include <string>
#include <filesystem>
#include <algorithm>
#include <regex>
#include <cstdlib>
#include <sstream>
#include <memory>
#include <stdexcept>
#include <array>

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
    static std::unique_ptr<Java> getJavaHomeCup();
    
private:
    static int compareVersions(const std::string& v1, const std::string& v2);
    static std::vector<Java> getCommonLinuxCups();
    static std::vector<Java> getCommonWindowsCups();
    static void findJavaBinaries(const fs::path& dir, std::vector<Java>& cups);
    static std::vector<Java> getCupsFromPath(); };