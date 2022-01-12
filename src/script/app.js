/**
 * Golde
 * YuKun Liu <mrxzx.info@gmail.com>
 * https://github.com/mrxiaozhuox/golde/
 */

setInterval(function() {

    var queue = document.getElementById("GoldeEventQueue").value;
    var queue = JSON.parse(queue);

    var new_queue = {};

    if (Object.keys(queue).length) {
        for (const key in queue) {
            var data = queue[key];
            result = eval(data[0]);
            new_queue[key] = data[0];
        }
    }

}, 1000);

document.getElementById("GoldeEventQueue").onsubmit = function() {
    return false;
}