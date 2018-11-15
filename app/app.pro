TEMPLATE = app

QT += qml quick opengl
CONFIG += c++14

DEFINES += QT_DEPRECATED_WARNINGS

SOURCES += main.cpp Bindings.cpp
HEADERS += Bindings.h
RESOURCES += qml.qrc

LIBS += -L"$$PWD/../lib/target/release" -lrust

win32 {
    DESTDIR = bin
    MOC_DIR = obj
    OBJECTS_DIR  = obj
    LIBS += WS2_32.lib Userenv.lib Advapi32.lib
}

# rust lib, requires cargo in path
RUST_FILES = \
    "$$PWD/../lib/src/lib.rs" \
    "$$PWD/../lib/src/interface.rs" \
    "$$PWD/../lib/src/implementation.rs"
rust_cargo.output = "$$PWD/../lib/target/release/rust.lib"
rust_cargo.commands = cargo build --manifest-path="$$PWD/../lib/Cargo.toml" --release
rust_cargo.input = RUST_FILES
QMAKE_EXTRA_COMPILERS += rust_cargo
