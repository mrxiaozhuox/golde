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

    if (data != {}) {
        for (const key in data) {

            var args = JSON.stringify(data[key]);

            var argument = "";
            for (const index in args) {
                argument += args[index] + ",";
            }

            eval(key + "(" + argument + ")");
        }
    }
}, 250);