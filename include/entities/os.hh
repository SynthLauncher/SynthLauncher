#pragma once

#include <stdexcept>
#include <string>

/// @brief Contains the OperatingSystem enum and a function to convert strings
/// to OperatingSystem::OS
namespace OperatingSystem {
/// @brief Enum class representing an Operating System (Windows, Linux, OSX)
enum class OS { Windows, Linux, OSX };

/// @brief Converts a string to an OperatingSystem::OS, if the string is not a
/// valid OS, an exception is thrown
OS fromString(std::string_view str);
} // namespace OperatingSystem