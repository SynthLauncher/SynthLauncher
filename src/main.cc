#include "include/config/app.hh"
#include "include/json/client.hh"
#include "include/entities/instance.hh"
#include <iostream>


int main() {
  App::AppConfig appConfig = App::initAppConfig();
  App::initLauncherDir(appConfig);

  rapidjson::Document doc = rapidjson_utils::fromJson("E:/OneDrive/Desktop/SynthLauncher/assets/25w03a.json");

  Client cli = Client::fromJson(doc);
  Instance::init(appConfig);

  Instance instance = Instance::createInstance("test1", "1.21");

  return 0;
}
