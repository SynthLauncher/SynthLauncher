#pragma once

#include <string>

#include <vector>

#define USER_AGENT "SynthLauncher/1.0.0"

#define HTTP_ENDLINE "\r\n"

struct Header {
  std::string key;
  std::string value;

  Header() {}
  Header(const std::string key, const std::string value)
      : key{key}, value{value} {}
};

struct HttpRequest {
  std::string method;
  std::string uri;
  std::string httpVersion = "HTTP/1.1";
  std::vector<Header> header;
  std::string body;
};

std::string buildRequest(HttpRequest &req);