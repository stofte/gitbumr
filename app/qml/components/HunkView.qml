import QtQuick 2.9
import "../base"
import "../style"

Rectangle {
    property string filenameOld: ""
    property string filenameNew: ""
    property string statusText: ""
    onFilenameNewChanged: {
        console.log(filenameOld, filenameNew)
    }

    color: "transparent"
    Rectangle {
        property bool isComparison: filenameOld !== filenameNew
        id: headerRectRef
        height: 3 + (isComparison ? (fnBotRef.y + fnBotRef.contentHeight)
                                  : (fnTopRef.y + fnTopRef.contentHeight))
        width: parent.width
        anchors.top: parent.top
        anchors.left: parent.left
        clip: true
        color: "transparent"

        DiffStatusIcon {
            y: 1
            x: 0
            statusValue: headerRectRef.isComparison ? "Deleted" : statusText
            iconSize: 13
        }

        DiffStatusIcon {
            x: 0
            y: fnTopRef.y + fnTopRef.contentHeight
            visible: headerRectRef.isComparison
            statusValue: "Added"
            iconSize: 13
        }

        TextItem {
            id: fnTopRef
            x: 20
            y: 4
            width: parent.width - 30
            wrapMode: Text.WrapAnywhere
            text: headerRectRef.isComparison ? filenameOld : filenameNew
        }

        TextItem {
            id: fnBotRef
            x: 20
            y: fnTopRef.y + fnTopRef.contentHeight + 3
            visible: headerRectRef.isComparison
            width: parent.width - 30
            wrapMode: Text.WrapAnywhere
            text: filenameNew
        }
    }

    Rectangle {
        id: headerRectBorderRef
        anchors.top: headerRectRef.bottom
        width: parent.width
        height: 1
        color: Style.mid
        visible: false
    }

    ListView {
        model: gitModel.hunks
        anchors.top: headerRectBorderRef.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        clip: true
        delegate: Component {
            Item {
                height: diffRef.contentHeight + 10
                TextEdit {
                    id: diffRef
                    width: parent.width
                    font.family: Style.fontNameFixedWidth
                    //textFormat: Text.StyledText
                    readOnly: true
                    selectByMouse: true
                    text: hunk
                }
            }
        }
    }
}
