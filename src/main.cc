#include "include/config/app.hh"
#include "include/entities/instance.hh"
#include "include/json/client.hh"
#include <iostream>

int main() {
  // Crashes after starting idk why
  std::cout << "Starting... \n";
  App::AppConfig appConfig = App::initAppConfig();
  App::initLauncherDir(appConfig);

  std::cout << "Initialized! \n";

  rapidjson::Document doc = rapidjson_utils::fromJson("../assets/25w03a.json");

  Client cli = Client::fromJson(doc);
  Instance::init(appConfig);

  std::cout << "Instance dir: " << Instance::PARENT_DIR << std::endl;

  Instance instance = Instance::createInstance("test4", "1.21");

  std::cout << "Instance path: " << instance.dir() << std::endl;

  return 0;
}
