import QtQuick 2.9
import QtQuick.Layouts 1.3
import QtQuick.Shapes 1.11
import "../base"
import "../style"

Item {
    id: root
    property int graphHeight: 100
    property int graphWidth: 150
    property int lanes
    Layout.fillHeight: true
    Layout.preferredWidth: graphWidth
    property bool isSelected: false
    property variant graphData
    property bool isMergeNode
    property bool debug: false
    property bool requiresUpdates: false
    function requestPaint() {
        canvas.requestPaint();
    }
    JsonListModel {
        id: graphModel
        debug: root.debug
        jsonArray: root.graphData
        onModelUpdated: {
            if (requiresUpdates) {
                canvas.requestPaint();
            }
        }
    }
    Canvas {
        id: canvas
        Path {
            id: linePath
            startX: 0; startY: 0
            PathLine { x: 0; y: 20 }
        }
        Path {
            id: leafPath
            startX: 0; startY: 20
            PathLine { x: 0; y: 10 }
        }
        Path {
            id: rootPath
            startX: 0; startY: 0
            PathLine { x: 0; y: 10 }
        }
        Path {
            id: lineDownPath
            startX: 10; startY: 10
            PathLine { x: 10; y: 18 }
            PathLine { x: 10; y: 20 }
        }
        Path {
            id: lineUpPath
            startX: 10; startY: 10
            PathLine { x: 10; y: 2 }
            PathLine { x: 10; y: 0 }
        }

        Path {
            id: lineShiftPath
            startX: 10; startY: 0
            PathLine { x: 10; y: 2 }
            PathLine { x: 10; y: 18 }
            PathLine { x: 10; y: 20 }
        }

        anchors.fill: parent
        anchors.rightMargin: 10
        function paintPaths(ctx, laneFlags, offW, rowOffW, shiftOffW) {
            if (laneFlags.isVisible) {
                // flags are merged, so we might both have a line through,
                // and it might also have been merged into the commit node
                linePath.startX = offW;
                linePath.pathElements[0].x = offW;
                ctx.beginPath();
                ctx.path = linePath;
                ctx.stroke();
            }
            if (laneFlags.isMerge) {
                lineDownPath.startX = rowOffW;
                lineDownPath.pathElements[0].x = offW;
                lineDownPath.pathElements[1].x = offW;
                ctx.beginPath();
                ctx.path = lineDownPath;
                ctx.stroke();
            }
            if (laneFlags.isBranch) {
                lineUpPath.startX = rowOffW;
                lineUpPath.pathElements[0].x = offW;
                lineUpPath.pathElements[1].x = offW;
                ctx.beginPath();
                ctx.path = lineUpPath;
                ctx.stroke();
            }
            if (laneFlags.isLeaf) {
                leafPath.startX = offW;
                leafPath.pathElements[0].x = offW;
                ctx.beginPath();
                ctx.path = leafPath;
                ctx.stroke();
            }
            if (laneFlags.isRoot) {
                rootPath.startX = offW;
                rootPath.pathElements[0].x = offW;
                ctx.beginPath();
                ctx.path = rootPath;
                ctx.stroke();
            }
            if (laneFlags.isShift) {
                lineShiftPath.startX = offW;
                lineShiftPath.pathElements[0].x = offW;
                lineShiftPath.pathElements[1].x = shiftOffW;
                lineShiftPath.pathElements[2].x = shiftOffW;
                ctx.beginPath();
                ctx.path = lineShiftPath;
                ctx.stroke();
            }
        }
        function updateCanvas() {
            if (root.debug) {
                console.log("painting", isSelected, available)
            }
            if (available) {
                var ctx = getContext("2d")
                ctx.clearRect(0, 0, width, height);
                var halfG = graphHeight / 2;
                ctx.lineWidth = 1;
                ctx.strokeStyle = isSelected ? 'white' : Qt.rgba(0.5,0.5,0.5,1);
                ctx.fillStyle = root.isMergeNode ? Qt.rgba(0.5,0.5,0.5,1) : Qt.rgba(0.52549,0.7490,0.81960,1);
                for(var i = 0; i < graphModel.model.count; i++) {
                    var elm = graphModel.model.get(i);
                    var offW = halfG + i * graphHeight;
                    var rowOffW = halfG + elm.rowCommitIndex * graphHeight;
                    var shiftOffW = offW + elm.rowShiftOffset * graphHeight;
                    paintPaths(ctx, elm, offW, rowOffW, shiftOffW);
                }
                var cOffW = halfG + graphModel.commitIndex * graphHeight;
                ctx.beginPath();
                ctx.arc(cOffW, halfG, 3.5, 0, 2 * Math.PI);
                ctx.fill();
                ctx.stroke();
            }
        }
        onPaint: updateCanvas()
    }
}
