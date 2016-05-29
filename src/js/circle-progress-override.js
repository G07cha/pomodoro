$(document).ready(function() {
  $.circleProgress.defaults.setValue = function(newValue) {
    if (this.animation) {
      var canvas = $(this.canvas),
               q = canvas.queue();

      if (q[0] == 'inprogress') {
        canvas.stop(true, true);
      }

      this.animationStartValue = this.lastFrameValue;
    }

    this.value = newValue;
    this.draw();
  };
});
