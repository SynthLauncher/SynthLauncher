#pragma once

#include <optional>
#include <string>
#include <simdjson.h>

namespace simdjson_utils {
    template <typename T>
    std::optional<T> get_optional(
        simdjson::ondemand::object &obj, 
        std::string_view field_name
    );

    template <>
    std::optional<std::string> get_optional<std::string>(
        simdjson::ondemand::object &obj, 
        std::string_view field_name
    );

    template <>
    std::optional<simdjson::ondemand::object> get_optional<simdjson::ondemand::object>(
        simdjson::ondemand::object &obj, 
        std::string_view field_name
    );

    template <typename T>
    T get_with_default( 
        simdjson::ondemand::object &obj, 
        std::string_view field_name, 
        T default_value
    );
}