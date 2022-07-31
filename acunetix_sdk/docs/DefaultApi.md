# \DefaultApi

All URIs are relative to *https://127.0.0.1:13443/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_target**](DefaultApi.md#add_target) | **POST** /targets | Add a new target to the scan list
[**delete_scan**](DefaultApi.md#delete_scan) | **DELETE** /scans/{scanid} | delete scan by scanid
[**delete_target**](DefaultApi.md#delete_target) | **DELETE** /targets/{targetid} | get target by id
[**get_info**](DefaultApi.md#get_info) | **GET** /info | get awvs info
[**get_me**](DefaultApi.md#get_me) | **GET** /me | get user info
[**get_scan_detail**](DefaultApi.md#get_scan_detail) | **GET** /scans/{scanid} | get scan status by scanid
[**get_scan_profiles**](DefaultApi.md#get_scan_profiles) | **GET** /scanning_profiles | get scan profile
[**get_scan_reports**](DefaultApi.md#get_scan_reports) | **GET** /reports/{scanid} | get scan reports by scanid
[**get_scan_stat**](DefaultApi.md#get_scan_stat) | **GET** /scans/{scanid}/results/{sessionid}/statistics | get stat by scanid,sessionid
[**get_scans**](DefaultApi.md#get_scans) | **GET** /scans | get scan list
[**get_target**](DefaultApi.md#get_target) | **GET** /targets/{targetid} | get target by id
[**get_target_config**](DefaultApi.md#get_target_config) | **GET** /targets/{targetid}/configuration | get target by id
[**get_targets**](DefaultApi.md#get_targets) | **GET** /targets | get all targets
[**get_vuln**](DefaultApi.md#get_vuln) | **GET** /scans/{scanid}/results/{sessionid}/vulnerabilities | get results by scanid,sessionid
[**login**](DefaultApi.md#login) | **POST** /me/login | login
[**set_target_config**](DefaultApi.md#set_target_config) | **PATCH** /targets/{targetid}/configuration | get target by id
[**start_scan**](DefaultApi.md#start_scan) | **POST** /scans | start scan by scanid
[**stop_scan**](DefaultApi.md#stop_scan) | **POST** /scans/{scanid}/abort | stop scan by scanid



## add_target

> crate::models::TargetResp add_target(target)
Add a new target to the scan list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | [**Target**](Target.md) | target parameter | [required] |

### Return type

[**crate::models::TargetResp**](TargetResp.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_scan

> delete_scan(scanid)
delete scan by scanid

https://github.com/bit4woo/Ashe/blob/master/lib/WVS11.py

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scanid** | **String** | scan task id | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_target

> crate::models::TargetDel delete_target(targetid)
get target by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**targetid** | **String** | target id | [required] |

### Return type

[**crate::models::TargetDel**](TargetDel.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_info

> crate::models::Info get_info()
get awvs info

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Info**](Info.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_me

> crate::models::Me get_me()
get user info

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Me**](Me.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scan_detail

> crate::models::ScanDetail get_scan_detail(scanid)
get scan status by scanid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scanid** | **String** | scan task id | [required] |

### Return type

[**crate::models::ScanDetail**](ScanDetail.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scan_profiles

> crate::models::Profiles get_scan_profiles()
get scan profile

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Profiles**](Profiles.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scan_reports

> get_scan_reports(scanid)
get scan reports by scanid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scanid** | **String** | scan task id | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scan_stat

> crate::models::ScanStat get_scan_stat(scanid, sessionid)
get stat by scanid,sessionid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scanid** | **String** | scan task id | [required] |
**sessionid** | **String** | scan session id | [required] |

### Return type

[**crate::models::ScanStat**](ScanStat.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scans

> crate::models::Scans get_scans()
get scan list

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Scans**](Scans.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_target

> crate::models::TargetDetail get_target(targetid)
get target by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**targetid** | **String** | target id | [required] |

### Return type

[**crate::models::TargetDetail**](TargetDetail.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_target_config

> crate::models::TargetDetail get_target_config(targetid)
get target by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**targetid** | **String** | target id | [required] |

### Return type

[**crate::models::TargetDetail**](TargetDetail.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_targets

> crate::models::TargetsResp get_targets()
get all targets

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TargetsResp**](TargetsResp.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vuln

> get_vuln(scanid, sessionid)
get results by scanid,sessionid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scanid** | **String** | scan task id | [required] |
**sessionid** | **String** | scan session id | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login

> login(login_req)
login

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_req** | [**LoginReq**](LoginReq.md) | login parameter | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_target_config

> set_target_config(targetid, target_config)
get target by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**targetid** | **String** | target id | [required] |
**target_config** | Option<[**TargetConfig**](TargetConfig.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_scan

> start_scan(scan)
start scan by scanid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scan** | [**Scan**](Scan.md) | target response | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_scan

> stop_scan(scanid)
stop scan by scanid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scanid** | **String** | scan task id | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

