#include <string>

namespace OperatingSystem {
    enum class OS {
        Windows,
        Linux,
        OSX
    };

    OS os_from_string(const std::string &str);
    std::string os_to_string(const OS& os);
}