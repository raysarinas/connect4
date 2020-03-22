var canvas, context;
$(document).ready(function () {
    canvas = document.getElementById("gameboard");
    context = canvas.getContext("2d");

    context.fillStyle = "#00bfff";
    context.fillRect(0, 0, 640, 480);
    // Circle
    context.fillStyle = "white";
    for (var i = 0; i < 6; i++) {
        for (var j = 0; j < 7; j++) {
        context.beginPath();
        context.arc(50 + j * 90, 40 + i * 80, 28, 0, 2*Math.PI, false);
        context.fill();
        }
    }

    $('#sideNavigation a').click(function () {
        console.log('click');
        $('#content').load(this.getAttribute('href').split('/')[1] + ".html");
    });
});

