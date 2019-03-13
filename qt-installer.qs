// installer script for the Qt installer. Launch with 
// ./qt-unified-linux-x64-online.run --platform minimal --script qt-installer.qs
// --platform minimal is for running headless (without x)
// http://download.qt.io/official_releases/online_installers/qt-unified-linux-x64-online.run
// http://mirrors.dotsrc.org/qtproject/archive/qt/5.12/5.12.1/qt-opensource-linux-x64-5.12.1.run
// https://download.qt.io/official_releases/qt/5.12/5.12.1/
// https://stackoverflow.com/questions/25105269/silent-install-qt-run-installer-on-ubuntu-server/46607941#46607941
// also https://wiki.qt.io/Install_Qt_5_on_Ubuntu

function Controller() {
    console.log("qs:Controller");
    installer.autoRejectMessageBoxes();
    installer.installationFinished.connect(function() {
        gui.clickButton(buttons.NextButton);
    })
}

Controller.prototype.WelcomePageCallback = function() {
    console.log("qs:WelcomePageCallback");
    // click delay here because the next button is initially disabled for ~1 second
    gui.clickButton(buttons.NextButton, 3000);
}

Controller.prototype.CredentialsPageCallback = function() {
    console.log("qs:CredentialsPageCallback");
    gui.clickButton(buttons.NextButton);
}

Controller.prototype.IntroductionPageCallback = function() {
    console.log("qs:IntroductionPageCallback");
    gui.clickButton(buttons.NextButton);
}

Controller.prototype.TargetDirectoryPageCallback = function()
{
    console.log("qs:TargetDirectoryPageCallback");
    gui.currentPageWidget().TargetDirectoryLineEdit.setText(installer.value("HomeDir") + "/Qt");
    gui.clickButton(buttons.NextButton);
}

Controller.prototype.ComponentSelectionPageCallback = function() {
    console.log("qs:ComponentSelectionPageCallback");
    var widget = gui.currentPageWidget();
    widget.deselectAll();
    // strings for package names comes from (pick version first)
    // https://github.com/qtproject/qtsdk/tree/master/packaging-tools/configurations/pkg_templates
    widget.selectComponent("qt.qt5.5110.gcc_64");
    gui.clickButton(buttons.NextButton);
}

Controller.prototype.LicenseAgreementPageCallback = function() {
    console.log("qs:LicenseAgreementPageCallback");
    gui.currentPageWidget().AcceptLicenseRadioButton.setChecked(true);
    gui.clickButton(buttons.NextButton);
}

Controller.prototype.StartMenuDirectoryPageCallback = function() {
    console.log("qs:StartMenuDirectoryPageCallback");
    gui.clickButton(buttons.NextButton);
}

Controller.prototype.ReadyForInstallationPageCallback = function()
{
    console.log("qs:ReadyForInstallationPageCallback");
    gui.clickButton(buttons.NextButton);
}

Controller.prototype.FinishedPageCallback = function() {
    console.log("qs:FinishedPageCallback");
    var checkBoxForm = gui.currentPageWidget().LaunchQtCreatorCheckBoxForm;
    if (checkBoxForm && checkBoxForm.launchQtCreatorCheckBox) {
        checkBoxForm.launchQtCreatorCheckBox.checked = false;
    }
    gui.clickButton(buttons.FinishButton);
}