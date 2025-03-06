#pragma once

#include <cstdint>
#include <filesystem>
#include <map>
#include <optional>
#include <string>
#include <vector>
#include <minizip/unzip.h>
#include <minizip/zip.h>
#include <zlib.h>

#define CPPHTTPLIB_OPENSSL_SUPPORT

#include "include/config/app.hh"
#include "include/entities/arch.hh"
#include "include/entities/os.hh"
#include "include/utils/rapidjson_utils.hh"
#include "include/rapidjson/document.h"
#include "include/rapidjson/error/en.h"
#include "include/utils/httplib_utils.hh"
#include "include/json/assetindex.hh"

/*
  Windows macro was intefering with the GetObject function
  from the rapidjson library. This undefines the macro
*/
#undef GetObject

namespace fs = std::filesystem;

/// @brief Client JSON
class Client {
public:
  struct Features {
    bool isDemoUser;
    bool hasCustomResolution;
    bool hasQuickPlaysSupport;
    bool isQuickPlaySingleplayer;
    bool isQuickPlayMultiplayer;
    bool isQuickPlayRealms;

    static Features parse(const rapidjson::Value &obj);
  };

  struct OSRules {
    std::optional<OperatingSystem::OS> name;
    std::optional<Architecture::Arch> arch;
    std::string version;

    static OSRules parse(const rapidjson::Value &obj);
  };

  struct Rule {
    std::string action;
    std::optional<OSRules> os;
    std::optional<Features> features;

    static Rule parse(const rapidjson::Value &obj);
    bool osMatches(AppConfig &config);
    static bool osMatches(AppConfig &config, std::vector<Rule> rules);
  };

  struct Argument {
    std::string value;
    std::vector<std::string> values;
    std::vector<Rule> rules;

    static Argument parse(const rapidjson::Value &val);
  };

  struct Arguments {
    std::vector<Argument> game;
    std::vector<Argument> jvm;

    static Arguments parse(const rapidjson::Value &obj);
  };

  struct Download {
    std::string id;
    std::string path;
    std::string sha1;
    int64_t size;
    std::optional<int64_t> totalSize;
    std::string url;

    static Download parse(const rapidjson::Value &obj);
    std::vector<std::uint8_t> fetch();
  };

  struct ClientDownloads {
    Download client;
    Download client_mappings;
    Download server;
    Download server_mappings;

    static ClientDownloads parse(const rapidjson::Value &obj);
  };

  struct JavaVersion {
    std::string component;
    int majorVersion;

    static JavaVersion parse(const rapidjson::Value &obj);
  };

  struct LibraryDownloads {
    Download artifact;
    std::map<std::string, Download> classifiers;

    std::vector<uint8_t> fetchArtifact();
    fs::path artifactPath(AppConfig &config);
    std::vector<uint8_t> fetchNative(std::string nativeIndex);
    fs::path nativePath(AppConfig &config, std::string nativeIndex);

    static LibraryDownloads parse(const rapidjson::Value &obj);
  };

  struct LibraryExtractRules {
    std::vector<std::string> exclude;
    std::vector<std::string> include;

    static LibraryExtractRules parse(const rapidjson::Value &obj);
  };

  struct Library {
    LibraryDownloads downloads;
    std::string name;
    std::vector<Rule> rules;
    std::map<OperatingSystem::OS, std::string> natives;
    LibraryExtractRules extract;

    static Library parse(const rapidjson::Value &obj);
    void downloadArtifact(AppConfig &config);
    void downloadNative(AppConfig &config);
    void extractNative(AppConfig &config, fs::path instanceDir);
  };

  struct LoggingClient {
    std::string argument;
    Download file;
    std::string type;

    static LoggingClient parse(const rapidjson::Value &obj);
  };

  struct LoggingInfo {
    LoggingClient client;

    static LoggingInfo parse(const rapidjson::Value &obj);
  };

  /*
    The actual client.json:
  */
  Arguments arguments;
  Download assetIndex;
  std::string assets;
  short complianceLevel;
  ClientDownloads downloads;
  std::string id;
  JavaVersion javaVersion;
  std::vector<Library> libraries;
  LoggingInfo logging;
  std::string mainClass;

  std::string minecraftArguments;

  std::string type;

  static Client parse(const rapidjson::Value &obj);

  void downloadAssets(AppConfig &config);
  void downloadLibraries(AppConfig &config, fs::path instanceDir);
  void downloadClientDownloads(fs::path instanceDir);
  void download(AppConfig &config, fs::path instanceDir);
  std::vector<fs::path> getLibrariesList(AppConfig &config);
};
