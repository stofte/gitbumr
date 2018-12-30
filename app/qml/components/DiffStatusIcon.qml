import QtQuick 2.9
import RustCode 1.0
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import "../base"
import "../style"

Item {
    property string statusValue: "Unknown"
    property int iconSize: 18
    function mapGitStatusToColor(status) {
        switch (status) {
            case "Modified":    return "#F6C342";
            case "Added":       return "#14892C";
            case "Deleted":     return "#D04437";
            case "Renamed":     return "#AC707A";
            case "":            return "";
            default:
                throw new Error('mapGitStatusToColor hnhandled git status: ' + status);
        }
    }
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
        color: mapGitStatusToColor(statusValue)

        TextItem {
            x: mapGitStatusToLetterOffset(statusValue)
            y: 1
            color: "white"
            font.weight: Font.Bold
            text: mapGitStatusToLetter(statusValue)
        }
    }
}
