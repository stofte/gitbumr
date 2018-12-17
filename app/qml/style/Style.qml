pragma Singleton
import QtQuick 2.9

Item {

    FontLoader { id: mainFont; name: "Roboto" }
    FontLoader { id: fixedWidthFont; name: "Consolas" }
    SystemPalette { id: palette; colorGroup: SystemPalette.Active }

    readonly property int fontPointSize: 10
    readonly property int borderSize: 4

    readonly property string dark: palette.dark
    readonly property string mid: palette.mid
    readonly property string light: palette.light
    readonly property string window: "#FAFAFA" // palette.window

    readonly property string fontName: mainFont.name

    readonly property string fontNameFixedWidth: fixedWidthFont.name
}
