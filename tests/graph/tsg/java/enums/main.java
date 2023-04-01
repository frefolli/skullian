enum Enum {
  A(0), B(1), C(2);
  int i;

  Enum(int j) {
    i = j;
  }

  void assign(int j) {
    this.i = j;
  }

  String toString() {
    return "Enum(" + i + ")";
  }
}