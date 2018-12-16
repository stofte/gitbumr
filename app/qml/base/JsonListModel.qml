import QtQuick 2.9

Item {
    property ListModel model : ListModel { id: jsonModel }
    property int commitIndex: 0
    property alias count: jsonModel.count

    property variant jsonArray

    onJsonArrayChanged: updateJSONModel()

    function updateJSONModel() {
        jsonModel.clear();
        if (!jsonArray) return;
        var ia = new Uint8Array(jsonArray);
        commitIndex = ia[0];
        var closedLanes = 0;
        var ival = 0;
        for (var i = 1; i < ia.length; i++) {
            ival = ia[i];
            var elm = {
                rowCommitIndex: commitIndex,
                rowShiftOffset: closedLanes,
                isVisible: ival & 0x1,
                isCommit: ival & 0x2,
                isLeaf: ival & 0x4,
                isRoot: ival & 0x8,
                isMerge: ival & 0x10,
                isBranch: ival & 0x20,
                isShift: ival & 0x40,
            };
            // for each lane that was closed, before we came to this column,
            // matches the number of lanes an eventual shift must use.
            if (elm.isBranch) {
                closedLanes -= 1
            }
            jsonModel.append(elm);
        }
    }
}
