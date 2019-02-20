#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQuickStyle>
#include <QIcon>
#include <QStandardPaths>
#include <QFile>
#include <QDir>
#include <QQmlContext>
#include <QFontDatabase>
#include <QDebug>
#include <QScreen>
#include "Bindings.h"

int main(int argc, char *argv[])
{
    QGuiApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
    // forces use of angle gl backend
    // http://blog.qt.io/blog/2014/11/27/qt-weekly-21-dynamic-opengl-implementation-loading-in-qt-5-4/
    // todo make this conditional on a gpu check or something?
    // opengl snafu
    // https://bugreports.qt.io/browse/QTBUG-54451
    QCoreApplication::setAttribute(Qt::AA_UseOpenGLES);
    QQuickStyle::setStyle("fusion");
    QGuiApplication app(argc, argv);
    app.setWindowIcon(QIcon(":/ApplicationIcon"));
    QFontDatabase::addApplicationFont(":/res/Roboto-Regular-latin-20.woff2");
    QFontDatabase::addApplicationFont(":/res/Roboto-Regular-latin-24.woff2");
    qmlRegisterType<Repositories>("RustCode", 1, 0, "Repositories");
    qmlRegisterType<Git>("RustCode", 1, 0, "Git");
    qmlRegisterType<Branches>("RustCode", 1, 0, "Branches");
    qmlRegisterType<Log>("RustCode", 1, 0, "Log");
    qmlRegisterType<Commit>("RustCode", 1, 0, "Commit");
    qmlRegisterType<Diffs>("RustCode", 1, 0, "Diffs");

#if DEBUG
    QDir dataPath = QDir(QGuiApplication::applicationDirPath());
#else
    QDir dataPath = QDir(QStandardPaths::writableLocation(QStandardPaths::AppLocalDataLocation));
#endif

    // seems qFile will not create the file if some part of the path is missing.
    // mkpath should ensure all parent folders in the path exists.
    if (!dataPath.exists()) {
        dataPath.mkpath(dataPath.path());
    }
    QString dbFilePath = dataPath.filePath("gitbumr.sqlite");
    QFile dbFile(dbFilePath);
    if (!dbFile.exists()) {
        if (dbFile.open(QIODevice::WriteOnly)) {
            dbFile.close();
        } else {
            qDebug() << "Db file did not exist, and could not create it";
        }
    }
    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty("DatabaseFileName", dbFilePath);
    engine.rootContext()->setContextProperty("MAX_U32_INT", quint32(4294967295));
    // Using Windows 10, plain desktop no scaling, DevicePixelRatio is 96
    engine.rootContext()->setContextProperty("LogicalDpi", QGuiApplication::primaryScreen()->logicalDotsPerInch());
    engine.load(QUrl(QStringLiteral("qrc:/qml/main.qml")));

#if DEBUG
    if (engine.rootObjects().isEmpty())
        return -1;
#endif

    return app.exec();
}
