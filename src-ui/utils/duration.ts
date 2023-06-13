export class Duration {
  constructor(private durationMs: number) {}

  static fromSecs = (secs: number) => new Duration(secs * 1000);

  static fromMins = (minutes: number) => new Duration(minutes * 60 * 1000);

  get mins() {
    return Math.floor(this.secs / 60);
  }

  get secs() {
    return Math.floor(this.durationMs / 1000);
  }

  get millis() {
    return this.durationMs;
  }
}
