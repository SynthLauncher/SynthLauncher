#include <string>

namespace OperatingSystem {
    enum class OS {
        Windows,
        Linux,
        OSX
    };

    OS os_from_string(const std::string &str);
}