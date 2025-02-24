#pragma once

#include "include/http/builder.hh"

#include <string>

#include <vector>

struct HttpResponse {
  std::string httpVersion;
  std::string httpCode;
  std::vector<Header> header;
  std::string body;
};

HttpResponse parseResponse(std::string &res);