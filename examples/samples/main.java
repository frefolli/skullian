public class Main extends Object {
  public static void main() {
    System.out.println("ciao");
    foo(false);
  }

  public static void foo(boolean flag) {
    System.out.println("foo");
    bar(! flag);
  }

  public static void bar(boolean flag) {
    System.out.println("bar");
    if (flag) {
      foo();
    }
  }
}
