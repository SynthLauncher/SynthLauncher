#include "include/entities/os.hh"

OperatingSystem::OS OperatingSystem::os_from_string(std::string &str) {
  if (str == "windows")
    return OS::Windows;

  else if (str == "osx")
    return OS::OSX;

  else if (str == "linux")
    return OS::Linux;

  throw std::invalid_argument("Unknown OS: " + str);
}