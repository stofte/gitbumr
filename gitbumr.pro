TEMPLATE = app
TARGET = gitbumr

CONFIG(debug, debug|release):BUILD_MODE=debug
CONFIG(release, release|debug) {
    BUILD_MODE=release
    CARGO_FLAG=--release
}
message("Builde mode: $$BUILD_MODE")

QT += qml quick opengl quickcontrols2
CONFIG += c++14
CONFIG += qtquickcompiler # http://doc.qt.io/QtQuickCompiler/

DEFINES += QT_DEPRECATED_WARNINGS

SOURCES += app/main.cpp app/Bindings.cpp
HEADERS += app/Bindings.h
RESOURCES += app/qml.qrc

DESTDIR = bin
LIBS += -L"$$PWD/lib/target/$$BUILD_MODE" -lrust

win32 {
    LIBS += WS2_32.lib Userenv.lib Advapi32.lib
    RC_FILE = app/res/gitbumr.rc
    # embed manifest
    QMAKE_POST_LINK = pushd $$DESTDIR & \
        mt  -nologo -manifest $$PWD/app/res/gitbumr.exe.manifest \
            -outputresource:gitbumr.exe;1
}

# rust lib, requires cargo in path
RUST_FILES = \
    lib/src/lib.rs \
    lib/src/interface.rs \
    lib/src/implementation\mod.rs \
    lib/src/implementation\app.rs \
    lib/src/implementation\repositories.rs \
    lib/src/implementation\history.rs
rust_cargo.output = "$$PWD/lib/target/$$BUILD_MODE/rust.lib"
rust_cargo.commands = cargo build --manifest-path="$$PWD/lib/Cargo.toml" $$CARGO_FLAG
rust_cargo.input = RUST_FILES
QMAKE_EXTRA_COMPILERS += rust_cargo

