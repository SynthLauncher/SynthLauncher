#pragma once

#include <string>

namespace Architecture {
    enum class Arch {
        X86,
        X86_64,
        Arm64,
        Arm
    };

    Arch arch_from_string(const std::string& str);
    std::string arch_to_string(const Arch& arch);
}