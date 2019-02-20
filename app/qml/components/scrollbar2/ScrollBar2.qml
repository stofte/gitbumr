import QtQuick 2.11

Item {
    id: root
    // Scroll target
    property Item target
    property Item container
    property int orientation: Qt.Vertical
    // The width in vertical mode and height in horizontal.
    property real barSize: 15
    // the minimum length of the scrollbar dragger in qt pixels
    property real minimumLength: 20
    // Optional length of scrollbar itself, if not derived from container.
    property real barLength: shared.containerSize
    property real stepSize: 20
    property real pageStepSize: shared.containerSize - shared.stepSize
    property bool enableMouseWheel: shared.isVertical
    // Allow containers to load when at max position
    readonly property real position: shared.position
    readonly property bool maxPosition: shared.maxPosition
    height: shared.isVertical ? barLength : barSize
    width: shared.isVertical ? barSize : barLength
    enum TargetType {
        TextEdit,
        VirtualListView
    }
    enum ContainerType {
        Rectangle,
        VirtualListView
    }
    enum Update {
        Bar,
        Content,
        Container,
        Mouse,
        Cursor,
        Step
    }
    Timer {
        id: contentTimer
        interval: 3050
    }
    Timer {
        id: stepTimer
        interval: 350
        property int stepDirection: 1
        property bool paging: false
        property real pagingTarget
        onTriggered: {
            repeatTimer.start();
        }
    }
    Timer {
        id: repeatTimer
        interval: 40
        repeat: true
        onTriggered: {
            var stepSize = stepTimer.paging ? shared.pageStepSize : shared.stepSize;
            shared.updatePosition(ScrollBar2.Update.Step, stepTimer.stepDirection * stepSize);
        }
    }
    Item {
        id: shared
        property int type: 0
        property int containerType: 0

        readonly property bool isVertical: orientation === Qt.Vertical
        readonly property real containerSize: getContainerSize(false)
        readonly property real containerOtherSize: getContainerSize(true)
        readonly property real scrollMinimumLength: root.minimumLength
        readonly property real draggerOffset: isVertical ? dragger.y : dragger.x;
        readonly property real barSize: root.barLength - 2 * root.barSize
        readonly property real contentSize: getContentSize(false)
        readonly property real position: getPosition()
        readonly property real scrollBarSize: Math.max(scrollMinimumLength, realScrollBarSize)
        readonly property real realScrollBarSize: Math.min(barSize, barSize * scrollBarUnitSize)
        readonly property real scrollBarUnitSize: Math.min(1, containerSize / contentSize)
        readonly property real scrollMinLengthMod: (barSize - realScrollBarSize) / (barSize - scrollBarSize)

        readonly property real stepSize: root.stepSize / contentSize
        readonly property real pageStepSize: root.pageStepSize / contentSize

        property bool maxPosition: false
        property bool minPosition: false
        readonly property bool scrollRequired: !minPosition || !maxPosition
        property rect cursor
        property rect oldCursor
        property real oldContentSize: -1
        // Some of the float comparisons need some help, this value is specifically
        // for pixel usage when detecting if the dragger is at scroll max, but might
        // not be 100% on the value, so we detect within this margin.
        readonly property real pixelMargin: 0.00001
        readonly property real divideByZeroGuard: 0.00000001

        onPositionChanged: {
            console.log("position", position, 'contentSize', contentSize)
        }
        onBarSizeChanged: {
            updatePosition(ScrollBar2.Update.Bar);
        }
        onContainerSizeChanged: {
            updatePosition(ScrollBar2.Update.Container); // just to ensure min/max
        }
        onContentSizeChanged: {
            // For content resizes disable mouse input briefly to let the user
            // see the loading actually happen via the scrollbar resize/repositioning.
            console.log("contentSize changed, old:", oldContentSize, ' new:', contentSize)
            contentTimer.start();
            updatePosition(ScrollBar2.Update.Content, contentSize - oldContentSize);
            oldContentSize = contentSize;
        }
        onCursorChanged: {
            var targetOffset = position * contentSize;
            var cursorOffset = isVertical ? cursor.y : cursor.x;
            var cursorSize= isVertical ? cursor.height : cursor.width;
            var vis = targetOffset <= cursorOffset && cursorOffset + cursorSize < targetOffset + containerSize;
            if (!vis) {
                var ensureTop = cursorOffset < targetOffset;
                var containerOff = ensureTop ? cursorOffset : Math.max(0, cursorOffset + cursorSize - containerSize);
                var offset = barSize * (containerOff / contentSize);
                updatePosition(ScrollBar2.Update.Cursor, offset);
            }
            oldCursor = cursor;
        }
        function getPosition() {
            var mod = (isFinite(scrollMinLengthMod) ? scrollMinLengthMod : 1);
            var p = draggerOffset / barSize * (isFinite(scrollMinLengthMod) ? scrollMinLengthMod : 1);
            console.log("getPosition", mod, p)
            return p;
        }
        function getContentSize(otherSide) {
            if (type === ScrollBar2.TargetType.TextEdit) {
                return isVertical && !otherSide || !isVertical && otherSide ?
                        root.target.contentHeight : root.target.contentWidth;
            } else if (type === ScrollBar2.TargetType.VirtualListView) {
                return isVertical && !otherSide || !isVertical && otherSide ?
                        root.target.contentHeight : root.target.width;
            } else {
                throw new Error('unhandled target');
            }
        }
        function getContainerSize(otherSide) {
            if (containerType === ScrollBar2.ContainerType.Rectangle) {
                return isVertical && !otherSide || !isVertical && otherSide ?
                        root.container.height : root.container.width;
            } else if (type === ScrollBar2.ContainerType.VirtualListView) {
                return isVertical && !otherSide || !isVertical && otherSide ?
                        root.container.height : root.container.width;
            }  else {
                throw new Error('unhandled container');
            }
        }
        function updatePosition(type, value) {
            // Updates the view depending on the event that occured.
            // The scrollbar values will hold old values in case of
            // content resize, so these are always re-computed.
            var offset;
            var cursorOffset;
            var newScrollBarUnit = Math.min(1, barSize / contentSize);
            var newScrollBarSize = Math.max(scrollMinimumLength, Math.min(barSize, containerSize * newScrollBarUnit));
            var scrollMax = barSize - newScrollBarSize;
            if (type === ScrollBar2.Update.Content) {
                var targetPos =  ((contentSize - value) * position) / contentSize;
                offset = targetPos * barSize;
                cursorOffset = (dragger.cursor / newScrollBarSize)
            } else if (type === ScrollBar2.Update.Bar) {
                offset = (position * barSize)// * barSize;
            } else if (type === ScrollBar2.Update.Container) {
                offset = (position * barSize)// * barSize;
            } else if (type === ScrollBar2.Update.Mouse) {
                offset = value;
            } else if (type === ScrollBar2.Update.Cursor) {
                offset = value;
            } else if (type === ScrollBar2.Update.Step) {
                offset = (position + value) * barSize;
                // if we're paging, we dont want to go beyond the pageTargetOffset
                if (stepTimer.paging) {
                    if (stepTimer.stepDirection === 1 && offset > stepTimer.pagingTarget) {
                        return;
                    } else if (stepTimer.stepDirection === -1 && (position * barSize) < stepTimer.pagingTarget) {
                        return;
                    }
                }
            }
            var newOffset = Math.max(0, Math.min(offset, scrollMax));
            if (type === ScrollBar2.Update.Content) {
                dragger.cursor = cursorOffset * newScrollBarSize;
            }
            if (isVertical) {
                dragger.y = newOffset;
            } else {
                dragger.x = newOffset;
            }
            shared.maxPosition = newOffset >= (scrollMax - pixelMargin);
            shared.minPosition = newOffset <= pixelMargin;
        }
        function getPagePositionOffset(mouse) {
            var mapped = draggerContainer.mapFromItem(root, mouse.x, mouse.y);
            var mOffset = isVertical ? mapped.y : mapped.x;
            return mOffset;
        }
        function mousePressed(mouse) {
            var mapDragger = dragger.mapFromItem(root, mouse.x, mouse.y);
            dragger.dragging = dragger.contains(mapDragger);
            if (dragger.dragging) {
                dragger.cursor = isVertical ? mapDragger.y : mapDragger.x;
            }
            var mapIncrease = increaseRect.mapFromItem(root, mouse.x, mouse.y);
            var containIncrease = increaseRect.contains(mapIncrease);
            if (increaseRect.enabled && containIncrease) {
                updatePosition(ScrollBar2.Update.Step, stepSize);
                stepTimer.stepDirection = 1;
                stepTimer.paging = false;
                stepTimer.restart();
            }
            var mapDecrease = decreaseRect.mapFromItem(root, mouse.x, mouse.y);
            var containDecrease = decreaseRect.contains(mapDecrease);
            if (decreaseRect.enabled && containDecrease) {
                updatePosition(ScrollBar2.Update.Step, -stepSize);
                stepTimer.stepDirection = -1;
                stepTimer.paging = false;
                stepTimer.restart();
            }
            if (!dragger.dragging && !containIncrease && !containDecrease) {
                // the user must have hit outside the above elements, so lets page the viewport
                var stepMod = (isVertical ? mouse.y : mouse.x) - root.barSize > draggerOffset ? 1 : -1;
                var targetOffset = getPagePositionOffset(mouse);
                stepTimer.stepDirection = stepMod;
                stepTimer.paging = true;
                stepTimer.pagingTarget = targetOffset;
                updatePosition(ScrollBar2.Update.Step, stepMod * pageStepSize);
                stepTimer.restart();
            }
        }
        function mouseChanged(mouse) {
            if (dragger.dragging && !contentTimer.running) {
                console.log("mousechanged1")
                var mouseOffset = isVertical ? mouse.y - root.barSize : mouse.x - root.barSize;
                updatePosition(ScrollBar2.Update.Mouse, mouseOffset - dragger.cursor);
            } else if (stepTimer.paging)  {
                console.log("mousechanged2")
                var stepMod = (isVertical ? mouse.y : mouse.x) - root.barSize > draggerOffset ? 1 : -1;
                var targetOffset = getPagePositionOffset(mouse);
                if (stepMod === -1 && targetOffset < stepTimer.pagingTarget ||
                    stepMod === 1 && targetOffset > stepTimer.pagingTarget) {
                    stepTimer.pagingTarget = targetOffset;
                }
            }
        }
        function mouseReleased() {
            dragger.dragging = false;
            stepTimer.stop();
            repeatTimer.stop();
        }
        function targetRect() {
            var m = root.mapFromItem(root.target, 0, 0);

        }
    }
    Rectangle {
        anchors.fill: parent
        color: "#EAEAEA"
    }
    Rectangle {
        id: draggerContainer
        x: shared.isVertical ? 0 : barSize
        y: shared.isVertical ? barSize : 0
        height: shared.isVertical ? barLength - 2 * barSize : barSize
        width: shared.isVertical ? barSize : barLength - 2 * barSize
        color: "transparent"
        opacity: shared.scrollRequired ? 1 : 0
        Rectangle {
            id: dragger
            property bool dragging
            property real cursor
            x: 0
            y: 0
            width: shared.isVertical ? root.barSize : shared.scrollBarSize
            height: shared.isVertical ? shared.scrollBarSize : root.barSize
            color: "transparent"
            Rectangle {
                anchors.margins: 1
                anchors.fill: parent
                color: "silver"
            }
        }
    }
    Rectangle {
        id: decreaseRect
        height: barSize
        width: barSize
        enabled: !shared.minPosition
        opacity: enabled ? 1 : 0.4
        color: "silver"
        Text {
            anchors.centerIn: parent
            text: shared.isVertical ? '^' : '<'
        }
    }
    Rectangle {
        id: increaseRect
        x: shared.isVertical ? 0 : barLength - barSize
        y: shared.isVertical ? barLength - barSize : 0
        height: barSize
        width: barSize
        enabled: !shared.maxPosition
        opacity: enabled ? 1 : 0.4
        color: "silver"
        Text {
            anchors.centerIn: parent
            text: shared.isVertical ? 'v' : '>'
        }
    }
    TextEdit {
        anchors.right: shared.isVertical ? parent.left : undefined
        anchors.bottom: shared.isVertical ? undefined : parent.top
        text:
              "position: " + shared.position.toFixed(3) +
              "\nsize: " + shared.scrollBarUnitSize.toFixed(5) +
              "\ncontainerSize: " + shared.containerSize.toFixed(1) +
              "\nbarSize: " + shared.barSize.toFixed(1) +
              "\ncontentSize: " + shared.contentSize.toFixed(1) +
              "\ndragger: " + (shared.isVertical ? dragger.y : dragger.x).toFixed(2) +
              "\ntarget: " + (shared.isVertical ? target.y : target.x).toFixed(2)
    }
    MouseArea {
        anchors.fill: parent
        onPressed: shared.mousePressed(mouse)
        onMouseYChanged: shared.isVertical ? shared.mouseChanged(mouse) : function() {}
        onMouseXChanged: shared.isVertical ? function() {} : shared.mouseChanged(mouse)
        onReleased: shared.mouseReleased()
    }
    MouseArea {
        id: wheelArea
        enabled: enableMouseWheel
        function placeArea() {
            var m = root.container.mapToItem(root, containerX, containerY);
            x = m.x;
            y = m.y;
        }
        property real scrollerX: root.x
        property real scrollerY: root.y
        property real containerX: root.container.x
        property real containerY: root.container.y
        onScrollerXChanged: placeArea()
        onScrollerYChanged: placeArea()
        onContainerXChanged: placeArea()
        onContainerYChanged: placeArea()
        height: shared.isVertical ? shared.containerSize : shared.containerOtherSize
        width: shared.isVertical ? shared.containerOtherSize : shared.containerSize
        onWheel: {
            var stepMod = wheel.angleDelta.y < 0 ? 1 : -1;
            shared.updatePosition(ScrollBar2.Update.Step, stepMod * shared.stepSize);
        }
        onPressed: mouse.accepted = false
    }
    Component.onCompleted: {
        var t = target.toString();
        if (t.indexOf("QQuickTextEdit") === 0) {
            shared.type = ScrollBar2.TargetType.TextEdit;
            shared.cursor = Qt.binding(function() { return target.cursorRectangle; });
        } else if (t.indexOf("VirtualListView") === 0) {
            shared.type = ScrollBar2.TargetType.VirtualListView;
        } else {
            throw new Error('unknown target');
        }

        var c = container.toString();
        if (c.indexOf('QQuickRectangle') === 0) {
            shared.containerType = ScrollBar2.ContainerType.Rectangle;
        } else if (t.indexOf("VirtualListView") === 0) {
            shared.type = ScrollBar2.ContainerType.VirtualListView;
        } else {
            throw new Error('unknown container');
        }
    }
}
