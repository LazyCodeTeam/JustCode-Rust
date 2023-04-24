# \ProfileApi

All URIs are relative to *http://$*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_profile_current_avatar_upload_url_get**](ProfileApi.md#v1_profile_current_avatar_upload_url_get) | **Get** /v1/profile/current/avatar/upload-url | 
[**v1_profile_current_delete**](ProfileApi.md#v1_profile_current_delete) | **Delete** /v1/profile/current | 
[**v1_profile_current_get**](ProfileApi.md#v1_profile_current_get) | **Get** /v1/profile/current | 
[**v1_profile_current_push_delete**](ProfileApi.md#v1_profile_current_push_delete) | **Delete** /v1/profile/current/push | 
[**v1_profile_current_push_put**](ProfileApi.md#v1_profile_current_push_put) | **Put** /v1/profile/current/push | 
[**v1_profile_current_put**](ProfileApi.md#v1_profile_current_put) | **Put** /v1/profile/current | 



## v1_profile_current_avatar_upload_url_get

> crate::models::PresignedUrlDto v1_profile_current_avatar_upload_url_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PresignedUrlDto**](PresignedUrlDto.md)

### Authorization

[just-code-dev-auth](../README.md#just-code-dev-auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_profile_current_delete

> v1_profile_current_delete()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[just-code-dev-auth](../README.md#just-code-dev-auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_profile_current_get

> crate::models::ProfileDto v1_profile_current_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ProfileDto**](ProfileDto.md)

### Authorization

[just-code-dev-auth](../README.md#just-code-dev-auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_profile_current_push_delete

> v1_profile_current_push_delete()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[just-code-dev-auth](../README.md#just-code-dev-auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_profile_current_push_put

> v1_profile_current_push_put(push_data_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**push_data_dto** | [**PushDataDto**](PushDataDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[just-code-dev-auth](../README.md#just-code-dev-auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_profile_current_put

> v1_profile_current_put(update_profile_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_profile_dto** | [**UpdateProfileDto**](UpdateProfileDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[just-code-dev-auth](../README.md#just-code-dev-auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

