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
message("Build configuration: $$BUILD_MODE")
QML_IMPORT_PATH="$$OUT_PWD/../lib/release"
QT += qml quick opengl quickcontrols2
CONFIG += c++14
CONFIG += qtquickcompiler # http://doc.qt.io/QtQuickCompiler/

DEFINES += QT_DEPRECATED_WARNINGS

SOURCES += main.cpp
RESOURCES += qml.qrc app.qrc

MT_DESTDIR = $$BUILD_MODE
!$$(QTCREATOR) { # set env in qtcreators build modes, avoids debug linking issues
    DESTDIR = bin
    MT_DESTDIR = bin
}

win32 {
    LIBS += WS2_32.lib Userenv.lib Advapi32.lib \
        winhttp.lib Rpcrt4.lib OLE32.LIB Userenv.lib user32.lib
    RC_FILE = res/gitbumr.rc
    # embed manifest
    QMAKE_POST_LINK = pushd $$MT_DESTDIR & \
        mt  -nologo -manifest $$PWD/res/gitbumr.exe.manifest \
            -outputresource:gitbumr.exe;1
}
