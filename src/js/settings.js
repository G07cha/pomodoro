var remote = require('remote');
var dialog = remote.require('dialog');
var app = remote.require('app');
var fs = require('fs');
var browserWindow = remote.require('browser-window');
window.$ = window.jQuery = require('jquery');

var settingsWindow = remote.getCurrentWindow();

var workTimer = 25;
var relaxTimer = 5;
var launchOnStartup = false;

/*
 * Load settings
 */
try {
	var data = JSON.parse(fs.readFileSync(app.getDataPath() + '/config.json'));
	workTimer = data.workTimer;
	relaxTimer = data.relaxTimer;
	launchOnStartup = data.launchOnStartup;
} catch(err) {
	console.log('Didn\'t found previous config. Using default settings');
}

$(document).ready(function() {
	/*
	 * Set sliders and checkbox with default value
	 */
	slider('work', workTimer);
	slider('relax', relaxTimer);
//	$('.launch').attr('checked', launchOnStartup);
	
	/*
	 * Save settings
	 */
	$('.save').on('click', function() {
		fs.writeFile(app.getDataPath() + '/config.json', JSON.stringify({
			workTimer: $('.workTimer').val(),
			relaxTimer: $('.relaxTimer').val(),
//			launchOnStartup: $('.launch').prop('checked')
		}), function(err) {
			if (err) {
				dialog.showErrorBox('Failed to save settings', err);
			} else {
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
});

/**
 * Update html slider with %value% 
 * @param {String} name  Part of name(work or relax)
 * @param {Number} value New slider value
 */
function slider(name, value) {
	var timerSelector = '.' + name + 'Timer';
	var valueSelector = timerSelector.replace('Timer', 'Value');
	$(timerSelector).attr('value', value);
	$(valueSelector).append(value);
	$(timerSelector).change(function() {
		$(valueSelector).empty();
		$(valueSelector).append($(this).val());
	});
}