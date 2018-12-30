import QtQuick 2.9
import RustCode 1.0
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import "../base"
import "../style"

Item {
    property int rowHeight: 18
    ListView {
        id: diffListViewRef
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.bottom: parent.bottom
        anchors.right: diffScrollRef.left
        currentIndex: 0
        // todo: must be a better way to detect list has been reloaded/rebound.
        // this relies on the fact that currentIndex fires before currentItem,
        // but at least this resets the selected row when changing commits.
        property bool indexChanged: false
        onCurrentIndexChanged: {
            indexChanged = true
        }
        onCurrentItemChanged: {
            if (!indexChanged) {
                currentIndex = 0
            } else {
                indexChanged = false
            }
        }
        model: gitModel.diffs
        interactive: false
        highlightResizeDuration: 1
        highlightMoveDuration: 1
        highlightMoveVelocity: -1
        keyNavigationEnabled: true
        highlightFollowsCurrentItem: true
        highlight: Component{
            Item {
                z: 2
                height: rowHeight
                clip: true
                Rectangle{
                    anchors.fill: parent
                    color: Style.selection
                    TextItem {
                        x: 5
                        y: 3
                        color: "white"
                        text: diffListViewRef.currentItem ? diffListViewRef.currentItem.filenameText : ""
                    }
                }
            }
        }
        ScrollBar.vertical: ScrollBar {
            id: diffRealScrollRef
            width: 0
        }
        delegate: Component {
            Item {
                height: rowHeight
                property string filenameText: filename
                width: parent.width
                clip: true
                TextItem {
                    x: 5
                    y: 3
                    text: filename
                }
                MouseArea {
                    anchors.fill: parent
                    onPressed: {
                        diffListViewRef.currentIndex = index;
                        diffListViewRef.forceActiveFocus();
                    }
                    onWheel: {
                        var isDown = wheel.angleDelta.y < 0;
                        var topIdx = Math.max(0, diffListViewRef.indexAt(1, diffListViewRef.contentY + 1) + 3 * (isDown ? 1 : -1));
                        diffListViewRef.positionViewAtIndex(topIdx, ListView.Beginning);
                    }
                }
            }
        }
    }

    DesktopScrollbar {
        id: diffScrollRef
        anchors.top: parent.top
        anchors.right: parent.right
        scrollHeight: parent.height
        scrollSize: diffRealScrollRef.size
        scrollPosition: diffRealScrollRef.position
        onPositionChanged: {
            diffRealScrollRef.position = position
        }
        onStep: {
            var stepVal = down ? 1 : -1;
            var topIdx = Math.max(0, diffListViewRef.indexAt(1, diffListViewRef.contentY + 1) + stepVal);
            diffListViewRef.positionViewAtIndex(topIdx, ListView.Beginning);
        }
    }
}