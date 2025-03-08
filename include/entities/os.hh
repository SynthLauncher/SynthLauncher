#pragma once

#include <stdexcept>
#include <string>

/// @brief Enum class representing an Operating System (Windows, Linux, OSX)
enum class OS { Windows, Linux, OSX };

/// @brief Converts a string to an OS, if the string is not a
/// valid OS, an exception is thrown
OS stringToOs(const std::string &str);