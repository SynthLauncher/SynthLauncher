#include "include/http/parser.hh"

#include <gtest/gtest.h>

#include <sstream>

#include <string>

TEST(Http, ParserTest) {

  std::stringstream ss;

  std::string response = "HTTP/1.1 201 Created\r\n"

                         "Content-Type: application/json\r\n"

                         "Location: http://example.com/users/123\r\n\r\n"

                         "{\n"

                         "\"message\": \"New user created\","

                         "\"user\": {\n"

                         "\"id\": 123,\n"

                         "\"firstName\": \"Example\",\n"

                         "\"lastName\": \"Person\",\n"

                         "\"email\": \"bsmth@example.com\"\n"

                         "}"

                         "  }";

  HttpResponse res = parseResponse(response);

  ASSERT_EQ(res.httpVersion, "HTTP/1.1");

  ASSERT_EQ(res.httpCode, "201");

  ASSERT_EQ(res.header[0].key, "Content-Type");

  ASSERT_EQ(res.header[0].value, "application/json");

  ASSERT_EQ(res.header[1].key, "Location");

  ASSERT_EQ(res.header[1].value, "http://example.com/users/123");
}