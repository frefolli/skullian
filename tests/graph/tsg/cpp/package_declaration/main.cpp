namespace package {
    namespace subpackage {
        class Class {
        };
    }

    subpackage::Class functionB();
}

package::subpackage::Class functionA();