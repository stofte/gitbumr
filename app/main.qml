import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import "components"

ApplicationWindow {
    visible: true
    width: 450
    height: 580
    FontLoader { id: mainFont; name: "Segoe UI" }
    Page {
        anchors.fill: parent
        header: ToolBar {
            Label {
                anchors.fill: parent
                text: qsTr("gitbumr")
                font.pixelSize: 11
                font.family: mainFont.name
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }
        }
        RepoView { }
    }
}
