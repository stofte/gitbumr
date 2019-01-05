import QtQuick 2.0
import QtQuick.Controls 2.4
import "../style"

ScrollBar {
    // the qml scrollbar element lacks up/down arrrows, and has other non-desktoppy behavior,
    // such as going to position when the scrollbar gutter is clicked outside the tracker,
    // instead of paging down/up.
    // currently the implementation mimics traditional windows sematics of held
    // buttons or when paging (pressed outside scrollbar dragger itself):
    // 1. instantly step up/down view on mouse-down
    // 2. wait N millisecs
    // 3. if still held, step view down/up one row
    // 4. wait M millisecs
    // 5. goto step 3
    // 6. For scrollbar paging, the mouse coords is used to update the target position,
    //    at check point the scrollbar stops paging, until coords update again or user
    //    release the button.
    // Above with the assumption M < N, such that the user must wait a
    // perceptable amount before rows will start scrolling "fast".
    // Must of the stuff in here is to make stuff work in non-attached mode,
    // but component should work in either mode.
    id: root
    property bool isVertival: orientation === Qt.Vertical
    property bool debug: false
    property real stepButtonSize: isVertival ? width : height
    property real scrollContainerSize: 0
    property real scrollContainerSizePrev: 0
    property real scrollContentSize: 0
    property real pageScrollOverlapSize: 35
    property real pageScrollStepSize: (scrollContainerSize - pageScrollOverlapSize) / scrollContentSize
    property real scrollBarMinimumSize: 18
    // if the scrollbar is attached, the container will do this by itself
    property bool adjustPositionOnResize: true
    property real scrollPageStepSize: 0.2
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
    // stepButtonSize is a sensible default if not overwritten
    property real containerOtherSize: stepButtonSize
    leftPadding: isVertival ? 2 : height
    rightPadding: isVertival ? 2 : height
    topPadding: isVertival ? width : 2
    bottomPadding: isVertival ? width : 2
    minimumSize: Math.min(0.5, scrollBarMinimumSize / (isVertival ? height : width))
    contentItem: Rectangle {
        opacity: debug ? 0.5 : 1
        color: !parent.enabled ? Style.window :
                parent.pressed ? Style.controlActive : Style.control
    }
    property string debugColor: Style.window
    background: Rectangle {
        color: debug ? debugColor : Style.window
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
    Timer {
        id: timerRef
        property bool isIncrease: false
        // For scroll bar repeats, we want to stop when we come to the cursor
        // location, if the user is still holding the button.
        property bool scrollCheck: false
        property real scrollPosition: 0
        interval: 60;
        running: false;
        repeat: true
        triggeredOnStart: true
        onTriggered: {
            var skipStep = false;
            if (scrollCheck) {
                skipStep = isIncrease ? position + (stepSize / 2) > scrollPosition
                                      : position - (stepSize / 2) < scrollPosition;
            }
            if (!skipStep) {
                isIncrease ? root.increase() : root.decrease();
            }
        }
    }
    MouseArea {
        property int debugPadding: 0
        property real oneSide: -(containerOtherSize - stepButtonSize - debugPadding / 2)
        property real otherSide: debugPadding / 2 // == 0
        id: scrollMouseRef
        x: isVertival ? oneSide : otherSide
        y: isVertival ? otherSide : oneSide
        height: isVertival ? parent.height - debugPadding : (containerOtherSize - debugPadding)
        width: isVertival ? containerOtherSize - debugPadding : parent.width - debugPadding
        property real pressSavedStepSize
        function getScrollTargetPosition(mappedOffset) {
            // todo this attemps to figure out where the user clicked, it could be better
            var realWidth = parent.width - stepButtonSize * 2;
            var scrollSize = isVertival ? parent.height : parent.width;
            var adjustedMouseOffset = mappedOffset - stepButtonSize - size * scrollSize / 2;
            var scrollPos = adjustedMouseOffset / (scrollSize - stepButtonSize * 2);
            return scrollPos
        }
        onWheel: {
            if (!captureMouseWheel) {
                wheel.accepted = false
                return
            }
            var isDec = wheel.angleDelta.y > 0;
            var isMaxed = isDec && root.position === 0 || !isDec && (root.size + root.position >= 1);
            if (isMaxed) {
                wheel.accepted = false;
            } else {
                isDec ? root.decrease() : root.increase();
            }
        }
        onPressed: {
            // This code assumes that mouseareas for the step buttons will capture
            // events over them, so we wont have to check for those cases, as they
            // should never happen.
            // We manually check for the hit box detection of every click over
            // the container, if its in the scrollbar box, then we check if its
            // on the dragger or outside it. we do this "manually" (instead of contains),
            // since that considers padding and we dont want that.
            // Also explicitly de-accept events that should go through to actual
            // content, or the dragger itself for normal operation.
            var mapped = root.mapFromItem(scrollMouseRef, mouse.x, mouse.y);
            var inScroll = root.contains(mapped);
            if (inScroll) {
                var ci = root.contentItem;
                var draggerOffset = isVertival ? ci.y : ci.x;
                var draggerSize = isVertival ? ci.height : ci.width;
                var mappedOffset = isVertival ? mapped.y : mapped.x;
                var inDragger = inScroll && mappedOffset > draggerOffset && mappedOffset < draggerOffset + draggerSize;
                if (inScroll && !inDragger) {
                    debugColor = "red";
                    var isDec = mappedOffset <= draggerOffset;
                    // we set this in anticipation of the user keeping the button held
                    timerRef.isIncrease = !isDec;
                    timerRef.scrollPosition = getScrollTargetPosition(mappedOffset);
                    pressSavedStepSize = root.stepSize;
                    root.stepSize = root.pageScrollStepSize;
                    isDec ? root.decrease() : root.increase()
                } else {
                    mouse.accepted = false
                }
            } else {
                mouse.accepted = false
            }
        }
        // we should only come to these handlers in the case from onPressed where we
        // paged the view, so now we want to mass-page the view instead.
        onPressAndHold: {
            timerRef.scrollCheck = true;
            timerRef.restart();
        }
        onMouseXChanged: {
            var mapped = root.mapFromItem(scrollMouseRef, mouse.x, mouse.y);
            var mappedOffset = isVertival ? mapped.y : mapped.x;
            var newScrollPos = getScrollTargetPosition(mappedOffset);
            if (timerRef.isIncrease && newScrollPos > timerRef.scrollPosition ||
                !timerRef.isIncrease && newScrollPos < timerRef.scrollPosition)
            {
                timerRef.scrollPosition = newScrollPos;
            }
        }
        onReleased: {
            root.stepSize = pressSavedStepSize;
            timerRef.stop(); // if hold case triggered
        }
        Rectangle {
            visible: debug
            anchors.fill: parent
            color: isVertival ? "pink" : "orange"
            opacity: 0.3
        }
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
                timerRef.isIncrease = false;
                timerRef.scrollCheck = false;
                timerRef.restart();
            }
            onReleased: timerRef.stop();
            Rectangle {
                visible: debug
                anchors.fill: parent
                color: "blue"
                opacity: 0.5
            }
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
                timerRef.scrollCheck = false;
                timerRef.isIncrease = true;
                timerRef.restart();
            }
            onReleased: timerRef.stop();
            Rectangle {
                visible: debug
                anchors.fill: parent
                color: "yellow"
                opacity: 0.5
            }
        }
    }
}
