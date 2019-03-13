import QtQuick 2.0

Item {
    property ListModel model : ListModel { id: jsonModel }
    property int byteSize: 8
    signal modelUpdated();
    property variant jsonArray
    onJsonArrayChanged: updateJSONModel()

    function updateJSONModel() {
        jsonModel.clear();
        if (!jsonArray) return;
        var ia;
        if (byteSize === 8) {
            ia = new Uint8Array(jsonArray);
        } else if (byteSize === 32) {
            ia = new Uint32Array(jsonArray);
        } else {
            throw new Error("unsupported byteSize");
        }
        for (var i = 0; i < ia.length; i++) {
            jsonModel.append({ value: ia[i] });
        }
        modelUpdated();
    }
}
