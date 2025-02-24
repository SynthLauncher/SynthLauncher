#include "include/http/parser.hh"

HttpResponse parseResponse(std::string &res) {

  HttpResponse response;

  std::vector<std::string> method;

  std::string ss;

  int pos = res.find('\n');

  for (int i = 0; i < pos; ++i) {

    if (res[i] == ' ' || (res[i] == '\r' && res[i + 1] == '\n')) {

      method.push_back(ss);

      ss.clear();

      if (res[i] == '\r') {

        break;
      }

    } else {

      ss.push_back(res[i]);
    }
  }

  response.httpVersion = method[0];

  response.httpCode = method[1];

  res.erase(res.begin(), res.begin() + pos + 1);

  std::vector<std::string> header;

  ss.clear();

  pos = res.find("\r\n\r\n");

  for (int i = 0; i < pos + 2; ++i) {

    if (res[i] == ':' && res[i + 1] == ' ') {

      header.push_back(ss);

      ss.clear();

    } else if (res[i] == '\r' && res[i + 1] == '\n') {

      header.push_back(ss);

      ss.clear();

      response.header.push_back(Header(header[0], header[1]));

      header.clear();

      ++i;

    } else if (res[i] == ' ' && res[i - 1] == ':') {

    } else {

      ss.push_back(res[i]);
    }
  }

  response.body = std::string(res.begin() + pos + 4, res.end());

  return response;
}