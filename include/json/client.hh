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

    Features deserialize(simdjson::ondemand::object &obj);
  };

  struct OSRules {
    std::optional<OperatingSystem::OS> name;
    std::optional<Architecture::Arch> arch;
    std::string version;

    OSRules deserialize(simdjson::ondemand::object &obj);
  };
};
