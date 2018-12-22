import QtQuick 2.0
import RustCode 1.0

Item {
    ListView {
        clip: true
        anchors.fill: parent
        model: gitModel.tree
        delegate: Component {
            Item {
                height: textRef.contentHeight
                Text {
                    id: textRef
                    width: parent.width
                    wrapMode: Text.NoWrap
                    text: patch
                }
            }
        }
    }
}
