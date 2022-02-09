# MoAlyousef

A demo project using [Livid](https://github.com/MoAlyousef/livid) for the frontend. 

The site can be visited here:

https://moalyousef.github.io/site/

## Building locally
You need CMake, git and a working installation of Emscripten, installation instructions can be found [here](https://emscripten.org/docs/getting_started/downloads.html):
```
$ git clone https://github.com/MoAlyousef/site
$ cd site
$ emcmake cmake -S. -Bbin -DCMAKE_BUILD_TYPE=Release
$ cmake --build bin
```

This produces 3 index files (index.html, index.js, index.wasm) in the bin directory.
You can copy the files into the docs directory. Assuming you're running a Posix system:
```
$ cp bin/index.* docs
```
On Windows:
```
$ copy bin\index.* docs
```

Then run a server to serve them:
```
$ python3 -m http.server --dir docs
```
Or use emrun (part of the emsdk):
```
$ emrun docs
```