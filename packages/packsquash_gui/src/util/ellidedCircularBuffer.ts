/**
 * A data type that implements a circular buffer that, when reaching its maximum size,
 * replaces its oldest elements and inserts an elision marker in the beginning,
 * signalling that some elements were dropped.
 */
export class EllidedCircularBuffer<T, E> implements Iterable<T | E> {
  private buffer: (T | E)[];
  private i = 0;
  private readonly elisionMarker;

  constructor(size: number, ellisionMarker: E) {
    this.buffer = new Array(size + 1);
    this.elisionMarker = ellisionMarker;
  }

  push(element: T) {
    this.buffer[this.i] = element;

    if (++this.i == this.buffer.length) {
      this.buffer[0] = this.elisionMarker;
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
