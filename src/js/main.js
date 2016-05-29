'use strict';

const {remote, ipcRenderer} = require('electron');
const {dialog, globalShortcut, BrowserWindow} = remote;
const hrt = require('human-readable-time');
window.$ = window.jQuery = require('jquery');

const timeFormat = new hrt('%mm%:%ss%');
var settingsWindow = createWindow();

globalShortcut.register('ctrl+alt+s', function() {
	ipcRenderer.send('start-timer');
});

ipcRenderer.on('update-timer', function(event, value) {
	if(remote.getGlobal('timer').state) {
		if(remote.getGlobal('isRelaxTime')) {
			$('.timer').circleProgress({fill: { gradient: ["orange", "yellow"]}});
		} else {
			$('.timer').circleProgress({fill: { gradient: ["blue", "skyblue"]}});
		}
	} else {
		$('.timer').circleProgress({fill: { gradient: ["gray", "lightgray"]}});
	}
	$('.timer').circleProgress('value', value);
});

ipcRenderer.on('end-timer', function() {
	const isRelaxTime = remote.getGlobal('isRelaxTime');
	$('.timer').circleProgress('value', 1);

	dialog.showMessageBox({
		type: 'info',
		title: 'Pomodoro',
		message: isRelaxTime ? 'Timer ended it\'s time to relax' : 'Back to work',
		buttons: ['OK'],
		noLink: true
	}, function() {
		if(isRelaxTime) {
			$('.timer').circleProgress({fill: { gradient: ["blue", "skyblue"]}});
		} else {
			$('#counter').text(remote.getGlobal('pomodoroCount'));
			$('.timer').circleProgress({fill: { gradient: ["orange", "yellow"]}});
		}

		ipcRenderer.send('start-timer');
	});
});

$(document).ready(function() {
	$('div.timer').on('click', function() {
		ipcRenderer.send('start-timer');
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
		$('.timer').circleProgress({fill: { gradient: ["blue", "skyblue"]}});
		ipcRenderer.send('reset-timer');
	});

	$('.timer').circleProgress({
		value: 0,
		size: 250,
		lineCap: 'round',
		fill: {
			gradient: ["blue", "skyblue"]
		}
	}).on('circle-animation-progress', function() {
		let timer = remote.getGlobal('timer');
		let text = timer.state ?
				timeFormat(new Date(timer.ms)) : 'Click to start'

		$(this).find('strong').text(text);
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
