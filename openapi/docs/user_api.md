# user_api

All URIs are relative to *http://petstore.swagger.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
**createUser**](user_api.md#createUser) | **POST** /user | Add a new user to the store


# **createUser**
> serde_json::Value createUser(ctx, optional)
Add a new user to the store



### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **create_user_request** | [**CreateUserRequest**](CreateUserRequest.md)|  | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

[user_write](../README.md#user_write)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

