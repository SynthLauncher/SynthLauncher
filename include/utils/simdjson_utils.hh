#pragma once

#include <optional>
#include <simdjson.h>
#include <string>

namespace simdjson_utils {
template <typename T>
inline std::optional<T> get_optional(simdjson::ondemand::object &obj,
                                     std::string_view field_name) {
  auto field = obj[field_name];

  if (field.error())
    return std::nullopt;

  T result;
  if (field.get(result) == simdjson::SUCCESS)
    return result;

  return std::nullopt;
}

template <>
inline std::optional<std::string> get_optional(simdjson::ondemand::object &obj,
                                               std::string_view field_name) {
  auto field = obj[field_name];

  if (field.error())
    return std::nullopt;

  std::string_view result;
  if (field.get(result) == simdjson::SUCCESS)
    return std::string(result);

  return std::nullopt;
}

template <>
inline std::optional<simdjson::ondemand::object>
get_optional<simdjson::ondemand::object>(simdjson::ondemand::object &obj,
                                         std::string_view field_name) {
  auto field = obj[field_name];
  if (field.error())
    return std::nullopt;

  if (field.type().error() ||
      field.type() != simdjson::ondemand::json_type::object)
    return std::nullopt;

  simdjson::ondemand::object result;
  if (field.get(result) == simdjson::SUCCESS)
    return result;

  return std::nullopt;
}

template <>
inline std::optional<simdjson::ondemand::array> 
simdjson_utils::get_optional<simdjson::ondemand::array>(simdjson::ondemand::object& obj, 
                                                       std::string_view field_name) {
    auto field = obj[field_name];
    if (field.error()) return std::nullopt;
    if (field.type().error() || field.type() != simdjson::ondemand::json_type::array) 
        return std::nullopt;
    
    return field.get_array().value();
}

template <>
inline std::optional<int64_t> get_optional<int64_t>(simdjson::ondemand::object& obj,
                                                   std::string_view field_name) {
    auto field = obj[field_name];
    if (field.error()) return std::nullopt;
    
    int64_t result;
    if (field.get_int64().get(result) == simdjson::SUCCESS) return result;
    return std::nullopt;
}

template <>
inline std::optional<int> get_optional<int>(simdjson::ondemand::object& obj,
                                           std::string_view field_name) {
    auto value = get_optional<int64_t>(obj, field_name);
    return value ? std::optional<int>(static_cast<int>(*value)) : std::nullopt;
}


template <typename T>
inline T get_with_default(simdjson::ondemand::object &obj,
                          std::string_view field_name, T default_value) {
  auto value = get_optional<T>(obj, field_name);

  return value.value_or(default_value);
}

template <>
inline int64_t get_with_default<int64_t>(simdjson::ondemand::object& obj,
                                        std::string_view field_name,
                                        int64_t default_value) {
    return get_optional<int64_t>(obj, field_name).value_or(default_value);
}

template <typename T>
inline T get(simdjson::ondemand::object &obj, std::string_view field_name) {
  std::optional<T> value = get_optional<T>(obj, field_name);
  if (!value.has_value()) {
    throw std::runtime_error(std::string("Missing required field '") +
                             std::string(field_name) + "' or invalid type!");
  }

  return *value;
}
} // namespace simdjson_utils

