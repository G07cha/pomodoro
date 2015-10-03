'use strict';
const menubar = require('menubar');
const Menu = require('menu');

// report crashes to the Electron project
require('crash-reporter').start();

let mb = menubar({
	'preloadWindow': true
});

process.on('uncaughtException', (err) => {
	dialog.showErrorBox('Uncaught Exception: ' + err.message, err.stack || '');
	mb.app.quit();
});

mb.on('ready', () => {
	mb.positioner.browserWindow.icon = __dirname + 'src/img/icon.png'
	console.log(JSON.stringify(mb, null, 2));

});
