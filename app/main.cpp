#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQuickStyle>
#include <QIcon>
#include "Bindings.h"

int main(int argc, char *argv[])
{
    QGuiApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
    QQuickStyle::setStyle("fusion");
    QGuiApplication app(argc, argv);
    app.setWindowIcon(QIcon(":/ApplicationIcon"));
    qmlRegisterType<Repositories>("RustCode", 1, 0, "Repositories");

    QQmlApplicationEngine engine;
    engine.load(QUrl(QStringLiteral("qrc:/main.qml")));
    if (engine.rootObjects().isEmpty())
        return -1;

    return app.exec();
}
