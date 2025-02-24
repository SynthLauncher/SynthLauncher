#include <map>
#include <string>
#include <filesystem>

namespace fs = std::filesystem;

class AssetIndex {
public:
    class AssetObject {
    public:
        std::string hash;

        std::string id();
        std::string url();
        fs::path path();
        void fetch();
    };

    std::map<std::string, AssetObject> objects;
};