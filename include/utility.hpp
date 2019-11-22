#pragma once
#include <nlohmann/json.hpp>
#include <cpprest/json.h>


namespace lipsum::utils {
    inline web::json::value to_json(nlohmann::json const& val) {
        return web::json::value::parse(val.dump());
    }

    inline nlohmann::json to_json(web::json::value const& val) {
        return nlohmann::json::parse(val.serialize());
    }
}