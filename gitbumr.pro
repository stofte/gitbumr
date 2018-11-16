TEMPLATE = app
TARGET = gitbumr

QT += qml quick opengl quickcontrols2
CONFIG += c++14

DEFINES += QT_DEPRECATED_WARNINGS

SOURCES += app/main.cpp app/Bindings.cpp
HEADERS += app/Bindings.h
RESOURCES += app/qml.qrc

LIBS += -L"$$PWD/lib/target/release" -lrust
DESTDIR = bin
MOC_DIR = obj
OBJECTS_DIR  = obj
win32: LIBS += WS2_32.lib Userenv.lib Advapi32.lib

# rust lib, requires cargo in path
RUST_FILES = \
    "$$PWD/lib/src/lib.rs" \
    "$$PWD/lib/src/interface.rs" \
    "$$PWD/lib/src/implementation.rs"
rust_cargo.output = "$$PWD/lib/target/release/rust.lib"
rust_cargo.commands = cargo build --manifest-path="$$PWD/lib/Cargo.toml" --release
rust_cargo.input = RUST_FILES
QMAKE_EXTRA_COMPILERS += rust_cargo
