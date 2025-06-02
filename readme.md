# Pomodoro

Time management tool for Windows and macOS inspired by [Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique). Build with love and [tauri](https://tauri.app/).

<p align="center">
  <img src="./screenshot.png" alt="Screenshot of the application"/>
</p>

## Installing

To install the application you can grab the installer for your platform from [the latest release](https://github.com/G07cha/pomodoro/releases/latest/), in case of macOS you need to open the app for the first time with Cmd + right mouse button as I don't feel like spending money on a license from Apple to notarize a free application. Alternatively you can also build the release yourself by following the guide below.

## Local development

### Prerequisites

- Node.js (preferably installed via [nvm](https://github.com/nvm-sh/nvm) to match local version)
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

```bash
npm install
```

### Building

To run full app execute the following command:

```bash
npm start
```

To build a release for the platform you're running it on execute the following command:

```bash
TAURI_PRIVATE_KEY=test npm run build
```

It will throw an error `Error invalid utf-8 sequence of 1 bytes from index 0` in the end due to mismatch in updater private key but will create binaries and installers in `src-tauri/target/release/bundle`.

## Release

To release a new version you'll need to have `cargo-bump` package installed:

```bash
cargo install cargo-bump
```

Afterwards you can bump the npm package version with the following command which will also bump the Cargo version:

```bash
npm version (patch|minor|major)
```

And then push it along with the tags to the main branch. Github Actions will create a new draft release and populate it with compiled binaries.

## License

MIT © Konstantin Azizov
