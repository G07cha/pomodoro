var remote = require('remote');
var dialog = remote.require('dialog');
var browserWindow = remote.require('browser-window');
window.$ = window.jQuery = require('jquery');

workTimer = 1500000;
relaxTimer = 300000;

$(document).ready(function() {
	$('.workTimer').attr('value', workTimer/1000/60);
	$('.relaxTimer').attr('value', relaxTimer/1000/60);
	$('.workValue').append(workTimer/1000/60);
	$('.relaxValue').append(relaxTimer/1000/60);
	
	$('.workTimer').change(function() {
		$('.workValue').empty();
		$('.workValue').append($(this).val());
	});
	$('.relaxTimer').change(function() {
		$('.relaxValue').empty();
		$('.relaxValue').append($(this).val());
	});
});