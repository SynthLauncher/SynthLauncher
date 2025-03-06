#include "include/config/app.hh"
#include "include/json/client.hh"
#include <iostream>


int main() {
  App::AppConfig appConfig = App::initAppConfig();
  App::initLauncherDir(appConfig);

  return 0;
}
