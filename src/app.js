/**
 * Golde
 * YuKun Liu <mrxzx.info@gmail.com>
 * https://github.com/mrxiaozhuox/golde/
 */

setInterval(function() {

    var value = document.getElementById("GoldeEventList").value;

    // click this button will clean all event_list
    // beacuse the value was save it, so we can clean it in `rs`.
    if (value != "{}") {
        document.getElementById("GoldeEventUsed").click();
    }

    var data = JSON.parse(value);
    var results = JSON.parse(document.getElementById("GoldeEventResultForm").value);

    for (const key in data) {

        var args = data[key];

        var argument = "";
        for (const index in args) {
            for (const type in args[index]) {
                argument += args[index][type] + ",";
            }
        }

        results[key] = eval(key + "(" + argument.substring(0, argument.length - 1) + ")");
    }

    if (Object.keys(results).length > 1) {
        console.log(results);
        document.getElementById("GoldeEventResultForm").value = JSON.stringify(results);
        document.getElementById("GoldeEventResultSubmit").click();
    }

}, 100);

document.getElementById("GoldeEventResultForm").onsubmit = function() {
    return false;
}