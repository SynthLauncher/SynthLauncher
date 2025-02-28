#include <gtest/gtest.h>

#include <iostream>
#include "include/config/app.hh"

TEST(AppCC, AppConfigInitializationTest) {
    const AppConfig config = initializeAppConfig();
}