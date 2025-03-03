#pragma once

#include <stdexcept>
#include <string>

namespace OperatingSystem {
enum class OS { Windows, Linux, OSX };

OS os_from_string(std::string_view str);
} // namespace OperatingSystem