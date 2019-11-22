#include "server.hpp"

#include "utility.hpp"

namespace lipsum {
    Lipsum::Lipsum() {
        Lipsum::instance = this;
    }

    Lipsum::Lipsum(utility::string_t url)
        : listener{ url } {
        listener.support("GET", [this](auto const& val) {
            this->handle_get(val);
        });

        listener.support("PUT", [this](auto const& val) {
            this->handle_put(val);
        });

        listener.support("POST", [this](auto const& val) {
            this->handle_post(val);
        });

        listener.support("DELETE", [this](auto const& val) {
            this->handle_delete(val);
        });

        Lipsum::instance = this;
    }

    void Lipsum::handle_get(web::http::http_request message) {
        std::cout << message.to_string() << std::endl;
        message.reply(web::http::status_codes::OK);
    }

    void Lipsum::handle_put(web::http::http_request message) {
        std::cout << message.to_string() << std::endl;
        message.reply(web::http::status_codes::OK);
    }

    void Lipsum::handle_post(web::http::http_request message) {
        std::cout << message.to_string() << std::endl;
        message.reply(web::http::status_codes::OK, utils::to_json({ { "status", "ok" }, { "message", "" }, { "error", "" } }));
    }

    void Lipsum::handle_delete(web::http::http_request message) {
        std::cout << message.to_string() << std::endl;
        message.reply(web::http::status_codes::OK);
    }

    pplx::task<void> Lipsum::open() {
        return listener.open();
    }

    pplx::task<void> Lipsum::close() {
        return listener.close();
    }
}  // namespace lipsum