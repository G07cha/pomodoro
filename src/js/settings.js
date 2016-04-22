var remote = require('remote');
var dialog = remote.require('dialog');
var app = remote.require('app');
var fs = require('fs');
var ipc = require('ipc');
var browserWindow = remote.require('browser-window');
window.$ = window.jQuery = require('jquery');

var settingsWindow = remote.getCurrentWindow();

var workTimer = 25;
var relaxTimer = 5;
var longRelaxTimer = 15;
var launchOnStartup = false;
var useCustomAlert = false;
var customAlertFilePath;

var configs = ipc.sendSync('request-config');

workTimer = configs.workTimer;
relaxTimer = configs.relaxTimer;
longRelaxTimer = configs.longRelaxTimer;
showTimer = configs.showTimer;
launchOnStartup = configs.launchOnStartup;
useCustomAlert = configs.useCustomAlert;
customAlertFilePath = configs.customAlertFilePath;

$(document).ready(function() {
	/*
	 * Set sliders and checkbox with default value
	 */
	slider('work', workTimer);
	slider('relax', relaxTimer);
	slider('longRelax', longRelaxTimer);
	$('.showTimer').attr('checked', showTimer);
	$('.launch').attr('checked', launchOnStartup);
	
	$('.customAlert').attr('checked', useCustomAlert);
	$('.customAlertPath').val(customAlertFilePath);

	/*
	 * Save settings
	 */
	$('.save').on('click', function() {
		fs.writeFile(app.getDataPath() + '/config.json', JSON.stringify({
			workTimer: $('div.work').slider('value'),
			relaxTimer: $('div.relax').slider('value'),
			longRelaxTimer: $('div.longRelax').slider('value'),
			showTimer: $('.showTimer').prop('checked'),
			launchOnStartup: $('.launch').prop('checked'),
			useCustomAlert: $('.customAlert').prop('checked'),
			customAlertFilePath: $('.customAlertPath').val()
		}), function(err) {
			if (err) {
				dialog.showErrorBox('Failed to save settings', err);
			} else {
				ipc.send('settings-updated');
				dialog.showMessageBox({
					title: 'Success',
					message: 'Changes saved successfully!',
					buttons: ['Ok']
				}, function() {
					settingsWindow.hide();
				});
			}
		});
	});
	
	/*
	 * Exit from settings without settings(Cancel action)
	 */
	$('.cancel').on('click', function() {
		dialog.showMessageBox({
			type: 'question',
			title: 'Warning',
			message: 'Settings will not save! Are you sure?',
			buttons: ['Yes', 'No']
		}, function(response) {
			//0 === "Yes" button
			if(response === 0) {
				settingsWindow.hide();
			}
		})
	});

	function toggleProps() {
		if ($('.customAlert').prop('checked')) {
			$('.customAlertPath').show();
			$('.testCustomAlert').show();
		} else {
			$('.customAlertPath').hide();
			$('.testCustomAlert').hide();
		}
	}

	toggleProps();
	$('.customAlert').on('click', function() {
		toggleProps();
	});

	$('.testCustomAlert').on('click', function() {
		ipc.send('test-alert-window', {
			file: $('.customAlertPath').val(),
			timeout: 4000
		});
	});
});

/**
 * Update html slider with %value% 
 * @param {String} name  Part of name(work or relax)
 * @param {Number} value New slider value
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