import QtQuick 2.0
import GitbumrComponents 1.0

// quickndirty
Item {
    Git { id: gitModel }

    function loadGit(strPath) {
        gitModel.load(strPath);
    }

    function getFilter() {
        return gitModel.revwalkFilter
    }
}
