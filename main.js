var remote = require('remote');
var app = remote.require('app');
var dialog = remote.require('dialog');
window.$ = window.jQuery = require('jquery');
var Stopwatch = require('timer-stopwatch');

const workTimer = 1500000;
const relaxTimer = 300000;
var isRelaxTime = false;

var timer = new Stopwatch(workTimer);

$(document).ready(function() {	
	timer.on('time', function(time) {
		var progress = (workTimer - time.ms) / (workTimer / 100) * 0.01;
		$('.timer').circleProgress('value', progress);
	});

	timer.on('done', function() {
		$('.timer').circleProgress('value', 1);
		dialog.showMessageBox({
			type: 'info',
			title: 'Pomodoro',
			message: (isRelaxTime) ? 'Back to work' : 'Timer ended it\'s time to relax',
			buttons: ['OK'],
			noLink: true
		}, function() {
			if(isRelaxTime) {
				timer.reset(workTimer);
				$('.timer').circleProgress({fill: { gradient: ["blue", "skyblue"]}});
				isRelaxTime = false;
			} else {
				timer.reset(relaxTimer);
				$('.timer').circleProgress({fill: { gradient: ["orange", "yellow"]}});
				isRelaxTime = true;
			}
			timer.start();
		});
	});
	
	$('div.timer').on('click', function() {
		if(timer.runTimer === false) {
			isRelaxTime = false;
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
