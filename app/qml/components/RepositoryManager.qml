import QtQuick 2.11
import QtQuick.Window 2.1

Window {
    title: "Repositories"
    modality: Qt.ApplicationModal
    id: repoWin
    width: 600
    height: 400
    RepoView { }
    Shortcut {
        sequence: StandardKey.Cancel
        onActivated: repoWin.close()
    }
}
