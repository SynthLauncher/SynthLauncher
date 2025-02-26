#include <gtest/gtest.h>

#include <iostream>
#include "include/config/app.hh"

TEST(App, AppConfigInitializationTest) {
    const AppConfig config = initializeAppConfig();
    
    std::cout << config << std::endl;
}