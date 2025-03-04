#include <gtest/gtest.h>

#include "include/config/app.hh"
#include <iostream>

TEST(AppHH, AppConfigInitialization) {
  AppConfig config = initializeAppConfig();

#ifdef _WIN32
  ASSERT_EQ(config.OS, OperatingSystem::OS::Windows);
  ASSERT_EQ(config.DIR, "C:\\Users\\User\\AppData\\Roaming\\SynthLauncher");
#elif __linux__
  ASSERT_EQ(config.OS, OperatingSystem::OS::Linux);
  ASSERT_EQ(config.DIR, "/usr/local/synthlauncher");
#elif __APPLE__
  ASSERT_EQ(config.OS, OperatingSystem::OS::OSX);
  ASSERT_EQ(config.DIR, "/usr/local/synthlauncher");
#endif

#if defined(__x86_64__) || defined(_M_X64)
  ASSERT_EQ(config.ARCH, Architecture::Arch::X86_64);
#elif defined(__i386) || defined(_M_IX86)
  ASSERT_EQ(config.ARCH, Architecture::Arch::X86);
#elif defined(__aarch64__)
  ASSERT_EQ(config.ARCH, Architecture::Arch::Arm64);
#elif defined(__arm__)
  ASSERT_EQ(config.ARCH, Architecture::Arch::Arm);
#endif
}

TEST(AppHH, LauncherDirectoryInitialization) {
  AppConfig config = initializeAppConfig();
  initializeLauncherDir(config);

  ASSERT_TRUE(fs::exists(config.DIR));
  ASSERT_TRUE(fs::exists(config.ASSETS_DIR));
  ASSERT_TRUE(fs::exists(config.LIBRARIES_DIR));
  ASSERT_TRUE(fs::exists(config.NATIVES_DIR));
}