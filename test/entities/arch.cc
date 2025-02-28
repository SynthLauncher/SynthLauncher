#include <gtest/gtest.h>
#include <string>

#include "include/entities/arch.hh"

TEST(Arch, ArchFromStringTest) {
  std::string str1 = "arm64";
  std::string str2 = "arm";
  std::string str3 = "x86";
  std::string str4 = "x86_64";

  Architecture::Arch arm64 = Architecture::arch_from_string(str1);
  Architecture::Arch arm = Architecture::arch_from_string(str2);
  Architecture::Arch x86 = Architecture::arch_from_string(str3);
  Architecture::Arch x86_64 = Architecture::arch_from_string(str4);

  ASSERT_EQ(arm64, Architecture::Arch::Arm64);
  ASSERT_EQ(arm, Architecture::Arch::Arm);
  ASSERT_EQ(x86, Architecture::Arch::X86);
  ASSERT_EQ(x86_64, Architecture::Arch::X86_64);
}