pragma Singleton
import QtQuick 2.9

Item {
    SystemPalette { id: pallete; colorGroup: SystemPalette.Active }

    readonly property int fontPointSize: 9
    readonly property int borderSize: 4

    readonly property string dark: pallete.dark
    readonly property string window: pallete.window
}
