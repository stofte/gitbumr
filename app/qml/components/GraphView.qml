import QtQuick 2.9
import QtQuick.Layouts 1.3
import QtQuick.Shapes 1.11
import "../base"
import "../style"

Item {
    property int graphHeight: 100
    property int lanes
    Layout.fillHeight: true
    Layout.preferredWidth: 160

    JsonListModel {
        id: graphModel
        jsonArray: graph
    }

    Canvas {
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

        anchors.fill: parent
        contextType: "2d"

        onPaint: {
            var halfG = graphHeight / 2;
            context.lineWidth = 1.5;
            context.strokeStyle = Qt.rgba(0,0,1);
            context.fillStyle = Qt.rgba(1,1,1);
            for(var i = 0; i < graphModel.model.count; i++) {
                context.beginPath();
                var elm = graphModel.model.get(i);
                var offW = halfG + i * graphHeight;
                var rowOffW = halfG + elm.rowCommitIndex * graphHeight;
                if (elm.isVisible) {
                    // flags are merged, so we might both have a line through,
                    // and it might also have been merged into the commit node
                    if (elm.isMerge) {
                        lineDownPath.startX = rowOffW;
                        lineDownPath.pathElements[0].x = offW;
                        lineDownPath.pathElements[1].x = offW;
                        context.path = lineDownPath;
                        context.stroke();
                    }
                    linePath.startX = offW;
                    linePath.pathElements[0].x = offW;
                    context.path = linePath;
                } else if (elm.isMerge) {
                    lineDownPath.startX = rowOffW;
                    lineDownPath.pathElements[0].x = offW;
                    lineDownPath.pathElements[1].x = offW;
                    context.path = lineDownPath;
                } else if (elm.isBranch) {
                    lineUpPath.startX = rowOffW;
                    lineUpPath.pathElements[0].x = offW;
                    lineUpPath.pathElements[1].x = offW;
                    context.path = lineUpPath;
                } else if (elm.isLeaf) {
                    leafPath.startX = offW;
                    leafPath.pathElements[0].x = offW;
                    context.path = leafPath;
                } else if (elm.isRoot) {
                    rootPath.startX = offW;
                    rootPath.pathElements[0].x = offW;
                    context.path = rootPath;
                }
                context.stroke();
            }
            context.beginPath();
            var cOffW = halfG + graphModel.commitIndex * graphHeight;
            context.lineWidth = 2;
            context.arc(cOffW, halfG, 4, 0, 2 * Math.PI);
            context.stroke();
            context.beginPath();
            context.arc(cOffW, halfG, 3, 0, 2 * Math.PI);
            context.fill();
        }
    }
}
