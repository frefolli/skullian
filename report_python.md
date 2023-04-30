# Quality Report

## Problems

`Problems(problem_id: TEXT, description: TEXT, causes: TEXT)`

| problem_id | description | causes |
| --------- | ----------- | ------ |
|  |  |  |

## Tests

## ".\\tests\\graph\\tsg\\python\\class_constructor_declaration\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| Class | class | OK |
| Class.__init__ | function | OK |
| Class.__init__.self | parameter | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| Class.__init__ | Class | definedBy | OK |
| Class.__init__.self | Class.__init__ | definedBy | OK |

## ".\\tests\\graph\\tsg\\python\\class_declaration\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| Class | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |

## ".\\tests\\graph\\tsg\\python\\class_function_declaration\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| Class | class | OK |
| Class.function | function | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| Class.function | Class | definedBy | OK |

## ".\\tests\\graph\\tsg\\python\\function_declaration\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| function | function | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |

## ".\\tests\\graph\\tsg\\python\\function_parameter_declaration\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| Class | class | OK |
| Class.function | function | OK |
| Class.function.parameter | parameter | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| Class.function | Class | definedBy | OK |
| Class.function.parameter | Class.function | definedBy | OK |