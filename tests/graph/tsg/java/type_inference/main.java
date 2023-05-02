class Poco {
    int wifi;
}

class Main {
    Type field;
    Poco nfc;

    public void foo() {
        field.method(nfc.wifi);
    }
}

class SubMain extends Main {
    public void bar() {
        field.method(nfc.wifi);
    }
}
