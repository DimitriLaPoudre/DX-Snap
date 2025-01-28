#!/bin/sh

if [ "$1" = "build" ]; then
    dpkg-buildpackage -us -uc
    rm -rf debian/dx-snap
    rm -rf debian/.debhelper/
    rm -rf debian/files
    rm -rf debian/*.substvars
    rm -rf debian/debhelper-build-stamp
    rm -rf ../dx-snap_0.0-1.dsc
    rm -rf ../dx-snap_0.0-1.tar.gz
    rm -rf ../dx-snap_0.0-1_amd64.changes
    rm -rf ../dx-snap_0.0-1_amd64.buildinfo
    mv ../dx-snap_0.0-1_all.deb ./dx-snap.deb
elif [ "$1" = "clean" ]; then
    rm -rf $(find src -name '*.o')
elif [ "$1" = "fclean" ]; then
    rm -rf $(find src -name '*.o')
    rm -rf dx-snap
else
    CFLAGS="-Wall -O2"
    for cpp_file in $(find src -name '*.cpp'); do
        g++ $CFLAGS -c "$cpp_file" -o "${cpp_file%.cpp}.o"
    done
    g++ $CFLAGS $(find src -name '*.o') -o dx-snap
    ./build.sh clean
fi
