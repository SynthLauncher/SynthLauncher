#include "include/config/config.hh"

uint64_t Config::getTotalPhysicalMemory() {
    #ifdef _WIN32
        MEMORYSTATUSEX status;
        status.dwLength = sizeof(status);
        if (!GlobalMemoryStatusEx(&status)) {
            throw std::runtime_error("Failed to get memory status");
        }
        return status.ullTotalPhys;
    #elif defined(__APPLE__)
        uint64_t mem;
        size_t len = sizeof(mem);
        int mib[2] = {CTL_HW, HW_MEMSIZE};
        if (sysctl(mib, 2, &mem, &len, NULL, 0) == -1) {
            throw std::runtime_error("Failed to get memory status");
        }
        return mem;
    #else
        struct sysinfo info;
        if (sysinfo(&info) != 0) {
            throw std::runtime_error("Failed to get memory status");
        }
        return static_cast<uint64_t>(info.totalram) * info.mem_unit;
    #endif
}

Config::Config(fs::path path) : java(), path(path.string()), min_ram(0), max_ram(0) {}

Config::Config() { 
    uint64_t total = getTotalPhysicalMemory();

    this->max_ram = total / 4 / 1024 / 1024;
    this->min_ram = this->max_ram / 2;

    this->java = Java::getAvaliableJavaCups()[0];
    this->path = MAIN_PATH.string();
}

uint64_t Config::getMinRam() const {
    return this->min_ram;
}

uint64_t Config::getMaxRam() const {
    return this->max_ram;
}

uint64_t Config::getMinRam() const {
    return this->min_ram;
}

Java Config::getJava() const {
    return this->java;
}