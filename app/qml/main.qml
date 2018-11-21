import QtQuick 2.9
import QtQuick.Controls 2.3
import QtQuick.Controls 1.4 as QQC14
import QtQuick.Layouts 1.3
import RustCode 1.0
import "components"
import "base"
import "style"

ApplicationWindow {
    visible: true
    width: 450
    height: 580
    FontLoader { id: mainFont; name: "Segoe UI" }
    Page {
        App {
            id: appModel
        }
        anchors.fill: parent
        header: ToolBar {
            TextItem {
                anchors.fill: parent
                text: qsTr("gitbumr")
                font.pointSize: AppStyle.fontPointSize
                font.family: mainFont.name
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }
        }
        QQC14.SplitView {
            orientation: Qt.Horizontal
            anchors.fill: parent
            handleDelegate: Rectangle {
                color: AppStyle.dark
                width: 1
            }
            RepoView {
                id: repoView
                Layout.fillHeight: true
                Layout.preferredWidth: 200
            }

            History {
                Layout.fillHeight: true
                Layout.fillWidth: true
            }
        }
    }
}
