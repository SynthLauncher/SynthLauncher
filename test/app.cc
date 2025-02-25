#include <iostream>
#include <gtest/gtest.h>
#include "include/config/app.hh"

TEST(App, DirectoryCreationTest) {
    const AppConfig config = initializeAppConfig();

    std::cout << config << std::endl;
}