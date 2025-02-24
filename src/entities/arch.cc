#include "include/entities/arch.hh"

Architecture::Arch Architecture::arch_from_string(const std::string &str) {
    if (str == "x86") {
        return Arch::X86;
    }
    else if (str == "x86_64") {
        return Arch::X86_64;
    }
    else if (str == "arm64") {
        return Arch::Arm64;
    }
    else if (str == "arm") {
        return Arch::Arm;
    }
}

std::string Architecture::arch_to_string(const Architecture::Arch& arch) {
    switch (arch)
    {
    case Arch::X86_64: return "x86_64";
    case Arch::X86: return "x86";
    case Arch::Arm64: return "arm64";
    case Arch::Arm: return "arm";
    }
}