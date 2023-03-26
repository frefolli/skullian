package com;

class C {
    public static void method() {
        method();
    }
}

class A {
    class B {
        public static void method() {
            method();
        }
    }
}

class Main {
    public static void main() {
        C.method();
        A.B.method();
    }
}