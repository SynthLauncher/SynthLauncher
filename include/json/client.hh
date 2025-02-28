#pragma once

#include "include/entities/arch.hh"
#include "include/entities/os.hh"
#include <cstdint>
#include <optional>
#include <simdjson.h>
#include <string>
#include <vector>

class Client {
public:
  struct Features {
    bool isDemoUser = false;
    bool hasCutomResolution = false;
    bool hasQuickPlaysSupport = false;
    bool isQuickPlaySingleplayer = false;
    bool isQuickPlayMultiplayer = false;
    bool isQuickPlayRealms = false;
  };

  struct OSRules {
    OperatingSystem::OS name;
    Architecture::Arch arch;
    std::string version;
  };

  struct Rule {
    std::string action;
    OSRules os;
    Features features;

    bool osMatches();
    bool osMatches(std::vector<Rule> rules);
  };

  struct Argument {
    std::string value;
    std::vector<std::string> values;
    std::vector<Rule> rules;
  };

  struct Arguments {
    std::vector<Argument> game;
    std::vector<Argument> jvm;
  };

  class ArgumentDeserializer {
  public:
  private:
    Features deserialize_features(simdjson::ondemand::object &obj);
    OSRules deserialize_os_rules(simdjson::ondemand::object &obj);
    Rule deserialize_rule(simdjson::ondemand::object &obj);
  };

  struct Download {
    std::string id;
    std::string path;
    std::string sha1;
    long size;
    std::optional<long> totalSize;
    std::string url;

    std::vector<uint8_t> fetch();
  };

  struct ClientDownloads {};

  struct JavaVersion {};

  struct LibraryDownloads {};

  struct LibraryExtractRules {};

  struct Library {};

  struct LoggingClient {};

  struct LoggingInfo {};
};