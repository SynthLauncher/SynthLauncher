#pragma once

#include <map>
#include <cstdint>
#include <filesystem>
#include <optional>
#include <string>
#include <vector>

#define CPPHTTPLIB_OPENSSL_SUPPORT

#include "include/entities/os.hh"
#include "include/entities/arch.hh"
#include "include/rapidjson/document.h"
#include "include/rapidjson/error/en.h"
#include "include/config/app.hh"
#include "include/utils/httplib_utils.hh"

#undef GetObject 

namespace fs = std::filesystem;

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
    bool osMatches(AppConfig &config, std::vector<Rule> rules);
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

    LibraryDownloads parse(rapidjson::Value &obj);
  };
};
