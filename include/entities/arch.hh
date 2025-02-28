#pragma once

#include <string>
#include <stdexcept>

namespace Architecture {
    enum class Arch {
        X86,
        X86_64,
        Arm64,
        Arm
    };

    Arch arch_from_string(std::string& str);
}