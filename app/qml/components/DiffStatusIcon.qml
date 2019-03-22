import QtQuick 2.9
import GitbumrComponents 1.0
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import "../base"
import "../style"

Item {
    property string statusValue: "Unknown"
    property int iconSize: 18
    function mapGitStatusToLetterOffset(status) {
        switch (status) {
            case "Modified":    return 3;
            case "Renamed":
            case "Deleted":
            case "Added":       return 4;
            case "":            return 0;
            default:
                throw new Error('mapGitStatusToLetterOffset unhandled git status: ' + status);
        }
    }
    function mapGitStatusToLetter(status) {
        return status ? status.substring(0, 1) : "";
    }

    Rectangle {
        height: iconSize
        width: iconSize
        x: 3
        y: 2.5
        color: Style.gitStatusColor(statusValue)
        visible: statusValue
        TextElement {
            x: mapGitStatusToLetterOffset(statusValue)
            y: 1
            color: "white"
            font.weight: Font.Bold
            // the offsets above are for using native render
            renderType: Text.NativeRendering
            text: mapGitStatusToLetter(statusValue)
        }
    }
}
