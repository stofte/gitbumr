import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import RustCode 1.0
import "qml/components"
import "qml/base"

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
                font.pixelSize: 12
                font.family: mainFont.name
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }
        }
        RepoView { }
    }
}
