#pragma once
#include "cpprest/asyncrt_utils.h"
#include "cpprest/http_listener.h"
#include "cpprest/json.h"
#include "cpprest/uri.h"

namespace lipsum {
    class Lipsum {
    public:
        Lipsum();
        Lipsum(utility::string_t url);

        pplx::task<void> open();
        pplx::task<void> close();

        static inline Lipsum* instance;

    private:
        void handle_get(web::http::http_request message);
        void handle_put(web::http::http_request message);
        void handle_post(web::http::http_request message);
        void handle_delete(web::http::http_request message);


        web::http::experimental::listener::http_listener listener;
    };
}  // namespace lipsum