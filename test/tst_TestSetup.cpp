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

    void cleanupTestCase() { }
};

QUICK_TEST_MAIN_WITH_SETUP(TestSetup, Setup)

#include "tst_TestSetup.moc"
