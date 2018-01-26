'use strict';
const {globalShortcut, ipcMain, dialog} = require('electron');
const menubar = require('menubar');
const Stopwatch = require('timer-stopwatch-dev');
const Hrt = require('human-readable-time');
const fs = require('fs');
const windowStateKeeper = require('electron-window-state');

const path = require('path');
const AutoLaunch = require('auto-launch');

let timeFormat = new Hrt('%mm%:%ss%');
let workTimer = 1500000;
let relaxTimer = 300000;
let longRelaxTimer = 900000;
let pomodoroCount = 0;
let isRelaxTime = false;
let showTimer = true;
let launchOnStartup = false;
let icon = (process.platform === 'darwin') ? '/src/img/IconTemplate.png' : '/src/img/winIcon.png';
let windowState;

let mb = menubar({
	dir: path.join(__dirname, '/src'),
	preloadWindow: true,
	tooltip: 'Pomodoro timer',
	height: 330,
	width: 340,
	icon: path.join(__dirname, icon)
});

let autolauncher = new AutoLaunch({ name: 'Pomodoro' });

getConfig();

global.timer = new Stopwatch(workTimer);
global.isRelaxTime = isRelaxTime;

process.on('uncaughtException', (err) => {
	console.log(err.stack);
	dialog.showErrorBox('Uncaught Exception: ' + err.message, err.stack || '');
	mb.app.quit();
});

mb.app.on('will-quit', () => {
	globalShortcut.unregisterAll();
	global.timer.stop();
});

mb.app.on('quit', () => {
	mb = null;
});

mb.on('ready', () => {
	windowState = windowStateKeeper()
	windowState.manage(mb.window);
})

mb.on('after-show', () => {
	if (typeof(windowState.x) == 'number' && typeof(windowState.y) == 'number') {
		mb.window.setPosition(windowState.x, windowState.y, false);
	}
})

global.timer.onTime(function(time) {
	if(showTimer) {
    if (time.ms !== workTimer
	       || time.ms !== relaxTimer
	       || time.ms !== longRelaxTimer) {
		  mb.tray.setTitle(timeFormat(new Date(time.ms)));
    }
	} else {
    mb.tray.setTitle('');
  }

	mb.window.webContents.send('update-timer', getProgress());
});

global.timer.onDone(function() {
	mb.window.webContents.send('end-timer');

	if(isRelaxTime) {
		isRelaxTime = false;
		global.timer.reset(workTimer);
	} else {
		isRelaxTime = true;
		pomodoroCount++;
		if(pomodoroCount % 4 === 0) {
			global.timer.reset(longRelaxTimer);
		} else {
			global.timer.reset(relaxTimer);
		}
	}

	global.isRelaxTime = isRelaxTime;
	global.pomodoroCount = pomodoroCount;
});

ipcMain.on('reset-timer', function(event) {
	global.timer.reset(workTimer);
	mb.tray.setTitle('');
	global.progress = getProgress();
	mb.window.webContents.send('update-timer', 0);
});

ipcMain.on('toggle-timer', function() {
	global.timer.startstop();
	mb.window.webContents.send('update-timer', getProgress());
	if(global.timer.isStopped()) mb.tray.setTitle('Paused');
});

ipcMain.on('settings-updated', function(event) {
	getConfig();

	mb.window.webContents.send('update-timer', getProgress());
});

ipcMain.on('request-config', function(event) {
	getConfig();

	event.returnValue = {
		workTimer: workTimer / 60 / 1000,
		relaxTimer: relaxTimer / 60 / 1000,
		longRelaxTimer: longRelaxTimer / 60 / 1000,
    showTimer: showTimer,
		launchOnStartup: launchOnStartup
	};
});

ipcMain.on('quit', function() {
	mb.app.quit();
});

function getConfig() {
	try {
		let dataPath = path.join(mb.app.getPath('userData'), 'config.json');
		let data = JSON.parse(fs.readFileSync(dataPath));

		workTimer = data.workTimer * 60 * 1000;
		relaxTimer = data.relaxTimer * 60 * 1000;
		longRelaxTimer = data.longRelaxTimer * 60 * 1000;
    showTimer = data.showTimer;
		launchOnStartup = data.launchOnStartup;

		(launchOnStartup ? autolauncher.enable() : autolauncher.disable())
		.then(function(err) {
			dialog.showErrorBox('Error on adding launch on startup functionality', err);
		});
	} catch(err) {
		console.log('Didn\'t find previous config. Using default settings');
	}
}

function getProgress() {
	let progress, max;

	if(isRelaxTime) {
		if(pomodoroCount % 4 === 0) {
			max = longRelaxTimer;
		} else {
			max = relaxTimer;
		}
	} else {
		max = workTimer;
	}

	progress = (max - timer.ms) / (max / 100) * 0.01;

	if(progress < 0) {
		progress = 0.01;
	}
	return progress;
}
