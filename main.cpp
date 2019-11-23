#include <iostream>
#include <thread>
#include <chrono>
#include <memory>

#include "server.hpp"

std::unique_ptr<lipsum::Lipsum> http_server;

void on_initialize(const utility::string_t& address) {
    web::http::uri_builder uri(address);
    auto addr = uri.to_uri().to_string();

    http_server = std::make_unique<lipsum::Lipsum>(addr);
    http_server->open().wait();

    std::cout << "Listening for requests on " << addr << std::endl;
}

void on_shutdown() {
    http_server->close().wait();
}

int main(int argc, char** argv) {
    if (argc < 3) {
        std::cout << "Invalid usage, usage: ./lipsum_best {address} {port}" << std::endl;
        return 1;
    }

    std::string address = argv[1];
    std::string port = argv[2];

    on_initialize(address + ":" + port);

    while (true) {
        std::this_thread::sleep_for(std::chrono::seconds(1));
    }

    on_shutdown();
    return 0;
}