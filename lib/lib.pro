#-------------------------------------------------
#
# Project created by QtCreator 2019-02-22T15:56:24
#
#-------------------------------------------------

QT += qml quick quickcontrols2
TARGET = rust
TEMPLATE = lib
CONFIG += c++14 plugin

# The following define makes your compiler emit warnings if you use
# any feature of Qt which has been marked as deprecated (the exact warnings
# depend on your compiler). Please consult the documentation of the
# deprecated API in order to know how to port your code away from it.
DEFINES += QT_DEPRECATED_WARNINGS

# You can also make your code fail to compile if you use deprecated APIs.
# In order to do so, uncomment the following line.
# You can also select to disable deprecated APIs only up to a certain version of Qt.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

SOURCES += \
        Bindings.cpp \
        rustcode_plugin.cpp

HEADERS += \
        Bindings.h \
        rustcode_plugin.h

DISTFILES = qmldir

BUILD_MODE=debug
CCRS_CFLAGS=pwd # noop
RUST_TARGET=x86_64-pc-windows-msvc
linux:RUST_TARGET=x86_64-unknown-linux-gnu
CONFIG(debug, debug|release) {
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
    # https://doc.qt.io/qt-5/deployment-plugins.html#loading-and-verifying-plugins-dynamically
    # force release mode for the plugin
    CONFIG += release
    win32:CCRS_CFLAGS = cd & set CFLAGS=/MD
}

DESTDIR = $$BUILD_MODE/RustCode
# Note that this is the rust "blob" itself,
# so it's not in placed in the DESTDIR folder.
LIBS += -L"$$PWD/target/$$RUST_TARGET/$$BUILD_MODE" -lrust
#message("LIBS => $$PWD/target/$$RUST_TARGET/$$BUILD_MODE")
win32 {
    LIBS += WS2_32.lib Userenv.lib Advapi32.lib Shell32.lib \
        winhttp.lib Rpcrt4.lib OLE32.LIB Userenv.lib user32.lib
}

# if you are using Shadow build, you need to get the output folder
CONFIG(release, debug|release): DESTDIR = $$OUT_PWD/release/RustCode
CONFIG(debug, debug|release): DESTDIR = $$OUT_PWD/debug/RustCode

#CONFIG(release, debug|release): DESTDIR = release
#CONFIG(debug, debug|release): DESTDIR = debug

# rust lib, requires cargo in path
RUST_FILES = \
    src/lib.rs \
    src/interface.rs \
    src/utils.rs \
    src/implementation/mod.rs \
    src/implementation/branches.rs \
    src/implementation/git.rs \
    src/implementation/repositories.rs \
    src/implementation/log.rs \
    src/implementation/commit.rs \
    src/implementation/hunks.rs \
    src/implementation/diffs.rs

#copyshit.commands = "copy $$PWD/qmldir $$DESTDIR/qmldir"

rust_cargo.output = "$$PWD/target/$$RUST_TARGET/$$BUILD_MODE/rust.lib"
rust_cargo.commands = $$CCRS_CFLAGS & \
    cargo build --manifest-path="$$PWD/Cargo.toml" --target=$$RUST_TARGET $$CARGO_FLAG
rust_cargo.input = RUST_FILES
# adding this seems to get executed on "clean", but seems to expect a file list.
# we work around this by adding a bogus file, and then adding the cargo clean cmd.
# again, qmake/qtcreator can't see the full build graph, but at least this will
# allow a clean build.
# rust_cargo.clean = this_file_is_not_here & cargo clean --manifest-path="$$PWD/lib/Cargo.toml"
QMAKE_EXTRA_COMPILERS += rust_cargo
#QMAKE_POST_LINK = copy qmldir $$DESTDIR/qmldir
# DISTFILES is only used when dist target is run via make. Just do a basic copy instead.
#system($$system_quote(copy $$PWD/qmldir $$DESTDIR/qmldir))

copydata.commands = copy $$shell_path($$PWD/qmldir) $$shell_path($$DESTDIR/qmldir)
first.depends = $(first) copydata
export(first.depends)
export(copydata.commands)
QMAKE_EXTRA_TARGETS += first copydata
