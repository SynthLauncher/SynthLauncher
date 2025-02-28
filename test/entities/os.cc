#include <gtest/gtest.h>
#include <string>

#include "include/entities/os.hh"

TEST(OS, OsFromStringTest) {
    std::string str1 = "linux";
    std::string str2 = "windows";
    std::string str3 = "osx";

    OperatingSystem::OS linux = OperatingSystem::os_from_string(str1);
    OperatingSystem::OS windows = OperatingSystem::os_from_string(str2);
    OperatingSystem::OS osx = OperatingSystem::os_from_string(str3);

    ASSERT_EQ(linux, OperatingSystem::OS::Linux);
    ASSERT_EQ(windows, OperatingSystem::OS::Windows);
    ASSERT_EQ(osx, OperatingSystem::OS::OSX);
}