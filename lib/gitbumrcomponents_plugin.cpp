#include "gitbumrcomponents_plugin.h"
#include <qqml.h>
#include <QQuickItem>
#include <QUrl>

void GitbumrComponentsPlugin::registerTypes(const char *uri)
{
	// register basic rust types
    qmlRegisterType<Repositories>(uri, 1, 0, "Repositories");
    qmlRegisterType<Git>(uri, 1, 0, "Git");
    qmlRegisterType<Branches>(uri, 1, 0, "Branches");
    qmlRegisterType<Log>(uri, 1, 0, "Log");
    qmlRegisterType<Commit>(uri, 1, 0, "Commit");
    qmlRegisterType<Diffs>(uri, 1, 0, "Diffs");
    // register qml types
    const QString prefix = "qrc:/components/";
    qmlRegisterType(QUrl(prefix + "GitApp.qml"), uri, 1, 0, "GitApp");
}
