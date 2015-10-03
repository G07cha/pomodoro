'use strict';
const menubar = require('menubar');
const Menu = require('menu');

// report crashes to the Electron project
require('crash-reporter').start();

// adds debug features like hotkeys for triggering dev tools and reload
require('electron-debug')();

// prevent window being garbage collected
let mainWindow;
let mb = menubar({
	'preloadWindow': true
});

process.on('uncaughtException', (err) => {
	dialog.showErrorBox('Uncaught Exception: ' + err.message, err.stack || '');
	mb.app.quit();
});

mb.on('ready', () => {
});
