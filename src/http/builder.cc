#include "include/http/builder.hh"

#include <sstream>

#include <string>

std::string buildRequest(HttpRequest &req) {

  std::stringstream ss;

  req.header.push_back(Header("User-Agent", USER_AGENT));

  req.header.push_back(Header("Content-Type", "application/json"));

  ss << req.method << " " << req.uri << " " << req.httpVersion << HTTP_ENDLINE;

  for (const auto &header : req.header) {

    ss << header.key << ": " << header.value << HTTP_ENDLINE;
  }

  if (!req.body.empty()) {

    ss << "Content-Length: " << std::to_string(req.body.size()) << HTTP_ENDLINE;

    ss << req.body;
  }

  ss << HTTP_ENDLINE;

  return ss.str();
}