#include "include/config/app.hh"
#include "include/entities/instance.hh"
#include "include/json/client.hh"
#include <iostream>

int main() {
  App::AppConfig appConfig = App::initAppConfig();
  App::initLauncherDir(appConfig);

  rapidjson::Document doc = rapidjson_utils::fromJson("../assets/25w03a.json");

  Client cli = Client::fromJson(doc);
  Instance::init(appConfig);

  Instance instance = Instance::createInstance("test5", "1.21");

  instance.install(appConfig);

  /*
  PS E:\OneDrive\Desktop\SynthLauncher\build> ./synthlauncher
  terminate called after throwing an instance of 'std::runtime_error'
    what():  Failed to open file:
  C:\Users\User\AppData\Roaming\SynthLauncher\assets\indexes\17\.json PS
  E:\OneDrive\Desktop\SynthLauncher\build>

  */
 
  return 0;
}
