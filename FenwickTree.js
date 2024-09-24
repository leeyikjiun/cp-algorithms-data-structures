class FenwickTree {
  bit;

  constructor(n) {
    this.bit = new Array(n).fill(0);
  }

  #sum(r) {
    let ret = 0;
    for (; r >= 0; r = (r & (r + 1)) - 1) {
      ret += this.bit[r];
    }
    return ret;
  }

  sum(l, r) {
    if (r === undefined) return this.#sum(l);
    let ret = this.#sum(r);
    if (l >= 1) ret -= this.#sum(l - 1);
    return ret;
  }

  add(idx, delta) {
    for (; idx < this.bit.length; idx |= (idx + 1)) {
      this.bit[idx] += delta;
    }
  }
}
