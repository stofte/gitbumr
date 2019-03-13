import QtQuick 2.11
import QtQuick.Window 2.1
import "../style"

Window {
    title: "Repositories"
    modality: Qt.ApplicationModal
    id: repoWin
    width: 600
    height: 400
    RepoView { }
    color: Style.window
    Shortcut {
        sequence: StandardKey.Cancel
        onActivated: repoWin.close()
    }
}
