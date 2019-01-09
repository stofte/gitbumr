pragma Singleton
import QtQuick 2.11

Item {
    // note the user role + offset, which is defined in Bindings.cpp.
    // its the json properties sorted alphabetically (and not as defined
    // in binding.json!). This is tricky because even just renaming 
    // properties shuffles their indexes.
    // These values should be used by the model.data(modelIdx, roleValue)
    // method, where role value is the appropiate column value from below.
    property int diffs_filenameNew: 0x100
    property int hunks_hunk: 0x100
    property int hunks_lines: 0x102
    property int hunks_linesOrigin: 0x108
    function modelValue(model, idx, role) {
        return model.data(model.index(idx, 0), role);
    }
    function hypot(a, b) {
        return Math.sqrt(a * a + b * b);
    }
}
