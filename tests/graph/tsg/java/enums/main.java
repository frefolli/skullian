enum Enum {
  A(0), B(1), C(2);
  int i;

  Enum(int i) {
    this.i = i;
  }

  String toString() {
    return "Enum(" + this.i + ")";
  }
}
