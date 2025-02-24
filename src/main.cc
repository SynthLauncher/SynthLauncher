#include <iostream>
#include "include/config/app.hh"

int main() {
  AppConfig config = initializeAppConfig();

  initializeLauncherDir(config);

  return 0;
}
