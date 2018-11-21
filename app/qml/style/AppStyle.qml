pragma Singleton
import QtQuick 2.9

Item {
    SystemPalette { id: myPalette; colorGroup: SystemPalette.Active }

    readonly property int fontPointSize: 9
    readonly property int borderSize: 4

    readonly property string dark: myPalette.dark
    readonly property string window: myPalette.window
}
