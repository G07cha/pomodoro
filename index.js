'use strict';
const menubar = require('menubar');
const Menu = require('menu');
const globalShortcut = require('global-shortcut');
const ipc = require('ipc');
const dialog = require('dialog');
const Stopwatch = require('timer-stopwatch');
const Hrt = require('human-readable-time');
const fs = require('fs');


// report crashes to the Electron project
require('crash-reporter').start();

let timeFormat = new Hrt('%mm%:%ss%');
let workTimer = 1500000;
let relaxTimer = 300000;
let longRelaxTimer = 900000;
let pomodoroCount = 0;
let isRelaxTime = false;
let sender;

let mb = menubar({
	'preloadWindow': true
});

getConfig();

global.timer = new Stopwatch(workTimer);

process.on('uncaughtException', (err) => {
	dialog.showErrorBox('Uncaught Exception: ' + err.message, err.stack || '');
	mb.app.quit();
});

mb.on('will-quit', () => {
	globalShortcut.unregisterAll();
});

global.timer.on('time', function(time) {
	mb.tray.setTitle(timeFormat(new Date(time.ms)));
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
	mb.tray.setTitle('');
	global.timer.reset(workTimer);
	
	event.sender.send('update-timer', 0);
});

ipc.on('start-timer', function(event) {
	sender = event.sender;
	if(global.timer.runTimer) {
		global.timer.stop();
	} else {
		global.timer.start();
	}
});

ipc.on('settings-updated', function(event) {
	getConfig();
	event.sender.send('update-timer', getProgress());
});

ipc.on('request-config', function(event) {
	getConfig();
	
	event.returnValue = { 
		workTimer: workTimer / 60 / 1000, 
		relaxTimer: relaxTimer / 60 / 1000, 
		longRelaxTimer: longRelaxTimer / 60 / 1000
	};
});

function getConfig() {
	try {
		var data = JSON.parse(fs.readFileSync(mb.app.getDataPath() + '/config.json'));
		workTimer = data.workTimer * 60 * 1000;
		relaxTimer = data.relaxTimer * 60 * 1000;
		longRelaxTimer = data.longRelaxTimer * 60 * 1000;
	} catch(err) {
		console.log(err);
		console.log('Didn\'t found previous config. Using default settings');
	}
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
	
	return progress;
}
