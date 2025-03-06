#include "include/entities/os.hh"

OperatingSystem::OS OperatingSystem::fromString(std::string_view str) {
  if (str == "windows")
    return OS::Windows;
  else if (str == "osx")
    return OS::OSX;
  else if (str == "linux")
    return OS::Linux;

  throw std::invalid_argument("Unknown OS: " + std::string(str));
}