#include "include/utils/simdjson_utils.hh"

template<typename T>
std::optional<T> get_optional(simdjson::ondemand::object& obj, std::string_view field_name) {
    auto field = obj[field_name];

    if (field.error()) return std::nullopt;
    
    T result;    
    if (field.get(result) == simdjson::SUCCESS) return result;

    return std::nullopt;
}

template<>
std::optional<std::string> get_optional<std::string>(simdjson::ondemand::object& obj, std::string_view field_name) {
    auto field = obj[field_name];
    
    if (field.error()) return std::nullopt;
    
    std::string_view result;
    if (field.get(result) == simdjson::SUCCESS) return std::string(result);
    
    return std::nullopt;
}

template<>
std::optional<simdjson::ondemand::object> get_optional<simdjson::ondemand::object>(simdjson::ondemand::object& obj, std::string_view field_name) {
    auto field = obj[field_name];
    if (field.error()) return std::nullopt;
    
    simdjson::ondemand::object result;
    if (field.get(result) == simdjson::SUCCESS) return result;

    return std::nullopt;
}

template<typename T>
T get_with_default(simdjson::ondemand::object& obj, std::string_view field_name, T default_value) {
    auto value = get_optional<T>(obj, field_name);
    return value.value_or(default_value);
}
