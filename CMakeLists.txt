cmake_minimum_required(VERSION 3.14)
project(lipsum_best)

find_package(cpprestsdk CONFIG REQUIRED NAMES)
find_package(Threads)

include_directories(${OpenSSL_INCLUDE_DIRS})
set(OPENSSL_USE_STATIC_LIBS TRUE)

set(CMAKE_CXX_STANDARD 17)

include_directories(include)

add_executable(lipsum_best
               main.cpp)
               
set(CMAKE_BUILD_TYPE Debug)

set(EXECUTABLE_OUTPUT_PATH "./")

if(MSVC)
  target_compile_options(lipsum_best PRIVATE /W4 /WX /WERROR /pedantic)
else()
  target_compile_options(lipsum_best PRIVATE -Wall -Wextra -Werror -pedantic)
endif()

target_link_libraries(lipsum_best PUBLIC ${OpenSSL_LIBRARIES})
target_link_libraries(lipsum_best PUBLIC OpenSSL::SSL)
target_link_libraries(lipsum_best PUBLIC OpenSSL::Crypto)
target_link_libraries(lipsum_best PUBLIC Threads::Threads)
target_link_libraries(lipsum_best PRIVATE cpprestsdk::cpprest cpprestsdk::cpprestsdk_zlib_internal cpprestsdk::cpprestsdk_boost_internal cpprestsdk::cpprestsdk_openssl_internal)