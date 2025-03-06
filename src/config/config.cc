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

std::string Config::toJson() { 
    std::string json = "\"path\": ";
    json += this->path + ", ";
    json += "\"min_ram\": " + std::to_string(this->min_ram) + ", ";
    json += "\"max_ram\": " + std::to_string(this->max_ram) + ", ";
    json += this->java.toJson();
    return json;
}

Config Config::parse(const rapidjson::Value &obj) {
    Config config;
    if (obj.HasMember("path"))
        config.path = obj["path"].GetString();
    if (obj.HasMember("min_ram")) 
        config.min_ram = obj["min_ram"].GetUint64();
    if (obj.HasMember("max_ram")) 
        config.max_ram = obj["max_ram"].GetUint64();
    if (obj.HasMember("java")) 
        config.java = Java::parse(obj["java"]);

    return config;
}

Config Config::getConfig(fs::path path) {
  auto json = parse_json_file(path);
  Config config = Config::parse(json);
  config.path = path.string();

  return config;
}

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

Java Config::getJava() const {
    return this->java;
}

void Config::setMinRam(uint64_t min_ram) {
    this->min_ram = min_ram;
}

void Config::setMaxRam(uint64_t max_ram) {
    this->max_ram = max_ram;
}

void Config::setJava(Java java) {
    this->java = java;
}

void Config::writeConfig() { 
    std::string json = this->toJson();

    std::ofstream file(this->path, std::ios::out | std::ios::trunc);
    if (!file) {
        throw std::runtime_error("Failed to open file: " + this->path);
    }

    file << json;
}

Config Config::readMainConfig() { 
    return getConfig(MAIN_PATH);
};
