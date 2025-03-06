#include <iostream>
#include "include/config/app.hh"
#include "include/entities/instance.hh"

int main() {
  App::AppConfig appConfig = App::initAppConfig();
  App::initLauncherDir(appConfig);

  return 0;
}
