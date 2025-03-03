#pragma once

#include <stdexcept>
#include <string>

namespace Architecture {
enum class Arch { X86, X86_64, Arm64, Arm };

Arch arch_from_string(std::string_view str);
} // namespace Architecture