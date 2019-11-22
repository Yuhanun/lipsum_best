# lipsum_best
Webserver for lipsum.best


## Installation ##
- Get vcpkg
- vcpkg install cpprestsdk
- vcpkg integrate install
- git clone https://github.com/Yuhanun/lipsum_best.git
- cd lipsum_best
- mkdir build
- cd build
- cmake .. -DCMAKE_TOOLCHAIN_FILE=/path/to/toolchain/vcpkg.cmake -Dcpprestsdk_DIR=/path/to/cpprestsdk/share
- make
- ./lipsum_best "http://*" 80
