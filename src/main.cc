#include <iostream>
#include <include/config/app.hh>

int main() {
  AppConfig config = initializeAppConfig();
  
  fs::create_directories(config.DIR);

  return 0;
}
