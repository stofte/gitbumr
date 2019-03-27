import QtQuick 2.11

Item {
    property Item target
    property Item container
    property int orientation: Qt.Vertical

    property real widgetMinSize: 15
    property real widgetMinLength: 20
    
    enum TargetType {
        TextEdit,
        VirtualListView
    }
    enum ContainerType {
        Rectangle,
        VirtualListView
    }
    enum ScrollBarState {
        Ready,
        Drag
    }
    enum Update {
        Bar,
        Content,
        Container,
        Mouse,
        Cursor,
        Step
    }
}
