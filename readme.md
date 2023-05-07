# Pomodoro

Time management tool for Windows and macOS inspired by [Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique). Build with love and [tauri](https://tauri.app/).

<p align="center">
  <img src="./screenshot.png" alt="Screenshot of the application"/>
</p>

## Installing

To install the application you can grab the installer for your platform from [the latest release](github.com/G07cha/pomodoro/releases/latest/), in case of macOS you need to open the app for the first time with Cmd + right mouse button as I don't feel like spending money on a license from Apple to notarize a free application. Alternatively you can also build the release yourself, to do so please follow the guide below.

## Local development

### Prerequisites

- Node.js (preferably installed via [nvm](https://github.com/nvm-sh/nvm) to match local version)
- [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

```bash
npm install
```

To run full app execute the following command:

```bash
npm start
```

To build a release for the platform you're running it on execute the following command:

```bash
npm run build
```

It will create binaries and installers in `src-tauri/target/release/bundle`.

## License

MIT © Konstantin Azizov
