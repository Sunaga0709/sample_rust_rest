# user_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**users-by-id-v1-get**](user_api.md#users-by-id-v1-get) | **GET** /v1/users/{user_id} | ユーザー詳細取得API
**users-v1-get**](user_api.md#users-v1-get) | **GET** /v1/users | ユーザー一覧取得API


# **users-by-id-v1-get**
> models::UsersV1Get200ResponseInner users-by-id-v1-get(user_id)
ユーザー詳細取得API

詳細を取得する

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **user_id** | **String**|  | 

### Return type

[**models::UsersV1Get200ResponseInner**](users_v1_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, applicatoin/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **users-v1-get**
> Vec<models::UsersV1Get200ResponseInner> users-v1-get()
ユーザー一覧取得API

ユーザー一覧取得する

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Vec<models::UsersV1Get200ResponseInner>**](users_v1_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, applicatoin/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

