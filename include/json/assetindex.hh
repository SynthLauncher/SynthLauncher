#pragma once

#define CPPHTTPLIB_OPENSSL_SUPPORT

#include <map>
#include <string>
#include <filesystem>
#include "include/config/app.hh"
#include "include/httplib.h"

namespace fs = std::filesystem;

class AssetIndex {
public:
    class AssetObject {
    public:
        std::string hash;

        std::string id();
        std::string url();
        fs::path path(AppConfig& config);
        void fetch(AppConfig& config);
    };

    std::map<std::string, AssetObject> objects;
};