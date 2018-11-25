TEMPLATE = app
TARGET = gitbumr
CCRS_CFLAGS = pwd # noop
CONFIG(debug, debug|release) {
    BUILD_MODE=debug
    DEFINES += DEBUG
    # the cc-rs crate looks at the CFLAGS, but curl breaks 
    # on the UNICODE define from the full qt CFLAGS definition (on win32)
    # so we include just the parts we care about (the crt switches.)
    # the cd seems to workaround issue with set being the first cmd?
    win32:CCRS_CFLAGS = cd & set CFLAGS=/MDd
}
CONFIG(release, release|debug) {
    BUILD_MODE=release
    CARGO_FLAG=--release
    win32:CCRS_CFLAGS = cd & set CFLAGS=/MD
}
message("Builde mode: $$BUILD_MODE")

QT += qml quick opengl quickcontrols2
CONFIG += c++14
CONFIG += qtquickcompiler # http://doc.qt.io/QtQuickCompiler/

DEFINES += QT_DEPRECATED_WARNINGS

SOURCES += app/main.cpp app/Bindings.cpp
HEADERS += app/Bindings.h
RESOURCES += app/qml.qrc app/app.qrc

RUST_TARGET=x86_64-pc-windows-msvc
LIBS += -L"$$PWD/lib/target/$$RUST_TARGET/$$BUILD_MODE" -lrust

MT_DESTDIR = $$BUILD_MODE
!$$(QTCREATOR) { # set env in qtcreators build modes, avoids debug linking issues
    DESTDIR = bin
    MT_DESTDIR = bin
}

win32 {
    LIBS += WS2_32.lib Userenv.lib Advapi32.lib \
        winhttp.lib Rpcrt4.lib OLE32.LIB Userenv.lib user32.lib
    RC_FILE = app/res/gitbumr.rc
    # embed manifest
    QMAKE_POST_LINK = pushd $$MT_DESTDIR & \
        mt  -nologo -manifest $$PWD/app/res/gitbumr.exe.manifest \
            -outputresource:gitbumr.exe;1
}

# rust lib, requires cargo in path
RUST_FILES = \
    lib/src/lib.rs \
    lib/src/git.rs \
    lib/src/interface.rs \
    lib/src/utils.rs \
    lib/src/implementation/mod.rs \
    lib/src/implementation/repositories.rs \
    lib/src/implementation/history.rs
rust_cargo.output = "$$PWD/lib/target/$$RUST_TARGET/$$BUILD_MODE/rust.lib"
rust_cargo.commands = $$CCRS_CFLAGS & \
    cargo build --manifest-path="$$PWD/lib/Cargo.toml" --target=$$RUST_TARGET $$CARGO_FLAG
rust_cargo.input = RUST_FILES
# adding this seems to get executed on "clean", but seems to expect a file list.
# we work around this by adding a bogus file, and then adding the cargo clean cmd.
# again, qmake/qtcreator can't see the full build graph, but at least this will 
# allow a clean build.
rust_cargo.clean = this_file_is_not_here & cargo clean --manifest-path="$$PWD/lib/Cargo.toml"
QMAKE_EXTRA_COMPILERS += rust_cargo
