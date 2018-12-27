import QtQuick 2.0
import RustCode 1.0
import "../base"
import "../style"

Item {
    ListView {
        clip: true
        anchors.fill: parent
        model: gitModel.diffs
        delegate: Component {
            Item {
                height: textRef.contentHeight
                TextItem {
                    id: textRef
                    font.family: Style.fontNameFixedWidth
                    width: parent.width
                    wrapMode: Text.NoWrap
                    text: patch
                }
            }
        }
    }
}
