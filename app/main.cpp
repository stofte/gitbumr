#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQuickStyle>
#include <QIcon>
#include <QStandardPaths>
#include <QFile>
#include <QDir>
#include <QQmlContext>
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
    qmlRegisterType<App>("RustCode", 1, 0, "App");
    qmlRegisterType<Repositories>("RustCode", 1, 0, "Repositories");

#if DEBUG
    QString dataPath = QGuiApplication::applicationDirPath();
#else
    QString dataPath = QStandardPaths::writableLocation(QStandardPaths::AppLocalDataLocation);
#endif

    QString dbFilePath = QDir(dataPath).filePath("gitbumr.sqlite");
    QFile dbFile(dbFilePath);
    if (!dbFile.exists()) {
        dbFile.open(QIODevice::WriteOnly);
        dbFile.close();
    }
    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty("DatabaseFilePath", dbFilePath);
    engine.load(QUrl(QStringLiteral("qrc:/qml/main.qml")));

#if DEBUG
    if (engine.rootObjects().isEmpty())
        return -1;
#endif

    return app.exec();
}
