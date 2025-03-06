#include "include/utils/rapidjson_utils.hh"
#include <gtest/gtest.h>

TEST(RapidJsonUtilsHH, ReadFileToString) {
  std::string str = rapidjson_utils::toString(
      "E:/OneDrive/Desktop/SynthLauncher/assets/download.json");

  std::cout << str;
}

TEST(RapidJsonUtilsHH, ParseJsonFile) {
  rapidjson::Document doc =
      rapidjson_utils::fromJson("E:/OneDrive/Desktop/SynthLauncher/assets/download.json");

  const rapidjson::Value &download = doc["download"];
  std::cout << download["path"].GetString() << std::endl;
}