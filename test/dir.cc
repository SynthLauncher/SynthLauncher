#include <iostream>
#include <gtest/gtest.h>
#include "include/config/app.hh"

TEST(Dir, DirCreationTest) {
    const AppConfig config = initializeAppConfig();

    #ifdef _WIN32
        std::cout << config << std::endl;
    #elif __linux__

    #elif __APPLE__
    
    #endif
}