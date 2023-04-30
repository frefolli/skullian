# Quality Report

## Problems

`Problems(problem_id: TEXT, description: TEXT, causes: TEXT)`

| problem_id | description | causes |
| --------- | ----------- | ------ |
|  |  |  |

## Tests

## ".\\tests\\graph\\tsg\\cpp\\class_constructor_declaration\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| main.cpp::Class | class | OK |
| main.cpp::Class.Class | function | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| main.cpp::Class.Class | main.cpp::Class | definedBy | OK |

## ".\\tests\\graph\\tsg\\cpp\\class_declaration\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| main.cpp::Class | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |

## ".\\tests\\graph\\tsg\\cpp\\class_destructor_declaration\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| main.cpp::Class | class | OK |
| main.cpp::Class.~Class | function | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| main.cpp::Class.~Class | main.cpp::Class | definedBy | OK |

## ".\\tests\\graph\\tsg\\cpp\\class_field_declaration\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| main.cpp::Class | class | OK |
| main.cpp::Class.field | attribute | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| main.cpp::Class.field | main.cpp::Class | definedBy | OK |

## ".\\tests\\graph\\tsg\\cpp\\class_field_uses_type\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| main.cpp::Class | class | OK |
| main.cpp::Class.field | attribute | OK |
| main.cpp::Type | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| main.cpp::Class.field | main.cpp::Class | definedBy | OK |
| main.cpp::Class.field | main.cpp::Type | usesType | OK |

## ".\\tests\\graph\\tsg\\cpp\\class_function_declaration\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| main.cpp::Class | class | OK |
| main.cpp::Class.function | function | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| main.cpp::Class.function | main.cpp::Class | definedBy | OK |

## ".\\tests\\graph\\tsg\\cpp\\class_function_uses_type\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| main.cpp::Class | class | OK |
| main.cpp::Class.function | function | OK |
| main.cpp::Type | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| main.cpp::Class.function | main.cpp::Class | definedBy | OK |
| main.cpp::Class.function | main.cpp::Type | usesType | OK |

## ".\\tests\\graph\\tsg\\cpp\\class_inheritance\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| main.cpp::Father | class | OK |
| main.cpp::Mother | class | OK |
| main.cpp::Child | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| main.cpp::Child | main.cpp::Mother | isChildOf | OK |
| main.cpp::Child | main.cpp::Father | isChildOf | OK |

## ".\\tests\\graph\\tsg\\cpp\\field_declaration\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| main.cpp::field | attribute | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |

## ".\\tests\\graph\\tsg\\cpp\\field_uses_type\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| main.cpp::field | attribute | OK |
| main.cpp::Type | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| main.cpp::field | main.cpp::Type | usesType | OK |

## ".\\tests\\graph\\tsg\\cpp\\function_declaration\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| main.cpp::function | function | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |

## ".\\tests\\graph\\tsg\\cpp\\function_parameter_declaration\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| main.cpp::Class | class | OK |
| main.cpp::Class.function | function | OK |
| main.cpp::Class.function.parameter | parameter | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| main.cpp::Class.function | main.cpp::Class | definedBy | OK |
| main.cpp::Class.function.parameter | main.cpp::Class.function | definedBy | OK |

## ".\\tests\\graph\\tsg\\cpp\\function_parameter_uses_type\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| main.cpp::Class | class | OK |
| main.cpp::Class.function | function | OK |
| main.cpp::Class.function.parameter | parameter | OK |
| main.cpp::Type | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| main.cpp::Class.function | main.cpp::Class | definedBy | OK |
| main.cpp::Class.function.parameter | main.cpp::Class.function | definedBy | OK |
| main.cpp::Class.function.parameter | main.cpp::Type | usesType | OK |

## ".\\tests\\graph\\tsg\\cpp\\function_uses_type\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| main.cpp::function | function | OK |
| main.cpp::Type | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| main.cpp::function | main.cpp::Type | usesType | OK |

## ".\\tests\\graph\\tsg\\cpp\\package_declaration\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| main.cpp::package | package | OK |
| main.cpp::package.subpackage | package | OK |
| main.cpp::package.subpackage.Class | class | OK |
| main.cpp::functionA | function | OK |
| main.cpp::package.functionB | function | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| main.cpp::package.subpackage | main.cpp::package | nestedTo | OK |
| main.cpp::package.subpackage.Class | main.cpp::package.subpackage | definedBy | OK |
| main.cpp::functionA | main.cpp::package.subpackage.Class | usesType | OK |
| main.cpp::package.functionB | main.cpp::package.subpackage.Class | usesType | OK |