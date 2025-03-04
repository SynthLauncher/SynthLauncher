#include "include/utils/rapidjson_utils.hh"
#include <gtest/gtest.h>

TEST(RapidJsonUtilsHH, ReadFileToString) { 
    std::string str = read_file_to_string("E:/OneDrive/Desktop/SynthLauncher/assets/download.json");

    std::cout << str;
}