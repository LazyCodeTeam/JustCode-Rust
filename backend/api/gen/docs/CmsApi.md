# \CmsApi

All URIs are relative to *https://ux55hiuu9e.execute-api.eu-central-1.amazonaws.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_content_assets_delete**](CmsApi.md#v1_content_assets_delete) | **Delete** /v1/content/assets | 
[**v1_content_assets_get**](CmsApi.md#v1_content_assets_get) | **Get** /v1/content/assets | 
[**v1_content_assets_upload_url_get**](CmsApi.md#v1_content_assets_upload_url_get) | **Get** /v1/content/assets/upload-url | 
[**v1_content_dry_run_put**](CmsApi.md#v1_content_dry_run_put) | **Put** /v1/content/dry-run | 
[**v1_content_put**](CmsApi.md#v1_content_put) | **Put** /v1/content | 



## v1_content_assets_delete

> v1_content_assets_delete(request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[just-code-dev-moderator-authorizer](../README.md#just-code-dev-moderator-authorizer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_content_assets_get

> Vec<crate::models::ContentAssetDto> v1_content_assets_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ContentAssetDto>**](ContentAssetDto.md)

### Authorization

[just-code-dev-moderator-authorizer](../README.md#just-code-dev-moderator-authorizer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_content_assets_upload_url_get

> Vec<crate::models::PresignedUrlDto> v1_content_assets_upload_url_get(count)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**u32**> |  |  |

### Return type

[**Vec<crate::models::PresignedUrlDto>**](PresignedUrlDto.md)

### Authorization

[just-code-dev-moderator-authorizer](../README.md#just-code-dev-moderator-authorizer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_content_dry_run_put

> v1_content_dry_run_put(expected_technology_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expected_technology_dto** | [**Vec<crate::models::ExpectedTechnologyDto>**](ExpectedTechnologyDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[just-code-dev-moderator-authorizer](../README.md#just-code-dev-moderator-authorizer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_content_put

> v1_content_put(expected_technology_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expected_technology_dto** | [**Vec<crate::models::ExpectedTechnologyDto>**](ExpectedTechnologyDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[just-code-dev-moderator-authorizer](../README.md#just-code-dev-moderator-authorizer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

