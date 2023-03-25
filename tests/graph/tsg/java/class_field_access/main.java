package com;

class A {
    class B {
        public static int field;
        public static void method() {

        }
    }
}

class Main {
    public static void main() {
        A.B.method + 1;
        A.B.field = 2;
    }
}