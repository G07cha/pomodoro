var remote = require('remote');
var app = remote.require('app');
var dialog = remote.require('dialog');
var browserWindow = remote.require('browser-window');
var globalShortcut = remote.require('global-shortcut');
var ipc = require('ipc');

var fs = require('fs');
var hrt = require('human-readable-time');
var Stopwatch = require('timer-stopwatch');
var settingsWindow = createWindow();
window.$ = window.jQuery = require('jquery');

var timeFormat = new hrt('%mm%:%ss%');

settingsWindow.on('blur', function() {
	ipc.send('settings-updated');
	$('.timer').circleProgress();
});


globalShortcut.register('ctrl+alt+s', function() {
	ipc.send('start-timer');
});

ipc.on('update-timer', function(event) {
	var progress = ipc.sendSync('request-update');
	$('.timer').circleProgress('value', progress);
});

ipc.on('end-timer', function(event, arg) {
	$('.timer').circleProgress('value', 1);
	
	dialog.showMessageBox({
		type: 'info',
		title: 'Pomodoro',
		message: (arg.isRelaxTime) ? 'Back to work' : 'Timer ended it\'s time to relax',
		buttons: ['OK'],
		noLink: true
	}, function() {
		if(arg.isRelaxTime) {
			$('.timer').circleProgress({fill: { gradient: ["blue", "skyblue"]}});
		} else {
			$('#counter').text(arg.pomodoroCount);
			$('.timer').circleProgress({fill: { gradient: ["orange", "yellow"]}});
		}
		
		event.returnValue = true;
	});
});

$(document).ready(function() {
	$('div.timer').on('click', function() {
		ipc.send('start-timer');
	});
	
	$('img.settings').on('click', function() {
		if(settingsWindow) {
			settingsWindow.show();
		} else {
			settingsWindow = createWindow();
		}
	});
	
	$('div.quit').on('click', function() {
		app.quit();
	});
	
	$('div.reset').on('click', function() {
		ipc.send('reset-timer');
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
		
		var timer = remote.getGlobal('timer');
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

// For creating settings window
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
