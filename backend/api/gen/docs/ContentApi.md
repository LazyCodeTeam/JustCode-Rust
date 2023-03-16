# \ContentApi

All URIs are relative to *https://ux55hiuu9e.execute-api.eu-central-1.amazonaws.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_content_load_put**](ContentApi.md#v1_content_load_put) | **Put** /v1/content/load | 
[**v1_content_public_section_section_id_tasks_get**](ContentApi.md#v1_content_public_section_section_id_tasks_get) | **Get** /v1/content/public/section/{section_id}/tasks | 
[**v1_content_public_technologies_get**](ContentApi.md#v1_content_public_technologies_get) | **Get** /v1/content/public/technologies | 
[**v1_content_public_technology_technology_id_sections_get**](ContentApi.md#v1_content_public_technology_technology_id_sections_get) | **Get** /v1/content/public/technology/{technology_id}/sections | 



## v1_content_load_put

> v1_content_load_put(expected_technology_dto)


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


## v1_content_public_section_section_id_tasks_get

> Vec<crate::models::PublicTaskDto> v1_content_public_section_section_id_tasks_get(section_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | Generated path parameter for section_id | [required] |

### Return type

[**Vec<crate::models::PublicTaskDto>**](PublicTaskDto.md)

### Authorization

[just-code-dev-app-authorizer](../README.md#just-code-dev-app-authorizer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_content_public_technologies_get

> Vec<crate::models::TechnologyDto> v1_content_public_technologies_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::TechnologyDto>**](TechnologyDto.md)

### Authorization

[just-code-dev-app-authorizer](../README.md#just-code-dev-app-authorizer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_content_public_technology_technology_id_sections_get

> Vec<crate::models::SectionDto> v1_content_public_technology_technology_id_sections_get(technology_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**technology_id** | **String** | Generated path parameter for technology_id | [required] |

### Return type

[**Vec<crate::models::SectionDto>**](SectionDto.md)

### Authorization

[just-code-dev-app-authorizer](../README.md#just-code-dev-app-authorizer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
