#include <gtest/gtest.h>

#include "include/config/java.hh"

TEST(JavaHH, VersionExtraction) {
  auto cups = Java::getAvaliableJavaCups();
  auto cup = cups[0];
  Java::extractJavaVersion(cup);

  ASSERT_EQ(cup.version, "21.0.5");
}