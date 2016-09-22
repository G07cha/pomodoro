'use strict';

const {remote, ipcRenderer} = require('electron');
const {dialog, globalShortcut, BrowserWindow} = remote;

const hrt = require('human-readable-time');
const timeFormat = new hrt('%mm%:%ss%');
const retina = require('retinajs');

window.$ = window.jQuery = require('jquery');

var settingsWindow = createWindow();
var circleTimer;

globalShortcut.register('ctrl+alt+s', function() {
	ipcRenderer.send('toggle-timer');
});

ipcRenderer.on('update-timer', function(event, value) {
	if(remote.getGlobal('timer').runTimer) {
		if(remote.getGlobal('isRelaxTime')) {
			circleTimer.mode = 'relax';
		} else {
			circleTimer.mode = 'work';
		}
	} else {
		circleTimer.pause();
	}

	circleTimer.value = value;
});

ipcRenderer.on('end-timer', function() {
	const isRelaxTime = remote.getGlobal('isRelaxTime');
	circleTimer.value = 1;

	dialog.showMessageBox({
		type: 'info',
		title: 'Pomodoro',
		message: isRelaxTime ? 'Timer ended it\'s time to relax' : 'Back to work',
		buttons: ['OK'],
		noLink: true
	}, function() {
		if(isRelaxTime) {
			circleTimer.mode = 'work';
		} else {
			$('#counter').text(remote.getGlobal('pomodoroCount'));
			circleTimer.mode = 'relax';
		}

		ipcRenderer.send('toggle-timer');
	});
});

$(document).ready(function() {
	retinajs();
	$('div.timer').on('click', function() {
		ipcRenderer.send('toggle-timer');
	});

	$('#settingsBtn').on('click', function() {
		if(settingsWindow) {
			settingsWindow.show();
		} else {
			settingsWindow = createWindow();
		}
	});

	$('#quitBtn').on('click', function() {
		ipcRenderer.send('quit');
	});

	$('#resetBtn').on('click', function() {
		circleTimer.reset();
		ipcRenderer.send('reset-timer');
	});


	circleTimer = new CircleController('.timer', {
		onAnimation: function() {
			let timer = remote.getGlobal('timer');
			let text = timer.runTimer ?
					timeFormat(new Date(timer.ms)) : 'Click to start'

			$(this).find('strong').text(text);
		}
	});
});

// For creating settings window
function createWindow() {
	var win = new BrowserWindow({
		width: 300,
		height: 500,
		frame: false,
		show: false
	});
	win.loadURL('file://' + __dirname + '/settings.html');

	return win;
}
