#include "include/entities/instance.hh"
#include "include/utils/rapidjson_utils.hh"

#include <gtest/gtest.h>

TEST(InstanceHH, InstanceParsing) {
  std::string json = read_file_to_string("E:/OneDrive/Desktop/SynthLauncher/assets/instances.json");
  rapidjson::Document doc;
  rapidjson::Value &arr = doc.Parse(json.c_str());

  Instance instance = Instance::parse(arr[0]);

  EXPECT_EQ(instance.name, "test");
  EXPECT_EQ(instance.version, "1.0.0");
}