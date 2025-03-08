#include "include/config/config.hh"

uint64_t Config::getTotalPhysicalMemory() {
#ifdef _WIN32
  MEMORYSTATUSEX status;
  status.dwLength = sizeof(status);
  if (!GlobalMemoryStatusEx(&status))
    throw std::runtime_error("Failed to get memory status");

  return status.ullTotalPhys;
#elif defined(__APPLE__)
  uint64_t mem;
  size_t len = sizeof(mem);
  int mib[2] = {CTL_HW, HW_MEMSIZE};
  if (sysctl(mib, 2, &mem, &len, NULL, 0) == -1)
    throw std::runtime_error("Failed to get memory status");

  return mem;
#else
  struct sysinfo info;
  if (sysinfo(&info) != 0)
    throw std::runtime_error("Failed to get memory status");

  return static_cast<uint64_t>(info.totalram) * info.mem_unit;
#endif
}

Config::Config() {
  uint64_t total = getTotalPhysicalMemory();

  max_ram = total / 4 / 1024 / 1024;
  min_ram = max_ram / 2;

  java = Java::getAvaliableJavaCups()[0];
  path = MAIN_PATH.string();
}

Config::Config(const fs::path &path)
    : java(), path(path), min_ram(0), max_ram(0) {}

Config::Config(const Java &java, const fs::path &path, const uint64_t &min_ram,
               const uint64_t &max_ram)
    : java(java), path(path), min_ram(min_ram), max_ram(max_ram) {}

std::string Config::toJson() {
  std::ostringstream json;
  json << "\"path\": \"" << path << "\", "
       << "\"min_ram\": " << min_ram << ", "
       << "\"max_ram\": " + max_ram << ", "
       << "\"java\": " << java.toJson();
  return json.str();
}

Config Config::fromJson(const rapidjson::Value &obj) {
  Config config;

  if (obj.HasMember("path"))
    config.path = obj["path"].GetString();

  if (obj.HasMember("min_ram"))
    config.min_ram = obj["min_ram"].GetUint64();

  if (obj.HasMember("max_ram"))
    config.max_ram = obj["max_ram"].GetUint64();

  if (obj.HasMember("java"))
    config.java = Java::fromJson(obj["java"]);

  return config;
}

Config Config::getConfig(fs::path path) {
  auto json = rapidjson_utils::fromJson(path);

  Config config = Config::fromJson(json);
  config.path = path;

  return config;
}

void Config::writeConfig() {
  std::string json = this->toJson();

  std::ofstream file(this->path, std::ios::out | std::ios::trunc);
  if (!file)
    throw std::runtime_error("Failed to open file: " + this->path.string());

  file << json;
}

Config Config::readMainConfig() { return getConfig(MAIN_PATH); };

void Config::launch(App::AppConfig &config, Instance &instance) {
  Client client = instance.readClient();
  std::vector<fs::path> paths = client.getLibrariesList(config);

  std::string classpath;
  char seperator =
#ifdef _WIN32
      ';';
#else
      ':';
#endif

  for (const auto &path : paths)
    classpath += path.string() + seperator;

  classpath += (instance.dir() / "client.jar").string();

  std::vector<std::string> args = {
      java.path,
      "-Djava.library.path=" + (instance.dir() / "natives").string(),
      "-Xms" + std::to_string(min_ram) + "M",
      "-Xmx" + std::to_string(max_ram) + "M",
      "-cp",
      classpath,
      client.mainClass,
      "--username",
      "testUser",
      "--skinURL",
      "https://live.staticflickr.com/65535/53083566002_ae3333d694.jpg",
      "--gameDir",
      instance.dir().string(),
      "--assetsDir",
      config.ASSETS_DIR.string(),
      "--assetIndex",
      client.assets,
      "--version",
      client.id,
      "--accessToken",
      "0"};

  std::cout << "Running:\n";
  for (const auto &arg : args)
    std::cout << arg << " ";
  std::cout << '\n';

/*
    Maybe move this into a separate function?
*/
#ifdef _WIN32
  std::string cmd;
  for (const auto &arg : args) {
    if (arg.find(' ') != std::string::npos)
      cmd += '"' + arg + "\" ";
    else
      cmd += arg + " ";
  }

  // Convert UTF-8 command line to UTF-16
  int wlen = MultiByteToWideChar(CP_UTF8, 0, cmd.c_str(), -1, nullptr, 0);
  std::wstring wcmd;
  wcmd.resize(wlen);
  MultiByteToWideChar(CP_UTF8, 0, cmd.c_str(), -1, wcmd.data(), wlen);

  // Create modifiable buffer for Windows API
  std::vector<wchar_t> cmdLine(wcmd.size() + 1);
  std::copy(wcmd.begin(), wcmd.end(), cmdLine.begin());
  cmdLine[wcmd.size()] = L'\0';

  STARTUPINFOW si = {sizeof(si)};
  PROCESS_INFORMATION pi;
  si.dwFlags = STARTF_USESTDHANDLES;
  si.hStdInput = GetStdHandle(STD_INPUT_HANDLE);
  si.hStdOutput = GetStdHandle(STD_OUTPUT_HANDLE);
  si.hStdError = GetStdHandle(STD_OUTPUT_HANDLE);

  if (!CreateProcessW(nullptr,        // Application name
                      cmdLine.data(), // Command line (wide version)
                      nullptr,        // Process attributes
                      nullptr,        // Thread attributes
                      TRUE,           // Inherit handles
                      0,              // Creation flags
                      nullptr,        // Environment
                      nullptr,        // Current directory
                      &si, &pi)) {
    std::cerr << "CreateProcess failed (" << GetLastError() << ")\n";
    return;
  }

  WaitForSingleObject(pi.hProcess, INFINITE);
  CloseHandle(pi.hProcess);
  CloseHandle(pi.hThread);
#else
  /*
    I'll write this later
  */
#endif
}