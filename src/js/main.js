'use strict';

const {remote, ipcRenderer} = require('electron');
const {dialog, globalShortcut, BrowserWindow} = remote;
const path = require('path');

const HRT = require('human-readable-time');
const timeFormat = new HRT('%mm%:%ss%');
const windowStateKeeper = require('electron-window-state');
const retinajs = require('retinajs');

window.$ = require('jquery');
window.jQuery = window.$
require('../bower_components/jquery-circle-progress/dist/circle-progress.js')()

let settingsWindow;
let circleTimer;

globalShortcut.register('ctrl+alt+s', function() {
	ipcRenderer.send('toggle-timer');
});

ipcRenderer.on('update-timer', function(event, value) {
	if(remote.getGlobal('timer').isRunning()) {
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
		if(settingsWindow) {
			settingsWindow.close();
		}
		ipcRenderer.send('quit');
	});

	$('#resetBtn').on('click', function() {
		circleTimer.reset();
		ipcRenderer.send('reset-timer');
	});


	circleTimer = new CircleController('.timer', {
		onAnimation: function() {
			let timer = remote.getGlobal('timer');
			let text = timer.isRunning() ? timeFormat(new Date(timer.ms)) : 'Click to start'

			$(this).find('strong')
						.text(text);
		}
	});
});

// For creating settings window
function createWindow() {
	let windowState = windowStateKeeper();

	let win = new BrowserWindow({
		width: 300,
		height: 500,
		frame: false,
		x: windowState.x,
		y: windowState.y
	});

	windowState.manage(win);

	win.loadURL(path.join('file://', __dirname, '/settings.html'));

	return win;
}
