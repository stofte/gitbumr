import QtQuick 2.0
import "../base"
import "../style"

ListView {
    model: gitModel.hunks
    delegate: Component {
        Item {
            height: diffRef.contentHeight + 10
            TextEdit {
                id: diffRef
                width: parent.width
                font.family: Style.fontNameFixedWidth
                //textFormat: Text.StyledText
                readOnly: true
                selectByMouse: true
                text: hunk
            }
        }
    }
}
