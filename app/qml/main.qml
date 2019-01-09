import QtQuick 2.9
import QtQuick.Controls 2.3
import QtQuick.Layouts 1.3
import RustCode 1.0
import "components"
import "base"
import "style"

ApplicationWindow {
    id: appRoot
    visible: true
    width: 1000
    x: 100
    y: 25
    height: 450
    color: Style.window
    property variant repoMgr;
    // Some basic handling of selected text. this isn't perfect, but at least
    // prevents the window from having multiple visible text selections as once.
    // For now, we deselect old text if some new item is touched, since theres
    // no way to currently restore the focus without destroying the existing
    // selection. The isTextElement property is defined for TextElement items.
    property variant prevActiveTextElement;
    onActiveFocusItemChanged: {
        if (activeFocusItem !== prevActiveTextElement) {
            if (prevActiveTextElement) {
                prevActiveTextElement.deselect();
            }
            if (activeFocusItem && activeFocusItem.isTextElement) {
                prevActiveTextElement = activeFocusItem;
            }
        }
    }
    Repositories {
        id: repositoriesModel
        onActiveRepositoryChanged: {
            gitView.gitPath = activeRepository;
            if (activeRepository) { // x = !!y doesnt seem to work to convert to boolean?
                noRepoView.visible = false;
            } else {
                noRepoView.visible = true;
            }
        }
    }
    Component.onCompleted: {
        // to actually cause repositoriesModel to be created on windows load,
        // the window using the model must be created on load.
        if (!repoMgr) {
            var component = Qt.createComponent("components/RepositoryManager.qml");
            repoMgr = component.createObject(appRoot);
            if (!repositoriesModel.activeRepository) {
                noRepoView.visible = true;
            }
        }
    }
    Page {
        background: Rectangle {
            color: "transparent"
        }
        anchors.fill: parent
        header: ToolBar {
            topPadding: 5
            rightPadding: 5
            bottomPadding: 5
            leftPadding: 5

            RowLayout {
                anchors.fill: parent
                ToolButton {
                    font.family: Style.fontName
                    font.pointSize: Style.fontPointSize
                    onClicked: {
                        repoMgr.show();
                    }
                    text: "Repositories"
                }
            }
        }
        GitView { id: gitView }
        Pane {
            id: noRepoView
            anchors.fill: parent
            visible: false
            TextElement {
                anchors.centerIn: parent
                text: "No repository open"
            }
        }
    }
}
