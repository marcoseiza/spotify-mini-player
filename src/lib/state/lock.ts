import { Mutex, type MutexInterface } from "async-mutex";

type Guard<T> = {
  release: MutexInterface.Releaser;
  set: (v: T) => void;
  get: () => T;
};

export class Lock<T> {
  private _value: T;
  private _mutex = new Mutex();

  constructor(value: T) {
    this._value = value;
  }

  get value() {
    return this._value;
  }

  set value(v: T) {
    if (this._mutex.isLocked()) return;
    this._value = v;
  }

  async acquire(): Promise<Guard<T>> {
    const release = await this._mutex.acquire();
    return {
      release,
      set: (v: T) => {
        this._value = v;
      },
      get: () => {
        return this._value;
      },
    };
  }
  runExclusive<T>(callback: MutexInterface.Worker<T>): Promise<T> {
    return this._mutex.runExclusive(callback);
  }
  waitForUnlock(): Promise<void> {
    return this._mutex.waitForUnlock();
  }
  isLocked(): boolean {
    return this._mutex.isLocked();
  }
  release(): void {
    return this._mutex.release();
  }
  cancel(): void {
    return this._mutex.cancel();
  }
}
