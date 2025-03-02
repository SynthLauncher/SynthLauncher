#include <string>
#include <optional>
#include <simdjson.h>
#include "include/utils/simdjson_utils.hh"
#include "include/entities/os.hh"
#include "include/entities/arch.hh"

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
};
