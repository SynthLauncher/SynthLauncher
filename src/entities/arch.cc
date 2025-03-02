#include "include/entities/arch.hh"

Architecture::Arch Architecture::arch_from_string(std::string &str) {
  if (str == "arm64")
    return Arch::Arm64;

  else if (str == "arm")
    return Arch::Arm;

  else if (str == "x86")
    return Arch::X86;

  else if (str == "x86_64")
    return Arch::X86_64;

  throw std::invalid_argument("Unknown architecture: " + str);
}