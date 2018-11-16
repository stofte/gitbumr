#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQuickStyle>
#include "Bindings.h"

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);
    app.setAttribute(Qt::AA_EnableHighDpiScaling);
    qmlRegisterType<Repositories>("RustCode", 1, 0, "Repositories");
    QQuickStyle::setStyle("fusion");

    QQmlApplicationEngine engine;
    engine.load(QUrl(QStringLiteral("qrc:/main.qml")));
    if (engine.rootObjects().isEmpty())
        return -1;

    return app.exec();
}
