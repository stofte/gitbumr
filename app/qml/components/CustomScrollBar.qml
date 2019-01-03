import QtQuick 2.0
import QtQuick.Controls 2.4
import "../style"

ScrollBar {
    id: root
    property bool isVertival: orientation === Qt.Vertical
    property real stepButtonSize: isVertival ? width : height
    property real scrollContainerSize: 0
    property real scrollContainerSizePrev: 0
    property real scrollContentSize: 0
    property real pageScrollOverlapSize: 35
    property real pageScrollStepSize: (scrollContainerSize - pageScrollOverlapSize) / scrollContentSize
    property real scrollBarMinimumSize: 18
    // scrollTarget is assumed to point to the content being scrolled,
    // so we can detect when it changes, to reset the scroll position.
    property variant scrollTarget
    property bool captureMouseWheel: false
    // positive == right side of scrollbar,
    // negative == left side, translate as top/bottom for horizontal
    property bool capturePositiveSide: true
    // this is only used if the mousewheel is captured. the code assumes
    // the dimension contains the scrollbar itself (eg for vertical mode,
    // the code offsets by the scrollbar width)
    property real containerOtherSize: 0
    leftPadding: isVertival ? 2 : height
    rightPadding: isVertival ? 2 : height
    topPadding: isVertival ? width : 2
    bottomPadding: isVertival ? width : 2
    minimumSize: Math.min(0.5, scrollBarMinimumSize / (height))
    contentItem: Rectangle {
        color: !parent.enabled ? Style.window :
                parent.pressed ? Style.controlActive : Style.control
    }
    background: Rectangle {
        color: Style.window
    }
    // We adjusts the position of the scrollbar, on container resize,
    // to make the content flow back into view as would happen normally,
    // but this is used non-attached, and so we must do this ourselves.
    // todo: fix when not at ends
    onScrollContainerSizeChanged: {
        if (root.position > 0) {
            var offset = 1 - (root.position + root.size);
            if (offset < 0) {
                root.position += offset;
            }
        }
    }
    // Use `Keys.forwardTo: [scrollRef]` to send keys to the scrollbar to handle
    // tradtional keyboard keys for list scrolling/paging
    Keys.onPressed: {
        if (!root.visible || !root.enabled) return;
        var oldStepSize;
        if (event.key === Qt.Key_Down) {
            root.increase();
        } else if (event.key === Qt.Key_Up) {
            root.decrease();
        } else if (event.key === Qt.Key_PageDown) {
            oldStepSize = root.stepSize;
            root.stepSize = pageScrollStepSize;
            root.increase();
            root.stepSize = oldStepSize;
        } else if (event.key === Qt.Key_PageUp) {
            oldStepSize = root.stepSize;
            root.stepSize = pageScrollStepSize;
            root.decrease();
            root.stepSize = oldStepSize;
        } else if (event.key === Qt.Key_Home) {
            root.position = 0;
        } else if (event.key === Qt.Key_End) {
            oldStepSize = root.stepSize;
            root.stepSize = 1;
            root.increase();
            root.stepSize = oldStepSize;
        }
    }
    onScrollTargetChanged: {
        position = 0;
    }
    // hacky but this allows mousewheel to get captured
    MouseArea {
        enabled: captureMouseWheel
        x: -(containerOtherSize - (isVertival ? parent.width : parent.height))
        y: 0
        height: isVertival ? parent.height : containerOtherSize
        width: isVertival ? containerOtherSize : parent.height
        onWheel: {
            var scrollValue = isVertival ? wheel.angleDelta.y : wheel.angleDelta.x;
            var isDec = scrollValue > 0;
            var isMaxed = isDec && root.position === 0 || !isDec && (root.size + root.position >= 1);
            if (isMaxed) {
                wheel.accepted = false;
            } else {
                isDec ? root.decrease() : root.increase();
            }
        }
        onPressed: mouse.accepted = false
        onClicked: mouse.accepted = false
    }
    Timer {
        id: timerRef
        property bool isIncrease: false
        interval: 60;
        running: false;
        repeat: true
        triggeredOnStart: true
        onTriggered: isIncrease ? root.increase() : root.decrease()
    }
    function getIconPath(active) {
        return active ? "/res/svg/up-active.svg" : "/res/svg/up.svg"
    }
    Rectangle {
        x: 0
        y: 0
        height: stepButtonSize
        width: stepButtonSize
        color: Style.window
        Image {
            rotation: isVertival ? 0 : -90
            anchors.fill: parent
            source: getIconPath(incMouseRef.pressed)
        }
        MouseArea {
            id: incMouseRef
            anchors.fill: parent
            onPressed: root.decrease();
            onPressAndHold: {
                timerRef.restart();
                timerRef.isIncrease = false;
            }
            onReleased: timerRef.stop();
        }
    }
    Rectangle {
        x: isVertival ? 0 : parent.width - stepButtonSize
        y: isVertival ? parent.height - stepButtonSize : 0
        height: stepButtonSize
        width: stepButtonSize
        color: Style.window
        Image {
            rotation: isVertival ? 180 : 90
            anchors.fill: parent
            source: getIconPath(decMouseRef.pressed)
        }
        MouseArea {
            id: decMouseRef
            anchors.fill: parent
            onPressed: root.increase();
            onPressAndHold: {
                timerRef.restart();
                timerRef.isIncrease = true;
            }
            onReleased: timerRef.stop();
        }
    }
}
