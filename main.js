var remote = require('remote');
var app = remote.require('app');
window.$ = window.jQuery = require('jquery');
var Stopwatch = require('timer-stopwatch');

const workTimer = 1500000;

var timer = new Stopwatch(workTimer);

$(document).ready(function() {	
	timer.on('time', function(time) {
		var progress = (workTimer - time.ms) / (workTimer / 100) * 0.01;
		$('.timer').circleProgress('value', progress);
	});

	timer.on('done', function() {
		$('.timer').circleProgress('value', 1);
		console.log('Timer is complete');
	});
	
	$('div.timer').on('click', function() {
		if(timer.runTimer === false) {
			timer.start();
		} else {
			timer.stop();
		}
	});
	
	$('div.quit').on('click', function() {
		app.quit();
	});
	
	$('div.reset').on('click', function() {
		timer.reset();
	});
	
	$('.timer').circleProgress({
		value: 0,
		size: 250,
		lineCap: 'round',
		fill: {
			gradient: ["blue", "skyblue"]
		}
	}).on('circle-animation-progress', function(event, progress, stepValue) {
		var text;
		
		if(parseInt(timer.ms) === workTimer) {
			text = 'Click to start';
		} else {
			text = (parseInt(timer.ms / 1000 / 60)) + ':' + (parseInt(timer.ms / 1000) % 60);
		}
		
		$(this).find('strong').text(text);
	});
	
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
