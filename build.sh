emcmake cmake -GNinja -Bbin -DCMAKE_BUILD_TYPE=Release
cmake --build bin
cp bin/index.* docs