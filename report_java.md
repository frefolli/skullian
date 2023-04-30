# Quality Report

## Problems

`Problems(problem_id: TEXT, description: TEXT, causes: TEXT)`

| problem_id | description | causes |
| --------- | ----------- | ------ |
|  |  |  |

## Tests

## ".\\tests\\graph\\tsg\\java\\annotation\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.Main | class | OK |
| com.Annotation | annotation | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main.foo | com.Annotation | usesType | OK |

## ".\\tests\\graph\\tsg\\java\\array_creation\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.Main | class | OK |
| com.Type | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main.foo | com.Type | usesType | OK |

## ".\\tests\\graph\\tsg\\java\\benchmark\\test.yml"

| node | kind | detected |
| --- | --- | --- |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| domain.direct.violating.AccessClassVariable.AccessClassVariable | technology.direct.dao.CheckInDAO.currentLocation | accessField | OK |
| domain.direct.violating.AccessClassVariableConstant.testAccessStaticFinalAttribute | technology.direct.dao.UserDAO.name | accessField | OK |
| domain.direct.violating.AccessClassVariableInterface.testAccessFinalAttribute | technology.direct.dao.ISierraDAO.NAME | accessField | OK |
| domain.direct.violating.AccessEnumeration.AccessEnumeration | technology.direct.dao.TipDAO.ONE | accessField | OK |
| domain.direct.violating.AccessInstanceVariableRead.AccessInstanceVariableRead | technology.direct.dao.ProfileDAO.name | accessField | NO |
| domain.direct.violating.AccessInstanceVariableWrite.AccessInstanceVariableWrite | technology.direct.dao.ProfileDAO.name | accessField | NO |
| domain.direct.violating.AccessInstanceVariableConstant.testAccessFinalAttribute | technology.direct.dao.UserDAO.message | accessField | NO |
| domain.direct.violating.AccessInstanceVariableSuperClass.Method | technology.direct.dao.CallInstanceSuperClassDAO.VariableOnSuperClass | accessField | NO |
| domain.direct.violating.AccessInstanceVariableSuperSuperClass.Method | technology.direct.dao.CallInstanceSuperClassDAO.VariableOnSuperClass | accessField | NO |
| domain.direct.violating.AccessObjectReferenceAsParameter.AccessObjectReferenceAsParameter | domain.direct.Base.profileDao | accessField | NO |
| domain.direct.violating.AccessObjectReferenceWithinIfStatement.AccessObjectReferenceWithinIfStatement | domain.direct.Base.profileDao | accessField | NO |
| domain.direct.violating.AnnotationDependency | technology.direct.dao.SettingsAnnotation | usesType | NO |
| domain.direct.violating.CallClassMethod.CallClassMethod | technology.direct.dao.BadgesDAO.getAllBadges | calls | OK |
| domain.direct.violating.CallConstructor.CallConstructor | technology.direct.dao.AccountDAO | usesType | OK |
| domain.direct.violating.CallConstructorLibraryClass.handleCallback | fi.foyt.foursquare.api.FoursquareApi | usesType | NO |
| domain.direct.violating.CallInstance.CallInstance | technology.direct.dao.ProfileDAO.getCampaignType | calls | NO |
| domain.direct.violating.CallInstanceInnerClass.CallMethodInstanceInnerClass | technology.direct.dao.CallInstanceOuterClassDAO.CallInstanceInnerClassDAO.getNext | calls | NO |
| domain.direct.violating.CallInstanceInterface.test | technology.direct.dao.CallInstanceInterfaceDAO.InterfaceMethod | calls | NO |
| domain.direct.violating.CallInstanceLibraryClass.handleCallback | fi.foyt.foursquare.api.FoursquareApi.getAuthenticationUrl | calls | NO |
| domain.direct.violating.CallInstanceSuperClass.MethodOfSuperClass | technology.direct.dao.CallInstanceSuperClassDAO.MethodOnSuperClass | calls | NO |
| domain.direct.violating.CallInstanceSuperSuperClass.MethodOfSuperClass | technology.direct.dao.CallInstanceSuperClassDAO.MethodOnSuperClass | calls | NO |
| domain.direct.violating.DeclarationExceptionThrows.getStatics | technology.direct.dao.StaticsException | throwsType | OK |
| domain.direct.violating.DeclarationParameter.getProfileInformation.dao | technology.direct.dao.ProfileDAO | usesType | OK |
| domain.direct.violating.DeclarationReturnType.getVenues | technology.direct.dao.VenueDAO | usesType | OK |
| domain.direct.violating.DeclarationTypeCast.getProfileInformation | technology.direct.dao.ProfileDAO | castsType | OK |
| domain.direct.violating.DeclarationTypeCastOfArgument.initializeProfileInformation | technology.direct.dao.ProfileDAO | castsType | OK |
| domain.direct.violating.DeclarationVariableInstance.pdao | technology.direct.dao.ProfileDAO | usesType | OK |
| domain.direct.violating.DeclarationVariableLocal.getProfileInformation.pdao | technology.direct.dao.ProfileDAO | usesType | OK |
| domain.direct.violating.DeclarationVariableLocal_Initialized.getProfileInformation.pdao | technology.direct.dao.ProfileDAO | usesType | OK |
| domain.direct.violating.DeclarationVariableStatic.pdao | technology.direct.dao.ProfileDAO | usesType | OK |
| domain.direct.violating | technology.direct.dao.AccountDAO | includes | OK |
| domain.direct.violating.InheritanceExtends | technology.direct.dao.HistoryDAO | isChildOf | OK |
| domain.direct.violating.InheritanceExtendsAbstractClass | technology.direct.dao.FriendsDAO | isChildOf | OK |
| domain.direct.violating.InheritanceImplementsInterface | technology.direct.dao.IMapDAO | isImplementationOf | OK |

## ".\\tests\\graph\\tsg\\java\\casts_type\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.Main | class | OK |
| com.Type | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main.foo | com.Type | castsType | OK |

## ".\\tests\\graph\\tsg\\java\\class_constructors\\main.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.Main | class | OK |
| com.Main.Main | function | OK |
| com.Main.Main.a | parameter | OK |
| com.Main.Main.b | attribute | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main | com | definedBy | OK |
| com.Main.Main | com.Main | definedBy | OK |
| com.Main.Main.a | com.Main.Main | definedBy | OK |
| com.Main.Main.b | com.Main.Main | definedBy | OK |

## ".\\tests\\graph\\tsg\\java\\class_field_access\\main.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.Main | class | OK |
| com.A | class | OK |
| com.A.B | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main.main | com.A.B.method | accessField | OK |
| com.Main.main | com.A.B.field | accessField | OK |

## ".\\tests\\graph\\tsg\\java\\class_implementation\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.A | interface | OK |
| com.B | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.B | com.A | isImplementationOf | OK |

## ".\\tests\\graph\\tsg\\java\\class_inheritance\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.A | class | OK |
| com.B | class | OK |
| com.A.C | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.B | com.A.C | isChildOf | OK |
| com.A.C | com.A | nestedTo | OK |

## ".\\tests\\graph\\tsg\\java\\class_inheritance_with_packages\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.A | class | OK |
| com.B | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.B | com.A | isChildOf | OK |

## ".\\tests\\graph\\tsg\\java\\class_methods\\main.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.Main | class | OK |
| com.Main.foo | function | OK |
| com.Main.bar | function | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main | com | definedBy | OK |
| com.Main.foo | com.Main | definedBy | OK |
| com.Main.bar | com.Main | definedBy | OK |

## ".\\tests\\graph\\tsg\\java\\class_methods_with_attributes\\main.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.Main | class | OK |
| com.Main.foo | function | OK |
| com.Main.foo.args | attribute | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main | com | definedBy | OK |
| com.Main.foo | com.Main | definedBy | OK |
| com.Main.foo.args | com.Main.foo | definedBy | OK |

## ".\\tests\\graph\\tsg\\java\\class_methods_with_parameters\\main.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.Main | class | OK |
| com.Main.foo | function | OK |
| com.Main.bar | function | OK |
| com.Main.foo.args | parameter | OK |
| com.Main.bar.argc | parameter | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main | com | definedBy | OK |
| com.Main.foo | com.Main | definedBy | OK |
| com.Main.bar | com.Main | definedBy | OK |
| com.Main.foo.args | com.Main.foo | definedBy | OK |
| com.Main.bar.argc | com.Main.bar | definedBy | OK |

## ".\\tests\\graph\\tsg\\java\\class_method_call\\main.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.Main | class | OK |
| com.A | class | OK |
| com.A.B | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main.main | com.A.B.method | calls | OK |
| com.A.B.method | com.A.B.method | calls | OK |
| com.C.method | com.A.B.method | calls | OK |
| com.C.method | com.C.method | calls | OK |

## ".\\tests\\graph\\tsg\\java\\class_packages\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| com.Main | class | OK |
| com.Foo | class | OK |
| com.sub.Bar | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main | com | definedBy | OK |
| com.Foo | com | definedBy | OK |
| com.sub.Bar | com.sub | definedBy | OK |
| com | com.sub.Bar | includes | OK |

## ".\\tests\\graph\\tsg\\java\\class_type_usage\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| com.Main | class | OK |
| com.Foo | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main | com | definedBy | OK |
| com.Foo | com | definedBy | OK |
| com.Main.main | com.Main | definedBy | OK |
| com.Main.bar | com.Main | definedBy | OK |
| com.Main.bing | com.Foo | usesType | OK |
| com.Main.main.obj1 | com.Foo | usesType | OK |
| com.Main.bar | com.Foo | usesType | OK |

## ".\\tests\\graph\\tsg\\java\\class_type_usage_nested_packages\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| com.net.Main | class | OK |
| com.net.Foo | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.net.Main | com.net | definedBy | OK |
| com.net.Foo | com.net | definedBy | OK |
| com.net.Main.main | com.net.Main | definedBy | OK |
| com.net.Main.bar | com.net.Main | definedBy | OK |
| com.net.Main.bing | com.net.Foo | usesType | OK |
| com.net.Main.main.obj1 | com.net.Foo | usesType | OK |
| com.net.Main.bar | com.net.Foo | usesType | OK |

## ".\\tests\\graph\\tsg\\java\\class_with_attributes\\main.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.Main | class | OK |
| com.Main.args | attribute | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main | com | definedBy | OK |
| com.Main.args | com.Main | definedBy | OK |

## ".\\tests\\graph\\tsg\\java\\enums\\main.yml"

| node | kind | detected |
| --- | --- | --- |
| Enum | enum | OK |
| Enum.toString | function | OK |
| Enum.A | enumVariant | OK |
| Enum.B | enumVariant | OK |
| Enum.C | enumVariant | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| Enum.Enum | Enum | definedBy | OK |
| Enum.toString | Enum | definedBy | OK |
| Enum.A | Enum | definedBy | OK |
| Enum.Enum | Enum.i | accessField | OK |
| Enum.assign | Enum.i | accessField | OK |
| Enum.Enum | Enum.Enum.j | accessField | OK |

## ".\\tests\\graph\\tsg\\java\\extension_bridge\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| Main | class | OK |
| Main.field | attribute | OK |
| Main.foo | function | OK |
| Type | class | OK |
| Base | class | OK |
| Base.method | function | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| Main.foo | Base.method | calls | OK |

## ".\\tests\\graph\\tsg\\java\\implementation_bridge\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| Main | class | OK |
| Main.field | attribute | OK |
| Main.foo | function | OK |
| Type | class | OK |
| Base | interface | OK |
| Base.method | function | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| Main.foo | Base.method | calls | OK |

## ".\\tests\\graph\\tsg\\java\\imports\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| com.Main | class | OK |
| com.Bing | class | OK |
| com.sub.Foo | class | OK |
| com.sub.Bar | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main | com | definedBy | OK |
| com.Bing | com | definedBy | OK |
| com.sub.Foo | com.sub | definedBy | OK |
| com.sub.Bar | com.sub | definedBy | OK |
| com | com.sub.Bar | includes | OK |
| com.Main.main.obj3 | com.Bing | usesType | OK |
| com.Main.main.obj2 | com.sub.Foo | usesType | OK |
| com.Main.main.obj1 | com.sub.Bar | usesType | OK |

## ".\\tests\\graph\\tsg\\java\\interface_inheritance\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.A | interface | OK |
| com.B | interface | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.B | com.A | isChildOf | OK |

## ".\\tests\\graph\\tsg\\java\\nested_classes\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.A | class | OK |
| com.A.B | class | OK |
| com.A.B.C | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.A.B | com.A | nestedTo | OK |
| com.A.B.C | com.A.B | nestedTo | OK |

## ".\\tests\\graph\\tsg\\java\\object_creation\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.Main | class | OK |
| com.Type | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main.foo | com.Type | usesType | OK |

## ".\\tests\\graph\\tsg\\java\\throws_type\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| com | package | OK |
| com.Main | class | OK |
| com.Type | class | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main.foo | com.Type | throwsType | OK |

## ".\\tests\\graph\\tsg\\java\\type_inference\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| Main | class | OK |
| Main.field | attribute | OK |
| Main.foo | function | OK |
| Type | class | OK |
| Type.method | function | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| Main.foo | Type.method | calls | OK |
| SubMain.bar | Type.method | calls | OK |
| Main.foo | Poco.wifi | accessField | OK |
| SubMain.bar | Poco.wifi | accessField | OK |

## ".\\tests\\graph\\tsg\\java\\type_inference_with_packages\\test.yml"

| node | kind | detected |
| --- | --- | --- |
| com.Main | class | OK |
| com.Main.field | attribute | OK |
| com.Main.foo | function | OK |
| com.sub.Type | class | OK |
| com.sub.Type.method | function | OK |

| source | sink | kind | detected |
| --- | --- | --- | --- |
| com.Main.foo | com.sub.Type.method | calls | NO |
| com.SubMain.bar | com.sub.Type.method | calls | NO |
| com.Main.foo | com.Poco.wifi | accessField | OK |
| com.SubMain.bar | com.Poco.wifi | accessField | OK |