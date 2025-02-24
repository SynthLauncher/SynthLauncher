#pragma once

#include <string>
#include <vector>
#include <filesystem>

namespace fs = std::filesystem;

class Manifest {
public:
    static fs::path PATH;

    class Latest {
    public:
        std::string release;
        std::string snapshot;
    };

    class Version {
    public:
        std::string id;
        std::string type;
        std::string url;
        std::string time;
        std::string releaseTime;
    };

    Latest latest;
    std::vector<Version> version;
};