export class EllidedCircularBuffer<T, E> implements Iterable<T | E> {
  private buffer: (T | E)[];
  private i = 0;
  private readonly ellisionMarker;

  constructor(size: number, ellisionMarker: E) {
    this.buffer = new Array(size + 1);
    this.ellisionMarker = ellisionMarker;
  }

  push(element: T) {
    this.buffer[this.i] = element;

    if (++this.i == this.buffer.length) {
      this.buffer[0] = this.ellisionMarker;
      this.i = 1;
    }
  }

  hasElements() {
    return this.i > 0;
  }

  empty() {
    if (this.hasElements()) {
      this.buffer = new Array(this.buffer.length);
      this.i = 0;
    }
  }

  length() {
    return this.buffer.reduce((count) => count + 1, 0);
  }

  [Symbol.iterator]() {
    return Object.values(this.buffer)[Symbol.iterator]();
  }
}
