pragma Singleton
import QtQuick 2.9

Item {

    FontLoader { id: mainFont; name: "Segoe UI" }
    FontLoader { id: fixedWidthFont; name: "Consolas" }
    SystemPalette { id: palette; colorGroup: SystemPalette.Active }

    readonly property int fontPointSize: 9
    readonly property int borderSize: 4

    readonly property string dark: palette.dark
    readonly property string mid: palette.mid
    readonly property string light: palette.light
    readonly property string window: palette.window

    readonly property string fontName: mainFont.name

    readonly property string fontNameFixedWidth: fixedWidthFont.name
}
