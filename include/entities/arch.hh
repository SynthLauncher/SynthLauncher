#pragma once

#include <stdexcept>
#include <string>


/// @brief Enum class representing an Architecture (X86, X86_64, Arm64, Arm)
enum class Arch { X86, X86_64, Arm64, Arm };

/// @brief Converts a string to an Arch, if the string is not a
/// valid architecture, an exception is thrown
Arch stringToArch(const std::string &str);