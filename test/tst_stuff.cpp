#include <QtQuickTest>
#include <QQmlEngine>
#include <QQmlContext>
#include <QProcessEnvironment>

class Setup : public QObject
{
    Q_OBJECT

public:
    Setup() {}

public slots:
    void qmlEngineAvailable(QQmlEngine *engine)
    {
        auto env = QProcessEnvironment::systemEnvironment();
        auto gitPath = env.value("TST_GIT_PATH");
        engine->rootContext()->setContextProperty("TST_GIT_PATH", gitPath);
    }
};

QUICK_TEST_MAIN_WITH_SETUP(mytest, Setup)

#include "tst_stuff.moc"
