# Quality Report for Java

## As Is

### Relationships

`Relationships(relationship_id: TEXT, fullname: TEXT, notes: TEXT)`

| relationship_id | fullname | notes |
| -------------- | --------- | ----- |
| AF | accessField |  |
| IC | includes |  |
| UT | usesType |  |
| CA | calls |  |
| CO | isChildOf |  |
| IO | isImplementationOf |  |
|  |  |  |

### Problems

`Problems(problem_id: TEXT, description: TEXT, causes: TEXT)`

| problem_id | description | causes |
| --------- | ----------- | ------ |
| IC0 | when a package is defined import statements cannot be used as bridges to resolve names | N/A |
| AF0 | extension doesn't serve as a bridge to resolve names |  |
| AF1 | implementation doesn't serve as a bridge to resolve names |  |
| AF2 | typing doesn't serve as a bridge to resolve names |  |
| UT0 | annotation usage isn't used to detect a usesType |  |
| UT0 | cast usage isn't used to detect a usesType |  |
| CA0 | `new` doesn't trigger call of constructor |

## Prujit

### Detection

`Detection(benchmark_id: TEXT, from: TEXT, relationship: TEXT, to: TEXT, detected: ENUM("NO", "YES"), problems: TEXT)`

| benchmark_id | from | relationship | to | detected | problems |
| ------------ | ---- | ------------ | --- | ------- | -------- |
| ACC-1 | domain.direct.allowed.AccessClassVariable.AccessClassVariable | AF | technology.direct.dao.CheckInDAO.currentLocation | NO | IC0 |
| ACC-2 | domain.direct.allowed.AccessClassVariableConstant.testAccessStaticFinalAttribute | AF | technology.direct.dao.UserDAO.name | YES |  |
| ACC-3 | domain.direct.allowed.AccessClassVariableInterface.testAccessFinalAttribute | AF | technology.direct.dao.ISierraDAO.NAME | YES |  |
| ACC-4 | domain.direct.allowed.AccessEnumeration.AccessEnumeration | AF | technology.direct.dao.TipDAO.ONE | NO | IC0 |
| ACC-5 | domain.direct.allowed.AccessInstanceVariableRead.AccessInstanceVariableRead | AF | technology.direct.dao.ProfileDAO.name | NO | AF0 |
| ACC-6 | domain.direct.allowed.AccessInstanceVariableWrite.AccessInstanceVariableWrite | AF | technology.direct.dao.ProfileDAO.name | NO | AF0 |
| ACC-7 | domain.direct.allowed.AccessInstanceVariableConstant.testAccessFinalAttribute | AF | technology.direct.dao.UserDAO.message | NO | AF0 |
| ACC-8 | domain.direct.allowed.AccessInstanceVariableSuperClass.Method | AF | domain.direct.Base.VariableOnSuperClass | NO | AF0,AF2 |
| ACC-9 | domain.direct.allowed.AccessInstanceVariableSuperSuperClass.Method | AF | domain.direct.Base.VariableOnSuperClass | NO | AF0,AF2 |
| ACC-10 | domain.direct.allowed.AccessObjectReferenceAsParameter.AccessObjectReferenceAsParameter | AF | domain.direct.Base.profileDao | NO | AF0 |
| ACC-11 | domain.direct.allowed.AccessObjectReferenceWithinIfStatement.AccessObjectReferenceWithinIfStatement | AF | domain.direct.Base.profileDao | NO | AF0 |
| ANN-1 | domain.direct.allowed.AnnotationDependency | UT | technology.direct.dao.SettingsAnnotation | NO | UT0 |
| CAL-1 | domain.direct.allowed.CallClassMethod.CallClassMethod | CA | technology.direct.dao.BadgesDAO.getAllBadges | NO | IC0 |
| CAL-2 | domain.direct.allowed.CallConstructor.CallConstructor | CA | technology.direct.dao.AccountDAO.AccountDAO | NO | CA0 |
| CAL-3 | domain.direct.allowed.CallConstructorLibraryClass.handleCallback | CA | fi.foyt.foursquare.api.FoursquareApi.FoursquareApi | NO | CA0 |
| CAL-4 | domain.direct.allowed.CallInstance.CallInstance | CA | technology.direct.dao.ProfileDAO.getCampaignType | NO | AF1 |
| CAL-5 | domain.direct.allowed.CallInstanceInnerClass.CallMethodInstanceInnerClass | CA | technology.direct.dao.CallInstanceOuterClassDAO.CallInstanceInnerClassDAO.getNext | NO | AF1 |
| CAL-6 | domain.direct.allowed.CallInstanceInterface.test | CA | technology.direct.dao.CallInstanceInterfaceDAO.InterfaceMethod | NO | AF2 |
| CAL-7 | domain.direct.allowed.CallInstanceLibraryClass.handleCallback | CA | fi.foyt.foursquare.api.FoursquareApi.getAuthenticationUrl | NO | AF1,AF2 |
| CAL-8 | domain.direct.allowed.CallInstanceSuperClass.MethodOfSuperClass | CA | technology.direct.subclass.CallInstanceSubClassDOA.MethodOnSuperClass | NO | AF1 |
| CAL-9 | domain.direct.allowed.CallInstanceSuperSuperClass.MethodOfSuperClass | CA | technology.direct.subclass.CallInstanceSubSubClassDOA.MethodOnSuperClass | NO | AF1 |
| DEC-1 | domain.direct.allowed.DeclarationExceptionThrows.getStatics | CA | technology.direct.dao.StaticsException.StaticsException | NO |  |
| DEC-2 | domain.direct.allowed.DeclarationParameter.getProfileInformation.dao | UT | technology.direct.dao.ProfileDAO | YES |  |
| DEC-3 | domain.direct.allowed.DeclarationReturnType.getVenues | UT | technology.direct.dao.VenueDAO | YES |  |
| DEC-4 | domain.direct.allowed.DeclarationTypeCast.getProfileInformation | UT | technology.direct.dao.ProfileDAO | NO | UT1 |
| DEC-5 | domain.direct.allowed.DeclarationTypeCastOfArgument.initializeProfileInformation | UT | technology.direct.dao.ProfileDAO | NO | UT1 |
| DEC-6 | domain.direct.allowed.DeclarationVariableInstance.pdao | UT | technology.direct.dao.ProfileDAO | YES |  |
| DEC-7 | domain.direct.allowed.DeclarationVariableLocal.getProfileInformation.pdao | UT | technology.direct.dao.ProfileDAO | YES |  |
| DEC-7 | domain.direct.allowed.DeclarationVariableLocal_Initialized.getProfileInformation.pdao | UT | technology.direct.dao.ProfileDAO | YES |  |
| DEC-9 | domain.direct.allowed.DeclarationVariableStatic.pdao | UT | technology.direct.dao.ProfileDAO | YES |  |
| IMP-1 | domain.direct.allowed | IC | technology.direct.dao.AccountDAO | YES |  |
| INH-1 | domain.direct.allowed.InheritanceExtends | CO | technology.direct.dao.HistoryDAO | YES |  |
| INH-2 | domain.direct.allowed.InheritanceExtendsAbstractClass | CO | technology.direct.dao.FriendsDAO | YES |  |
| INH-3 | domain.direct.allowed.InheritanceImplementsInterface | IO | technology.direct.dao.IMapDAO | YES |  |
