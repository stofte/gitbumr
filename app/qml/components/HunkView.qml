import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import QtQuick.Controls 1.4 as QQC14
import QtGraphicalEffects 1.0
import QtQml 2.11
import "../base"
import "../style"

Rectangle {
    // The hunk listings contains alot of state, to make it usable, chiefly:
    // 1. The hunk id property is used to detect "resetting", when set as "",
    //    which happens in GitView.qml. This is done to limit the number of
    //    times we might otherwise recompute the height of the listview.
    // 2. With variable height listview items, alot of additional stuff needs
    //    to be handled by ourselves.
    // 3. The floating horizontal scrollbar is somewhat heavy handed as well,
    //    as whenever the size of the container, or the listview is scrolled,
    //    we need to requery for what items is currently at the bottom
    // 4. All of compounded by a variable height header!
    id: root
    property string filenameOld: ""
    property string filenameNew: ""
    property string statusText: ""
    property string hunkId: "" // commitsha + idx
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
    property int floatScrollBarIndex: -1;
    property real floatScrollBarOffset: 0;
    property int hunkItemLineCount: 0
    function getBottomElementIndex() {
        // 4.5 is fudged value to mark as "bottom" when
        // the scrollbar crosses the window bottom.
        var scrollOffset = -hunkListViewRef.y;
        var offset = -hunkListViewRef.y + (hunkListViewRectContainerRef.height + 4.5);
        floatScrollBarOffset = offset - 20
        return hunkListViewRef.indexAt(0, offset);
    }
    onHeightChanged: floatScrollBarIndex = getBottomElementIndex()
    onWidthChanged: floatScrollBarIndex = getBottomElementIndex()
    property real listContentHeight: 0
    onHunkIdChanged: {
        if (!hunkId) {
            listContentHeight = 0;
            hunksMainScrollRef.position = 0;
            return;
        }
        floatScrollBarIndex = getBottomElementIndex()
        listContentHeight = getHeight();
        hunkListViewRef.height = listContentHeight;
    }
    function getHeight() {
        var h = 0;
        var itemOtherH = 55;
        var itemLineH = 10;
        hunkItemLineCount = 0;
        for (var i = 0; i < gitModel.hunks.rowCount(); i++) {
            var elmIdx = gitModel.hunks.index(i, 0);
            // note the user role + offset, which is defined in Bindings.cpp.
            // its the json properties sorted alphabetically (and not as defined
            // in binding.json!)
            var linesOrigin = gitModel.hunks.data(elmIdx, 0x0108);
            var ba = new Uint8Array(linesOrigin);
            hunkItemLineCount += ba.length;
            h += ba.length * itemLineH;
            h += itemOtherH;
        }
        return h;
    }
    Rectangle {
        // The hunkListViewRef listview contains variable height elements. Even
        // with just a few elements in the list, Qt will compute the full height,
        // based on the elements loaded in the view (a slice of the full list).
        // since these are variable, the contentHeight of the container fluxes
        // and cannot be used for predictable layout. Instead we compute the
        // approximate height of the full list (based on the number of lines in
        // each hunk), and scroll by offsetting the y coordinate of the listview.
        // This gives a nice stable scrollbar that doesn't twitch when dragged.
        id: hunkListViewRectContainerRef
        anchors.top: headerRectBorderRef.bottom
        anchors.left: parent.left
        anchors.bottom: parent.bottom
        anchors.right: parent.right
        color: "transparent"
        clip: true
        ListView {
            id: hunkListViewRef
            model: gitModel.hunks
            y: -hunksMainScrollRef.position * height
            x: 0
            width: parent.width - hunksMainScrollRef.width
            onYChanged: root.floatScrollBarIndex = getBottomElementIndex()
            clip: true
            interactive: false
            delegate: Component {
                Item {
                    property bool isFloatingScrollBar: index === root.floatScrollBarIndex
                    property int indexProperty: index
                    property int itemLineCount: originsJsonModel.model.count || 0
                    property real itemContentHeight: height
                    id: hunkListRootItemRef
                    height: diffRef.contentHeight + 10 + hunkBotScrollRef.height + hunkTitleRectRef.height
                    width: parent.width
                    Rectangle {
                        color: "transparent"
                        clip: true
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
                            width: linesOldListRef.width + linesNewListRef.width + 5
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
                            clip: true
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
                            Rectangle {
                                id: diffContainerRectRef
                                width: diffRef.contentWidth + 15
                                height: diffRef.contentHeight
                                color: "transparent"
                                x: -hunkBotScrollRef.position * width
                                y: 0
                                ListView {
                                    anchors.fill: parent
                                    spacing: 0
                                    model: originsJsonModel.model
                                    interactive: false
                                    delegate: Component {
                                        Rectangle {
                                            height: 10
                                            width: 15
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
                                TextEdit {
                                    id: diffRef
                                    x: 15
                                    y: 0
                                    font.family: Style.fontNameFixedWidth
                                    readOnly: true
                                    selectByMouse: true
                                    text: hunk
                                }
                            }
                            CustomScrollBar {
                                id: hunkBotScrollRef
                                x: 0
                                z: 2
                                function getYOffset() {
                                    if (isFloatingScrollBar && enabled) {
                                        var mappedY = hunkListRootItemRef.mapFromItem(hunkListViewRef, 0, floatScrollBarOffset).y;
                                        var offset = mappedY - height - 9 + hunkListViewRef.contentY;
                                        return Math.max(10, offset);
                                    }
                                    return parent.height - height;
                                }
                                y: getYOffset()
                                policy: ScrollBar.AlwaysOn
                                orientation: Qt.Horizontal
                                height: 15
                                enabled: size < 1
                                width: parent.width
                                size: hunkListingsRectRef.width / diffContainerRectRef.width
                                stepSize: 1 / (hunkMaxLineLength * 0.5)
                                scrollContainerSize: parent.width
                                scrollContentSize: diffContainerRectRef.width
                            }
                        }
                    }
                }
            }
        }
        CustomScrollBar {
            id: hunksMainScrollRef
            x: parent.width - width
            y: 0
            width: visible ? 15 : 0
            visible: size < 1
            height: parent.height
            orientation: Qt.Vertical
            size: height / hunkListViewRef.height
            stepSize: 1 / (hunkItemLineCount * 0.5)
            scrollContainerSize: parent.height
            scrollContentSize: hunkListViewRef.height
            captureMouseWheel: true
            capturePositiveSide: false
            containerOtherSize: parent.width
        }
    }
}
