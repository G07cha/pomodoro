var remote = require('remote');
var app = remote.require('app');
var dialog = remote.require('dialog');
var browserWindow = remote.require('browser-window');
var fs = require('fs');
var hrt = require('human-readable-time');
var Stopwatch = require('timer-stopwatch');
var settingsWindow = createWindow();
window.$ = window.jQuery = require('jquery');

var workTimer = 1500000;
var relaxTimer = 300000;
var timeFormat = new hrt('%mm%:%ss%');

settingsWindow.on('blur', function() {
	try {
		var data = JSON.parse(fs.readFileSync(app.getDataPath() + '/config.json'));
		workTimer = data.workTimer * 60 * 1000;
		relaxTimer = data.relaxTimer * 60 * 1000;
	} catch(err) {
		console.log('Didn\'t found previous config. Using default settings');
	}
	$('.timer').circleProgress();
})

try {
	var data = JSON.parse(fs.readFileSync(app.getDataPath() + '/config.json'));
	workTimer = data.workTimer * 60 * 1000;
	relaxTimer = data.relaxTimer * 60 * 1000;
} catch(err) {
	console.log('Didn\'t found previous config. Using default settings');
}

var isRelaxTime = false;

var timer = new Stopwatch(workTimer);

$(document).ready(function() {	
	timer.on('time', function(time) {
		var progress;
		if(isRelaxTime) {
			progress = (relaxTimer - time.ms) / (relaxTimer / 100) * 0.01;
		} else {
			progress = (workTimer - time.ms) / (workTimer / 100) * 0.01;
		}
		
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
	
	$('img.settings').on('click', function() {
		if(settingsWindow) {
			settingsWindow.show();
		} else {
			settingsWindow = createWindow();
		}
		// TODO add settings window here
	});
	
	$('div.quit').on('click', function() {
		app.quit();
	});
	
	$('div.reset').on('click', function() {
		timer.reset(workTimer);
		$('.timer').circleProgress('value', 0);
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
		
		if(timer.runTimer) {
			text = timeFormat(new Date(timer.ms));
		} else {
			text = 'Click to start';
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

function createWindow() {
	var win = new browserWindow({
		width: 300,
		height: 500,
		frame: false,
		show: false
	});
	
	win.loadUrl('file://' + __dirname + '/settings.html');
	
	return win;
}