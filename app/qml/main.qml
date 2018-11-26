import QtQuick 2.9
import QtQuick.Controls 2.3
import QtQuick.Controls 1.4 as QQC14
import QtQuick.Layouts 1.3
import RustCode 1.0
import "components"
import "base"
import "style"

ApplicationWindow {
    id: root
    visible: true
    width: 1000
    height: 600

    property variant repoMgr;

    Repositories {
        id: repositoriesModel
        onActiveRepositoryChanged: {
            gitView.gitPath = activeRepository;
            if (activeRepository) { // x = !!y doesnt seem to work to convert to boolean?
                noRepoView.visible = false;
            } else {
                noRepoView.visible = true;
            }
        }
    }

    Component.onCompleted: {
        // to actually cause repositoriesModel to be created on windows load,
        // the window using the model must be created on load.
        if (!repoMgr) {
            var component = Qt.createComponent("components/RepositoryManager.qml");
            repoMgr = component.createObject(root);
            if (!repositoriesModel.activeRepository) {
                noRepoView.visible = true;
            }
        }
    }

    Page {
        anchors.fill: parent
        header: ToolBar {
            topPadding: 5
            bottomPadding: 5
            RowLayout {
                anchors.fill: parent
                ToolButton {
                    font.family: Style.fontName
                    font.pointSize: Style.fontPointSize
                    text: qsTr("Repositories")
                    Layout.fillWidth: false
                    onClicked: {
                        repoMgr.show();
                    }
                }
            }
        }

        GitView { id: gitView }

        Pane {
            id: noRepoView
            anchors.fill: parent
            visible: false
            TextItem {
                anchors.centerIn: parent
                text: "No repository open"
            }
        }
    }

}
