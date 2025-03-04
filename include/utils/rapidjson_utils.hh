#pragma once

#include "include/rapidjson/document.h"
#include <filesystem>
#include <fstream>
#include <sstream>
#include <stdexcept>
#include <string>

namespace fs = std::filesystem;

inline std::string read_file_to_string(const fs::path &path) {
  std::ifstream file(path);
  if (!file)
    throw std::runtime_error("Failed to open file: " + path.string());

  std::stringstream buffer;
  buffer << file.rdbuf();

  return buffer.str();
}

inline rapidjson::Document parse_json_file(const fs::path &path) {
  std::string json_string = read_file_to_string(path);

  rapidjson::Document doc;
  doc.Parse(json_string.c_str());
  if (doc.HasParseError())
    throw std::runtime_error("Failed to parse JSON file: " + path.string());

  return doc;
}