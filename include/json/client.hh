#include <string>
#include <optional>
#include <simdjson.h>
#include "include/utils/simdjson_utils.hh"

class Client {
public:
  struct Features {    
    bool isDemoUser;
    bool hasCustomResolution;
    bool hasQuickPlaysSupport;
    bool isQuickPlaySingleplayer;
    bool isQuickPlayMultiplayer;
    bool isQuickPlayRealms;

    Features deserialize(simdjson::ondemand::object& obj);
  };
};
