import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import "components"

ApplicationWindow {
    visible: true
    width: 450
    height: 580
    Page {
        anchors.fill: parent
        header: ToolBar {
            Label {
                anchors.fill: parent
                text: qsTr("QT Test")
                font.pixelSize: 20
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }
        }
        RepoView { }
    }
}
