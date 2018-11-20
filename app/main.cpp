#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQuickStyle>
#include <QIcon>
#include "Bindings.h"

int main(int argc, char *argv[])
{
    QGuiApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
    // forces use of angle gl backend
    // http://blog.qt.io/blog/2014/11/27/qt-weekly-21-dynamic-opengl-implementation-loading-in-qt-5-4/
    // todo make this conditional on a gpu check or something?
    QCoreApplication::setAttribute(Qt::AA_UseOpenGLES);
    QQuickStyle::setStyle("fusion");
    QGuiApplication app(argc, argv);
    app.setWindowIcon(QIcon(":/ApplicationIcon"));
    qmlRegisterType<App>("RustCode", 1, 0, "App");
    qmlRegisterType<Repositories>("RustCode", 1, 0, "Repositories");

    QQmlApplicationEngine engine;
    engine.load(QUrl(QStringLiteral("qrc:/qml/main.qml")));
    if (engine.rootObjects().isEmpty())
        return -1;

    return app.exec();
}
