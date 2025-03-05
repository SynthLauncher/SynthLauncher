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
/// @brief Checks if the current OS is Windows
constexpr bool IS_WINDOWS = true;
#else
constexpr bool IS_WINDOWS = false;
#endif

class Java {
public:
  std::string version;
  std::string path;

  Java();
  Java(std::string version, std::string path);

  std::string toJson() const;
  /// @brief Gets all available Java versions
  static std::vector<Java> getAvaliableJavaCups();
  /// @brief Sors Java cups by version
  static void sortCups(std::vector<Java> &cups);
  /// @brief Gets the Java home cup
  static std::unique_ptr<Java> getJavaHomeCup();
  /// @brief Extracts the Java version from the Java cup
  static bool extractJavaVersion(Java &cup);

private:
  /// @brief Compares two Java versions
  static int compareVersions(const std::string &v1, const std::string &v2);
  /// @brief Gets the common Linux Java cups
  static std::vector<Java> getCommonLinuxCups();
  /// @brief Gets the common Windows Java cups
  static std::vector<Java> getCommonWindowsCups();
  /// @brief Finds Java binaries in a directory
  static void findJavaBinaries(const fs::path &dir, std::vector<Java> &cups);
  /// @brief Gets Java cups from the PATH environment variable
  static std::vector<Java> getCupsFromPath();
};