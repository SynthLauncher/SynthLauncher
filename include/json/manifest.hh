#include <string>

class Manifest {
public:
    class Latest {
    public:
        std::string release;
        std::string snapshot;
    };

    class Version {
    public:
        std::string id;
        std::string type;
        std::string url;
        std::string time;
        std::string releaseTime;
    };

    Latest latest;
    Version version[];
};