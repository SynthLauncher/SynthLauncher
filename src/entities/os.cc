#include "include/entities/os.hh"
#include "include/entities/arch.hh"

OperatingSystem::OS OperatingSystem::os_from_string(const std::string &str) { 
    if (str == "windows") {
        return OS::Windows;
    }
    else if (str == "linux") {
        return OS::Linux;
    }
    else if (str == "osx") {
        return OS::OSX;
    }
}