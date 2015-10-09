'use strict';
const menubar = require('menubar');
const Menu = require('menu');
const globalShortcut = require('global-shortcut');
const ipc = require('ipc');
const dialog = require('dialog');
const Stopwatch = require('timer-stopwatch');
const Hrt = require('human-readable-time');

// report crashes to the Electron project
require('crash-reporter').start();

let timeFormat = new Hrt('%mm%:%ss%');
let workTimer = 1500000;
let relaxTimer = 300000;
let longRelaxTimer = 900000;
let pomodoroCount = 0;
let isRelaxTime = false;
let sender;

try {
	var data = JSON.parse(fs.readFileSync(mb.app.getDataPath() + '/config.json'));
	workTimer = data.workTimer * 60 * 1000;
	relaxTimer = data.relaxTimer * 60 * 1000;
	longRelaxTimer = data.longRelaxTimer * 60 * 1000;
} catch(err) {
	console.log('Didn\'t found previous config. Using default settings');
}

global.global.timer = new Stopwatch(workTimer);

let mb = menubar({
	'preloadWindow': true
});

process.on('uncaughtException', (err) => {
	dialog.showErrorBox('Uncaught Exception: ' + err.message, err.stack || '');
	mb.app.quit();
});

mb.on('will-quit', () => {
	globalShortcut.unregisterAll();
});

global.timer.on('time', function(time) {
	mb.tray.setTitle(timeFormat(new Date(time.ms)));
	var progress;
	if(isRelaxTime) {
		if(pomodoroCount % 4 === 0) {
			progress = (longRelaxTimer - time.ms) / (longRelaxTimer / 100) * 0.01;
		} else {
			progress = (relaxTimer - time.ms) / (relaxTimer / 100) * 0.01;
		}
	} else {
		progress = (workTimer - time.ms) / (workTimer / 100) * 0.01;
	}
	
	sender.send('update-timer');
});

ipc.on('request-update', function(event) {
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
	
	event.returnValue = progress;
})

global.timer.on('done', function() {
	ipc.sendSync('end-timer', { 
		isRelaxTime: isRelaxTime,
		pomodoroCount: pomodoroCount 
	});
	
	if(isRelaxTime) {
		global.timer.reset(workTimer);
	} else {
		pomodoroCount++;
		if(pomodoroCount % 4 === 0) {
			global.timer.reset(longRelaxTimer);
		} else {
			global.timer.reset(relaxTimer);
		}
		
		isRelaxTime = true;
	}
	
	global.timer.start();
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
	try {
		var data = JSON.parse(fs.readFileSync(app.getDataPath() + '/config.json'));
		workTimer = data.workTimer * 60 * 1000;
		relaxTimer = data.relaxTimer * 60 * 1000;
		longRelaxTimer = data.longRelaxTimer * 60 * 1000;
	} catch(err) {
		console.log('Didn\'t found previous config. Using default settings');
	}
	
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
	
	event.sender.send('update-timer', progress);
});


function getConfig() {
	
}