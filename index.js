'use strict';
const menubar = require('menubar');
const Menu = require('menu');
const globalShortcut = require('global-shortcut');
const ipc = require('ipc');
const dialog = require('dialog');
const Stopwatch = require('timer-stopwatch');
const Hrt = require('human-readable-time');
const fs = require('fs');
const path = require('path');
const AutoLaunch = require('auto-launch');

// report crashes to the Electron project
require('crash-reporter').start();

let timeFormat = new Hrt('%mm%:%ss%');
let workTimer = 1500000;
let relaxTimer = 300000;
let longRelaxTimer = 900000;
let pomodoroCount = 0;
let isRelaxTime = true;
let launchOnStartup = false;
let sender;

let mb = menubar({
	'preloadWindow': true
});

let options = {};
options.name = 'Pomodoro';

if(process.platform === 'darwin') {
	options.path = path.join(mb.app.getAppPath(), 'Pomodoro.app');
} else if(process.platform === 'win32') {
	options.path = path.join(mb.app.getAppPath(), 'Pomodoro.exe');
}

let autolauncher = new AutoLaunch(options);

getConfig();

global.timer = new Stopwatch(workTimer);
global.isRelaxTime = isRelaxTime;

process.on('uncaughtException', (err) => {
	dialog.showErrorBox('Uncaught Exception: ' + err.message, err.stack || '');
	mb.app.quit();
});

mb.app.on('will-quit', () => {
	globalShortcut.unregisterAll();
});

mb.app.on('quit', () => {
	mb = null;
});

global.timer.on('time', function(time) {
	if(time.ms !== workTimer
	   || time.ms !== relaxTimer
	   || time.ms !== longRelaxTimer) {
		mb.tray.setTitle(timeFormat(new Date(time.ms)));
	}
	global.progress = getProgress();
	sender.send('update-timer');
});

global.timer.on('done', function() {
	global.isRelaxTime = isRelaxTime;
	global.pomodoroCount = pomodoroCount;
	
	sender.send('end-timer');
	
	if(isRelaxTime) {
		global.timer.reset(workTimer);
		isRelaxTime = false;
	} else {
		pomodoroCount++;
		if(pomodoroCount % 4 === 0) {
			global.timer.reset(longRelaxTimer);
		} else {
			global.timer.reset(relaxTimer);
		}
		
		isRelaxTime = true;
	}
});

ipc.on('reset-timer', function(event) {
	global.timer.reset(workTimer);
	mb.tray.setTitle('');
	
	event.sender.send('update-timer', 0);
});

ipc.on('start-timer', function(event) {
	sender = event.sender;
	if(global.timer.runTimer) {
		global.timer.stop();
		sender.send('update-timer');
		mb.tray.setTitle('Paused');
	} else {
		global.timer.start();
	}
});

ipc.on('settings-updated', function(event) {
	getConfig();
	
	if(sender) {
		sender.send('update-timer', getProgress());
	} else {
		event.sender.send('update-timer', getProgress());
	}
});

ipc.on('request-config', function(event) {
	getConfig();
	
	event.returnValue = { 
		workTimer: workTimer / 60 / 1000, 
		relaxTimer: relaxTimer / 60 / 1000, 
		longRelaxTimer: longRelaxTimer / 60 / 1000,
		launchOnStartup: launchOnStartup
	};
});

function getConfig() {
//	try {
//		var dataPath = path.join(mb.app.getDataPath(), 'config.json');
//		var data = JSON.parse(fs.readFileSync(dataPath));
//		workTimer = data.workTimer * 60 * 1000;
//		relaxTimer = data.relaxTimer * 60 * 1000;
//		longRelaxTimer = data.longRelaxTimer * 60 * 1000;
//		launchOnStartup = data.launchOnStartup;
//		if(launchOnStartup) {
//			autolauncher.enable();
//		} else {
//			autolauncher.disable();
//		}
//	} catch(err) {
//		console.log(err);
//		console.log('Didn\'t found previous config. Using default settings');
//	}
}

function getProgress() {
	var progress;
	if(isRelaxTime) {
		if(pomodoroCount % 4 === 0) {
			progress = (longRelaxTimer - timer.ms) / (longRelaxTimer / 100) * 0.01;
		} else {
			progress = (relaxTimer - timer.ms) / (relaxTimer / 100) * 0.01;
		}
	} else {
		progress = (workTimer - timer.ms) / (workTimer / 100) * 0.01;
	}
	
	if(progress < 0) {
		progress = 0.01;
	}
	return progress;
}
