#include <iostream>
#include <simdjson.h>
#include "include/json/client.hh"
#include "include/utils/simdjson_utils.hh"

/*
  Debugging purposes for now
*/
int main() { 
    simdjson::ondemand::parser parser; 
    simdjson::padded_string json = simdjson::padded_string::load("E:/OneDrive/Desktop/SynthLauncher/assets/client_download.json");
    simdjson::ondemand::document doc = parser.iterate(json);

    simdjson::ondemand::object obj = doc["downloads"].get_object().value();

    Client::ClientDownloads downloads = Client::ClientDownloads::parse(obj);

    std::cout << downloads.client.sha1 << std::endl;

    return 0; 
}
