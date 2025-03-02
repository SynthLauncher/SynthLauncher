#include <string>
#include <optional>
#include <cstdint>
#include <simdjson.h>

#define CPPHTTPLIB_OPENSSL_SUPPORT

#include "include/httplib.h"
#include "include/utils/httplib_utils.hh"
#include "include/utils/simdjson_utils.hh"
#include "include/entities/os.hh"
#include "include/entities/arch.hh"
#include "include/config/app.hh"

class Client {
public:
  struct Features {    
    bool isDemoUser;
    bool hasCustomResolution;
    bool hasQuickPlaysSupport;
    bool isQuickPlaySingleplayer;
    bool isQuickPlayMultiplayer;
    bool isQuickPlayRealms;

    static Features deserialize(simdjson::ondemand::object &obj);
  };

  struct OSRules {
    std::optional<OperatingSystem::OS> name;
    std::optional<Architecture::Arch> arch;
    std::string version;

    static OSRules deserialize(simdjson::ondemand::object &obj);
  };

  struct Rule {
    std::string action;
    std::optional<OSRules> os;
    std::optional<Features> features;

    static Rule deserialize(simdjson::ondemand::object &obj);
    bool osMatches(AppConfig &config);
    bool osMatches(AppConfig &config, std::vector<Rule> rules);
  };

  struct Argument {
    std::string value;
    std::vector<std::string> values;
    std::vector<Rule> rules;

    static Argument deserialize(simdjson::ondemand::value &val);
  };

  struct Arguments {
    std::vector<Argument> game;
    std::vector<Argument> jvm;

    static Arguments deserialize(simdjson::ondemand::object &obj);
  };

  struct Download {
    std::string id;
    std::string path;
    std::string sha1;
    int64_t size;
    std::optional<int64_t> totalSize;
    std::string url;

    static Download deserialize(simdjson::ondemand::object &obj);
    std::vector<std::uint8_t> fetch();
  };

  struct ClientDownloads {
    Download client;
    Download client_mappings;
    Download server;
    Download server_mappings;

    static ClientDownloads deserialize(simdjson::ondemand::object &obj);
  };
};
