import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import QtQuick.Controls 1.4 as QQC14
import QtGraphicalEffects 1.0
import QtQml 2.11
import "../base"
import "../style"

Item {
    // the qml scrollbar element lacks up/down arrrows, and has other non-desktoppy behavior,
    // such as going to position when the scrollbar gutter is clicked outside the tracker,
    // instead of paging down/up.
    // this implementation attempts to add in the up/down arrows, but keeps the non-paging
    // behavior for now.
    // currently the implementation mimics traditional windows sematics of held scrollbar buttons:
    // 1. instantly step up/down view on mouse-down
    // 2. wait N millisecs
    // 3. if still held, step view down/up one row
    // 4. wait M millisecs
    // 5. goto step 3
    // with the assumption M < N, such that the user must wait a
    // perceptable amount before rows will start scrolling "fast"

    id: root
    property real scrollSize: 0
    property real scrollPosition: 0
    property real nativeScrollPosition: 0
    property int scrollHeight: 100
    property int scrollWidth: 15
    property bool verticalMode: true

    width: scrollWidth
    height: scrollHeight

    signal step(bool down)
    signal positionChanged(real position)

    onScrollSizeChanged: {
        if (scrollSize >= 1 || isNaN(scrollSize)) {
            // hides scroller
            if (verticalMode) {
                scrollWidth = 0;
            } else {
                scrollHeight = 0;
            }
        } else {
            if (verticalMode) {
                scrollWidth = 15;
            } else {
                scrollHeight = 15;
            }
        }
    }

    Timer {
        id: scrollStepTimerTimeoutRef
        interval: 500; running: false; repeat: false
        onTriggered: {
            scrollStepTimerRef.running = true;
            scrollStepTimerRef.restart();
        }
    }

    Timer {
        id: scrollStepTimerRef
        interval: 60; running: false; repeat: true
        property bool isDown: false
        onTriggered: {
            root.step(isDown)
        }
    }

    Rectangle {
        id: stepUpRef
        anchors.top: parent.top
        anchors.left: parent.left
        height: 15
        width: 15
        color: Style.window
        Image {
            rotation: verticalMode ? 0 : -90
            anchors.fill: parent
            source: scrollUpMouseRef.pressed ? "/res/svg/up-active.svg" : "/res/svg/up.svg"
        }
        MouseArea {
            id: scrollUpMouseRef
            anchors.fill: parent
            onPressed: {
                root.step(false)
            }
            onPressedChanged: {
                if (pressed) {
                    scrollStepTimerRef.isDown = false;
                    scrollStepTimerTimeoutRef.start();
                } else {
                    scrollStepTimerTimeoutRef.stop();
                    scrollStepTimerRef.stop();
                }
            }
        }
    }

    ScrollBar {
        id: scrollerRef
        property bool manipulateList: false
        height: scrollHeight - (verticalMode ? 30 : 0)
        width: scrollWidth - (verticalMode ? 0 : 30)
        anchors.top: verticalMode ? stepUpRef.bottom : parent.top
        anchors.left: verticalMode ? parent.left : stepUpRef.right
        minimumSize: 0.02
        policy: ScrollBar.AlwaysOn
        size: scrollSize
        position: scrollPosition
        orientation: verticalMode ? Qt.Vertical : Qt.Horizontal
        background: Rectangle {
            color: Style.window
        }
        contentItem: Rectangle {
            color: scrollerRef.pressed ? Style.controlActive : Style.control
        }
        onPressedChanged: {
            manipulateList = scrollerRef.pressed;
        }
        onPositionChanged: {
            if (manipulateList) {
                root.positionChanged(scrollerRef.position);
            }
            nativeScrollPosition = position;
        }
    }

    Rectangle {
        anchors.top: verticalMode ? scrollerRef.bottom : parent.top
        anchors.left: verticalMode ? parent.left : scrollerRef.right
        height: 15
        width: 15
        color: Style.window
        Image {
            anchors.fill: parent
            rotation: verticalMode ? 180 : 90
            source: scrollDownMouseRef.pressed ? "/res/svg/up-active.svg" : "/res/svg/up.svg"
        }
        MouseArea {
            id: scrollDownMouseRef
            anchors.fill: parent
            onPressed: {
                root.step(true)
            }
            onPressedChanged: {
                if (pressed) {
                    scrollStepTimerRef.isDown = true;
                    scrollStepTimerTimeoutRef.start();
                } else {
                    scrollStepTimerTimeoutRef.stop();
                    scrollStepTimerRef.stop();
                }
            }
        }
    }
}
