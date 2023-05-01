# Quality Report

## Problems

`Problems(problem_id: TEXT, description: TEXT, causes: TEXT)`

| problem_id | description | causes |
| --------- | ----------- | ------ |
|  |  |  |

## Tests

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
| domain.direct.violating.AccessObjectReferenceAsParameter.AccessObjectReferenceAsParameter | domain.direct.Base.profileDao | accessField | OK |
| domain.direct.violating.AccessObjectReferenceWithinIfStatement.AccessObjectReferenceWithinIfStatement | domain.direct.Base.profileDao | accessField | OK |
| domain.direct.violating.AnnotationDependency | technology.direct.dao.SettingsAnnotation | usesType | OK |
| domain.direct.violating.CallClassMethod.CallClassMethod | technology.direct.dao.BadgesDAO.getAllBadges | calls | OK |
| domain.direct.violating.CallConstructor.CallConstructor | technology.direct.dao.AccountDAO | usesType | OK |
| domain.direct.violating.CallConstructorLibraryClass.handleCallback | fi.foyt.foursquare.api.FoursquareApi | usesType | NO |
| domain.direct.violating.CallInstance.CallInstance | technology.direct.dao.ProfileDAO.getCampaignType | calls | NO |
| domain.direct.violating.CallInstanceInnerClass.CallMethodInstanceInnerClass | technology.direct.dao.CallInstanceOuterClassDAO.CallInstanceInnerClassDAO.getNext | calls | NO |
| domain.direct.violating.CallInstanceInterface.test | technology.direct.dao.CallInstanceInterfaceDAO.InterfaceMethod | calls | OK |
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