#pragma once

#include <string>
#include <stdexcept>

namespace OperatingSystem {
    enum class OS {
        Windows,
        Linux,
        OSX
    };

    OS os_from_string(std::string& str);
}