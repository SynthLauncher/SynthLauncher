#pragma once

#include <algorithm>
#include <array>
#include <cstdlib>
#include <filesystem>
#include <iostream>
#include <memory>
#include <regex>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#ifdef _WIN32
#include <shlobj.h>
#include <windows.h>
#else
#include <limits.h>
#include <unistd.h>
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
  static void sortCups(std::vector<Java> &cups);
  static std::unique_ptr<Java> getJavaHomeCup();
  static bool extractJavaVersion(Java &cup);
private:
  static int compareVersions(const std::string &v1, const std::string &v2);
  static std::vector<Java> getCommonLinuxCups();
  static std::vector<Java> getCommonWindowsCups();
  static void findJavaBinaries(const fs::path &dir, std::vector<Java> &cups);
  static std::vector<Java> getCupsFromPath();
};