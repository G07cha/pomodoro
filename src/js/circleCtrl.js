'use strict';
const modes = ['relax', 'work', 'pause'];

class CircleController {
  constructor(selector, options = {}) {
    this.element = $(selector);
    this.workGradient = options.workGradient || ['blue', 'skyblue'];
    this.relaxGradient = options.relaxGradient || ['orange', 'yellow'];
    this.pauseGradient = options.pauseGradient || ['gray', 'lightgray'];

    this.element.circleProgress({
      value: options.value || 0,
      size: options.size || 250,
      lineCap: options.lineCap || 'round',
      fill: { gradient: this.workGradient }
    }).on('circle-animation-progress', options.onAnimation);

    overrideDefaults();
  }

  reset(mode = 'work') {
    this.mode = mode;
  }

  pause() {
    this.mode = 'pause';
  }

  set mode(newMode) {
    if(modes.indexOf(newMode) > -1) {
      this.element.circleProgress({fill: {gradient: this[newMode + 'Gradient']}});
    } else {
      throw new Error('Specified mode is incorrect, expecting '
      + modes + ' but got ' + newMode);
    }
  }

  set value(value) {
    if(value >= 0 && value <= 1) {
      this.element.circleProgress('value', value);
    } else {
      throw new Error('Incorrect value passed, value should be between 0 and 1');
    }
  }
}

global.CircleController = CircleController

function overrideDefaults() {
  $.circleProgress.defaults.setValue = function(newValue) {
    if (this.animation) {
      let canvas = $(this.canvas),
               q = canvas.queue();

      if (q[0] === 'inprogress') {
        canvas.stop(true, true);
      }

      this.animationStartValue = this.lastFrameValue;
    }

    this.value = newValue;
    this.draw();
  };
}
