'use strict';

const { remote, ipcRenderer } = require('electron');
const { app, dialog } = remote;
const fs = require('fs');

window.$ = require('jquery');
window.jQuery = window.$;

const settingsWindow = remote.getCurrentWindow();

let workTimer = 25;
let relaxTimer = 5;
let longRelaxTimer = 15;
let launchOnStartup = false;
let showTimer = false;

let configs = ipcRenderer.sendSync('request-config');

workTimer = configs.workTimer;
relaxTimer = configs.relaxTimer;
longRelaxTimer = configs.longRelaxTimer;
showTimer = configs.showTimer;
launchOnStartup = configs.launchOnStartup;

$(document).ready(function() {

	/*
	 * Set sliders and checkbox with default value
	 */
	slider('work', workTimer);
	slider('relax', relaxTimer);
	slider('longRelax', longRelaxTimer);
	$('.showTimer').attr('checked', showTimer);
	$('.launch').attr('checked', launchOnStartup);

	/*
	 * Save settings
	 */
	$('#saveBtn').on('click', function() {
		fs.writeFile(
			app.getPath('userData') + '/config.json',
			JSON.stringify({
				workTimer: $('div.work').slider('value'),
				relaxTimer: $('div.relax').slider('value'),
				longRelaxTimer: $('div.longRelax').slider('value'),
				showTimer: $('.showTimer').prop('checked'),
				launchOnStartup: $('.launch').prop('checked')
			}),
			function(err) {
				if (err) {
					dialog.showErrorBox('Failed to save settings', err);
				} else {
					ipcRenderer.send('settings-updated');
					settingsWindow.hide();
				}
			}
		);
	});

	/*
	 * Exit from settings without settings(Cancel action)
	 */
	$('#cancelBtn').on('click', function() {
        let options = {
				type: 'question',
				title: 'Warning',
				message: 'Settings will not save! Are you sure?',
				buttons: ['Yes', 'No'],
                defaultId: 1,
                cancelId: 1
			}

		dialog.showMessageBox(options).then((result) => {
				// 0 === "Yes" button
				if (result.response === 0) {
					settingsWindow.hide();
                } else {
                    settingsWindow.show();
                }
            }
		);
	});
});

/**
 * Update html slider with %value%
 * @param {String} name  Part of name(work or relax)
 * @param {Number} value New slider value
 * @returns {void}
 */
function slider(name, value) {
	$('div.' + name).slider({
		value: value,
		min: 1,
		max: 60,
		slide: function(event, ui) {
			$('span.' + name).html(ui.value);
		}
	});
	$('span.' + name).html($('div.' + name).slider('value'));
}
