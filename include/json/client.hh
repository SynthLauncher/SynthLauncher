#pragma once
#define CPPHTTPLIB_OPENSSL_SUPPORT

#include <cstdint>
#include <filesystem>
#include <map>
#include <minizip/unzip.h>
#include <minizip/zip.h>
#include <optional>
#include <string>
#include <vector>

#include "include/config/app.hh"
#include "include/entities/arch.hh"
#include "include/entities/os.hh"
#include "include/json/assetindex.hh"
#include "include/rapidjson/document.h"
#include "include/rapidjson/error/en.h"
#include "include/utils/httplib_utils.hh"
#include "include/utils/rapidjson_utils.hh"



namespace fs = std::filesystem;

/// @brief Client JSON
class Client {
public:
  struct Features {
    /* 
      By default all of these must be false`
    */
    bool isDemoUser = false;
    bool hasCustomResolution = false;
    bool hasQuickPlaysSupport = false;
    bool isQuickPlaySingleplayer = false;
    bool isQuickPlayMultiplayer = false;
    bool isQuickPlayRealms = false;

    static Features fromJson(const rapidjson::Value &obj);
  };

  struct OSRules {
    std::optional<OS> name;
    std::optional<Arch> arch;
    std::string version;

    static OSRules fromJson(const rapidjson::Value &obj);
  };

  struct Rule {
    std::string action;
    std::optional<OSRules> os;
    std::optional<Features> features;

    static Rule fromJson(const rapidjson::Value &obj);

    bool osMatches(App::AppConfig &config);
    static bool osMatches(App::AppConfig &config, std::vector<Rule> rules);
  };

  struct Argument {
    std::string value;
    std::vector<std::string> values;
    std::vector<Rule> rules;

    static Argument fromJson(const rapidjson::Value &val);
  };

  struct Arguments {
    std::vector<Argument> game;
    std::vector<Argument> jvm;

    static Arguments fromJson(const rapidjson::Value &obj);
  };

  struct Download {
    std::string id;
    std::string path;
    std::string sha1;
    int64_t size;
    int64_t totalSize;
    std::string url;

    static Download fromJson(const rapidjson::Value &obj);
    std::vector<std::uint8_t> fetch();
  };

  struct ClientDownloads {
    Download client;
    Download client_mappings;
    Download server;
    Download server_mappings;

    static ClientDownloads fromJson(const rapidjson::Value &obj);
  };

  struct JavaVersion {
    std::string component;
    int majorVersion;

    static JavaVersion fromJson(const rapidjson::Value &obj);
  };

  struct LibraryDownloads {
    Download artifact;
    std::map<std::string, Download> classifiers;

    std::vector<uint8_t> fetchArtifact();
    fs::path artifactPath(App::AppConfig &config);

    std::vector<uint8_t> fetchNative(std::string nativeIndex);
    fs::path nativePath(App::AppConfig &config, std::string nativeIndex);

    static LibraryDownloads fromJson(const rapidjson::Value &obj);
  };

  /*
    I have to further investigate about this
  */
  struct LibraryExtractRules {
    std::vector<std::string> exclude;
    std::vector<std::string> include;

    static LibraryExtractRules fromJson(const rapidjson::Value &obj);
  };

  struct Library {
    LibraryDownloads downloads;
    std::string name;
    std::vector<Rule> rules;
    std::map<OS, std::string> natives;
    LibraryExtractRules extract;

    static Library fromJson(const rapidjson::Value &obj);
    void downloadArtifact(App::AppConfig &config);
    void downloadNative(App::AppConfig &config);
    void extractNative(App::AppConfig &config, fs::path instanceDir);
  };

  struct LoggingClient {
    std::string argument;
    Download file;
    std::string type;

    static LoggingClient fromJson(const rapidjson::Value &obj);
  };

  struct LoggingInfo {
    LoggingClient client;

    static LoggingInfo fromJson(const rapidjson::Value &obj);
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

  static Client fromJson(const rapidjson::Value &obj);

  void downloadAssets(App::AppConfig &config);
  void downloadLibraries(App::AppConfig &config, fs::path instanceDir);
  void downloadClientDownloads(fs::path instanceDir);
  void download(App::AppConfig &config, fs::path instanceDir);
  std::vector<fs::path> getLibrariesList(App::AppConfig &config);
};
