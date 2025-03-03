#include <gtest/gtest.h>

#include "include/config/app.hh"
#include <iostream>

TEST(AppHH, AppConfigInitialization) {
  AppConfig config = initializeAppConfig();

#ifdef _WIN32
  ASSERT_EQ(config.OS, OperatingSystem::OS::Windows);
#elif __linux__
  ASSERT_EQ(config.OS, OperatingSystem::OS::Linux);
#elif __APPLE__
  ASSERT_EQ(config.OS, OperatingSystem::OS::OSX);
#endif
}

TEST(AppHH, LauncherDirectoryInitialization) {
  AppConfig config = initializeAppConfig();
  initializeLauncherDir(config);
}