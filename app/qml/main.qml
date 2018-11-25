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
    FontLoader { id: mainFont; name: "Segoe UI" }

    property variant repoMgr;

    Repositories {
        id: repositoriesModel
        onActiveRepositoryChanged: {
            branchView.gitPath = activeRepository;
        }
    }

    Component.onCompleted: {
        // to actually cause repositoriesModel to be created on windows load,
        // the window using the model must be created on load.
        if (!repoMgr) {
            var component = Qt.createComponent("components/RepositoryManager.qml");
            repoMgr = component.createObject(root);
        }
    }

    Page {
        anchors.fill: parent
        header: ToolBar {
            RowLayout {
                anchors.fill: parent
                ToolButton {
                    text: qsTr("Repositories")
                    Layout.fillWidth: false
                    onClicked: {
                        repoMgr.show();
                    }
                }
            }
        }
        QQC14.SplitView {
            orientation: Qt.Horizontal
            anchors.fill: parent
            handleDelegate: Rectangle {
                color: Style.dark
                width: 1
            }
            BranchView {
                id: branchView
                Layout.fillHeight: true
                Layout.minimumWidth: 100
                Layout.preferredWidth: 200
            }
            History {
                Layout.fillHeight: true
            }
        }
    }
}
