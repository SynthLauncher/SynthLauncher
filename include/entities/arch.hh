#pragma once

#include <stdexcept>
#include <string>

/// @brief Contains the Architecture enum and a function to convert strings to
/// Architecture::Arch
namespace Architecture {
/// @brief Enum class representing an Architecture (X86, X86_64, Arm64, Arm)
enum class Arch { X86, X86_64, Arm64, Arm };

/// @brief Converts a string to an Architecture::Arch, if the string is not a
/// valid architecture, an exception is thrown
Arch fromString(std::string_view str);
} // namespace Architecture