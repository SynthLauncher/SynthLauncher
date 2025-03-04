#include "include/utils/rapidjson_utils.hh"
#include <gtest/gtest.h>

TEST(RapidJsonUtilsHH, ReadFileToString) {
  std::string str = read_file_to_string(
      "E:/OneDrive/Desktop/SynthLauncher/assets/download.json");

  std::cout << str;
}

TEST(RapidJsonUtilsHH, ParseJsonFile) {
  rapidjson::Document doc =
      parse_json_file("E:/OneDrive/Desktop/SynthLauncher/assets/download.json");

  const rapidjson::Value &download = doc["download"];
  std::cout << download["path"].GetString() << std::endl;
}