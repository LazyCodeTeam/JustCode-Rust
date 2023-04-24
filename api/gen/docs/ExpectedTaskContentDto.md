# ExpectedTaskContentDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | [**crate::models::TaskContentKindDto**](TaskContentKindDto.md) |  | 
**content** | **String** |  | 
**dynamic_content** | **::std::collections::HashMap<String, String>** |  | 
**variations** | [**::std::collections::HashMap<String, Vec<crate::models::PlaygroundVariationDto>>**](array.md) |  | 
**options** | [**Vec<crate::models::ExpectedOptionDto>**](ExpectedOptionDto.md) |  | 
**correct_option** | **u16** |  | 
**hints** | [**Vec<crate::models::HintDto>**](HintDto.md) |  | 
**correct_options** | **Vec<u16>** |  | 
**keywords** | [**Vec<crate::models::ExpectedKeywordDto>**](ExpectedKeywordDto.md) |  | 
**correct_order** | **Vec<u16>** |  | 
**correct_code** | **::std::collections::HashMap<String, String>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


