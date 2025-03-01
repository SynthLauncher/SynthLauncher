#pragma once

#include <filesystem>
#include <stdexcept>
#include <string>
#include <vector>

namespace fs = std::filesystem;

class Instance {
public:
  static fs::path PARENT_DIR;
  static fs::path INSTANCE_FILE;

  std::string name;
  std::string version;
  fs::path icon;

  Instance(const std::string &name, const std::string &version);
  fs::path dir();
  void initDir();
  static Instance createInstance(const std::string &name,
                                 const std::string &version);
  static std::vector<Instance> readInstances();
  static Instance getInstance(const std::string &name);
};