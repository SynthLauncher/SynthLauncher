#include <gtest/gtest.h>

#include "include/utils/httplib_utils.hh"

TEST(HttpLibUtilsHH, ExtractHostAndPath) {
  std::string url = "https://github.com/SynthLauncher/SynthLauncher";

  auto [host, path] = httplib_utils::extractHostAndPath(url);

  ASSERT_EQ(host, "https://github.com");
  ASSERT_EQ(path, "/SynthLauncher/SynthLauncher");
}   