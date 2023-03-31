enum Enum {
  A(0), B(1), C(2);
  int i;

  Enum(int j) {
    i = j;
  }

  String toString() {
    return "Enum(" + i + ")";
  }
}
