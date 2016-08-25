import test from 'ava';
import {Application} from 'spectron';

test.beforeEach(async t => {
  t.context.app = new Application({
    path: './dist/pomodoro-darwin-x64/pomodoro.app/Contents/MacOS/Pomodoro'
  });

  await t.context.app.start();
});

test.afterEach.always(async t => {
  await t.context.app.stop();
});

test('Start/stop timer', async t => {
  const app = t.context.app;
  const {client} = app;

  await client.waitUntilWindowLoaded();

  const win = app.browserWindow;

  t.false(await win.isVisible(), 'Window isn\'t hidden on start');

  const {width, height} = await win.getBounds();

  t.true(width > 0);
  t.true(height > 0);

  t.is(await client.getText('.timer > strong'), 'Click to start', 'Timer doesn\'t show text on start');
  await client.leftClick('.timer');
  t.not(await client.getText('.timer > strong'), 'Click to start', 'Timer\'s text isn\'t changed after start');

  const timerData = await client.execute(function() {
    return $('.timer').circleProgress('value');
  });

  t.true(timerData.value > 0, 'Timer isn\'t started');
  await client.leftClick('.timer');
  t.is(await client.getText('.timer > strong'), 'Click to start', 'Timer is not stopped after click');
});

test('Reset timer', async t => {
  const app = t.context.app;
  const {client} = app;

  await client.waitUntilWindowLoaded();

  await client.leftClick('.timer');
  t.not(await client.getText('.timer > strong'), 'Click to start', 'Timer\'s text isn\'t changed after start');

  await client.leftClick('#resetBtn');
  await client.waitUntilTextExists('.timer > strong', 'Click to start', 1000);
  t.is(await client.getText('.timer > strong'), 'Click to start', 'Timer is not stopped after reset');

  const timerData = await client.execute(function() {
    return $('.timer').circleProgress('value');
  });

  t.true(timerData.value === 0, 'Timer value isn\'t resetted');
});

test('Open settings', async t => {
  const app = t.context.app;
  const {client} = app;

  await client.waitUntilWindowLoaded();

  await client.leftClick('#settingsBtn');
  t.is(await app.client.getWindowCount(), 2, 'Setting window is missing');
});

test('Main window accessibility', async t => {
  const app = t.context.app;

  app.client.windowByIndex(0).then(function() {
    app.client.auditAccessibility().then(function (audit) {
      if (audit.failed) {
        console.error('Main page failed audit')
        console.error(audit.message)
      }
    })
  })
})
