import QtQuick 2.11
import "../style"

Item {
    id: root
    width: parent.width
    height: parent.height
    property string color: "red"
    property bool enabled: false
    property bool aboveContent: true
    property string debugText
    property real debugPointSize: 20
    Rectangle {
        visible: root.enabled
        property string debugColor: parent.color
        property real lineOpacity: 0.15
        property real textOpacity: 0.5
        property real userOpacity: 0.3
        property real textPointSize: 7
        width: parent.width
        height: parent.height
        color: 'transparent'
        z: aboveContent ? 1 : 0
        Rectangle {
            anchors.top: parent.top
            width: parent.width
            height: 1
            color: parent.debugColor
            opacity: parent.lineOpacity
        }
        Rectangle {
            anchors.right: parent.right
            width: 1
            height: parent.height
            color: parent.debugColor
            opacity: parent.lineOpacity
        }
        Rectangle {
            anchors.bottom: parent.bottom
            width: parent.width
            height: 1
            color: parent.debugColor
            opacity: parent.lineOpacity
        }
        Rectangle {
            anchors.left: parent.left
            width: 1
            height: parent.width
            color: parent.debugColor
            opacity: parent.lineOpacity
        }
        function getRightAngle() {
            return 90 - (180 / Math.PI) *  Math.atan((width - 1) / (height - 1))
        }
        Rectangle {
            x: 0
            y: parent.height - 1
            height: 1
            width: LibHelper.hypot(parent.height, parent.width)
            color: parent.debugColor
            opacity: parent.lineOpacity
            transformOrigin: Item.BottomLeft
            rotation: -1 * parent.getRightAngle()
        }
        Rectangle {
            x: 0
            y: 0
            height: 1
            width: LibHelper.hypot(parent.height, parent.width)
            color: parent.debugColor
            opacity: parent.lineOpacity
            transformOrigin: Item.TopLeft
            rotation: parent.getRightAngle()
        }
        TextElement {
            x: parent.width - contentWidth - contentHeight - 2
            y: 2
            text: parent.width
            fixedWidthFont: true
            font.pointSize: parent.textPointSize
            opacity: parent.textOpacity
        }
        TextElement {
            x: parent.width - 2
            y: 4
            text: parent.height
            fixedWidthFont: true
            font.pointSize: parent.textPointSize
            opacity: parent.textOpacity
            transform: Rotation {
                angle: 90
                origin.x: 0
                origin.y: 0
            }
        }
        Text {
            anchors.centerIn: parent
            color: "transparent"
            text: root.debugText
            font.family: Style.fontNameFixedWidth
            font.pointSize: root.debugPointSize
            font.weight: Font.Bold
            style: Text.Outline
            opacity: parent.textOpacity
            styleColor: parent.debugColor // Style.window
            horizontalAlignment: Text.AlignHCenter
        }
    }
}
