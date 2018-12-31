import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import QtQuick.Controls 1.4 as QQC14
import QtGraphicalEffects 1.0
import QtQml 2.11
import "../base"
import "../style"

Rectangle {
    property string filenameOld: ""
    property string filenameNew: ""
    property string statusText: ""
    property variant originList
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
    // see lib/src/implementation/hunk.rs for encoding of origin vals
    function mapOriginToChar(val) {
        switch (val) {
            case 0: return ' ';
            case 1: return '+';
            case 2: return '-';
            case 3: return '<';
            case 4: return '>';
            case 5: return '=';
            default:
                throw new Error("unhandled case: '" + val + "' in mapOriginToChar");
        }
    }
    function mapOriginToColor(val) {
        switch (val) {
            case 1: return '#DDFFDD';
            case 2: return '#FEE8E9';
            case 0:
            case 3:
            case 4:
            case 5:
                return '#FFFFFF';
            default:
                throw new Error("unhandled case: '" + val + "' in mapOriginToChar");
        }
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
                id: hunkListRootItemRef
                height: diffRef.contentHeight + 10 + hunkBotScrollRef.height + hunkTitleRectRef.height
                width: parent.width
                Rectangle {
                    color: "transparent"
                    width: parent.width
                    height: parent.height
                    TypedArrayListModel {
                        id: originsJsonModel
                        jsonArray: linesOrigin
                    }
                    Rectangle {
                        id: hunkTitleRectRef
                        anchors.top: parent.top
                        anchors.left: parent.left
                        width: parent.width
                        height: 20
                        color: "transparent"
                        TextItem {
                            x: 5
                            y: 4
                            opacity: 0.6
                            text: "Hunk " + (index + 1) + " : Lines " + linesNewFrom + "-" + linesNewTo
                        }
                    }
                    Rectangle {
                        id: hunkLinesViewRef
                        anchors.top: hunkTitleRectRef.bottom
                        anchors.left: parent.left
                        width: linesOldListRef.width + linesNewListRef.width + sigilViewRef.width + 5
                        height: parent.height
                        anchors.margins: 5
                        TypedArrayListModel {
                            id: linesOldJsonModel
                            byteSize: 32
                            jsonArray: linesOld
                        }
                        TypedArrayListModel {
                            id: linesNewJsonModel
                            byteSize: 32
                            jsonArray: linesNew
                        }
                        color: "transparent"
                        ListView {
                            id: linesOldListRef
                            anchors.top: parent.top
                            anchors.left: parent.left
                            spacing: 0
                            opacity: 0.6
                            width: 7 + linesOldCols * 6
                            height: diffRef.height
                            model: linesOldJsonModel.model
                            interactive: false
                            delegate: Component {
                                Rectangle {
                                    height: 10
                                    width: parent.width
                                    color: "transparent"
                                    TextItem {
                                        anchors.right: parent.right
                                        anchors.top: parent.top
                                        height: 10
                                        width: parent.width
                                        horizontalAlignment: Text.AlignRight
                                        font.family: Style.fontNameFixedWidth
                                        text: value === MAX_U32_INT ? " " : value
                                    }
                                }
                            }
                        }
                        ListView {
                            id: linesNewListRef
                            anchors.top: parent.top
                            anchors.left: linesOldListRef.right
                            spacing: 0
                            width: 7 + linesNewCols * 6
                            opacity: 0.6
                            height: diffRef.height
                            model: linesNewJsonModel.model
                            interactive: false
                            delegate: Component {
                                Rectangle {
                                    height: 10
                                    width: parent.width
                                    color: "transparent"
                                    TextItem {
                                        anchors.right: parent.right
                                        anchors.top: parent.top
                                        height: 10
                                        width: parent.width
                                        horizontalAlignment: Text.AlignRight
                                        font.family: Style.fontNameFixedWidth
                                        text: value === MAX_U32_INT ? " " : value
                                    }
                                }
                            }
                        }
                        Rectangle {
                            id: sigilViewRef
                            anchors.top: parent.top
                            anchors.right: parent.right
                            width: 15
                            height: diffRef.height
                            color: "white"
                            ListView {
                                anchors.fill: parent
                                spacing: 0
                                model: originsJsonModel.model
                                interactive: false
                                delegate: Component {
                                    Rectangle {
                                        height: 10
                                        width: parent.width
                                        color: mapOriginToColor(value)
                                        TextItem {
                                            y: 0
                                            anchors.right: parent.right
                                            anchors.rightMargin: 4
                                            font.family: Style.fontNameFixedWidth
                                            opacity: 0.6
                                            text: mapOriginToChar(value)
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Rectangle {
                        id: hunkListingsRectRef
                        anchors.top: hunkTitleRectRef.bottom
                        anchors.left: hunkLinesViewRef.right
                        height: diffRef.contentHeight + 15
                        width: parent.width - hunkLinesViewRef.width - 10
                        anchors.topMargin: 5
                        color: "white"
                        clip: true
                        ListView {
                            x: 0
                            y: 0
                            spacing: 0
                            width: parent.width
                            height: parent.height
                            model: originsJsonModel.model
                            interactive: false
                            delegate: Component {
                                Rectangle {
                                    height: 10
                                    width: parent.width
                                    color: mapOriginToColor(value)
                                }
                            }
                        }
                        TextEdit {
                            id: diffRef
                            x: -hunkBotScrollRef.position * contentWidth
                            y: 0
                            font.family: Style.fontNameFixedWidth
                            readOnly: true
                            selectByMouse: true
                            text: hunk
                        }
                        CustomScrollBar {
                            id: hunkBotScrollRef
                            x: 0
                            y: parent.height - height
                            policy: ScrollBar.AlwaysOn
                            orientation: Qt.Horizontal
                            height: 15
                            enabled: size < 1
                            width: parent.width
                            size: hunkListingsRectRef.width / diffRef.contentWidth
                            stepSize: 1 / (hunkMaxLineLength * 0.5)
                            scrollContainerSize: parent.width
                            scrollContentSize: diffRef.contentWidth
                        }
                    }
                }
            }
        }
    }
}
