#pragma once

#include <regex>
#include <string>
#include <stdexcept>

namespace httplib_utils {

inline std::pair<std::string, std::string>
extractHostAndPath(const std::string &url) {
  std::regex urlRegex(R"((https?://[^/]+)(/.*)?)");
  std::smatch matches;

  if (std::regex_match(url, matches, urlRegex)) {
    std::string hostWithScheme =
        matches[1].str();
        
    std::string path = matches[2].str(); 
    if (path.empty()) 
      path = "/"; 

    return {hostWithScheme, path};
  } else {
    throw std::invalid_argument("Invalid URL format: " + url);
  }
}
} // namespace httplib_utils