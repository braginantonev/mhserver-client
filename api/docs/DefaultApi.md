# \DefaultApi

All URIs are relative to *https://my.best.server/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**files_create_connection**](DefaultApi.md#files_create_connection) | **POST** /files/connect | Получение параметров получения или отправки чанков
[**files_create_connection_0**](DefaultApi.md#files_create_connection_0) | **POST** /files/connect | Получение параметров получения или отправки чанков
[**files_get_available_space**](DefaultApi.md#files_get_available_space) | **GET** /files/space | Получить количество свободного места
[**files_get_available_space_0**](DefaultApi.md#files_get_available_space_0) | **GET** /files/space | Получить количество свободного места
[**files_get_chunk**](DefaultApi.md#files_get_chunk) | **GET** /files/get | Получить часть файла
[**files_get_chunk_0**](DefaultApi.md#files_get_chunk_0) | **GET** /files/get | Получить часть файла
[**files_get_sum**](DefaultApi.md#files_get_sum) | **GET** /files/sum | Получить контрольную сумму части файла
[**files_get_sum_0**](DefaultApi.md#files_get_sum_0) | **GET** /files/sum | Получить контрольную сумму части файла
[**files_make_directory**](DefaultApi.md#files_make_directory) | **POST** /files/mkdir | Создать каталог
[**files_make_directory_0**](DefaultApi.md#files_make_directory_0) | **POST** /files/mkdir | Создать каталог
[**files_remove_directory**](DefaultApi.md#files_remove_directory) | **POST** /files/rmdir | Удалить каталог
[**files_remove_directory_0**](DefaultApi.md#files_remove_directory_0) | **POST** /files/rmdir | Удалить каталог
[**files_save_chunk**](DefaultApi.md#files_save_chunk) | **POST** /files/save | Сохранить часть файла
[**files_save_chunk_0**](DefaultApi.md#files_save_chunk_0) | **POST** /files/save | Сохранить часть файла
[**get_files_list**](DefaultApi.md#get_files_list) | **GET** /files | Получить список файлов в каталоге
[**get_files_list_0**](DefaultApi.md#get_files_list_0) | **GET** /files | Получить список файлов в каталоге
[**ping**](DefaultApi.md#ping) | **POST** /tools/ping | Приветствие
[**users_login**](DefaultApi.md#users_login) | **POST** /users/login | Получение JWT пользователя
[**users_login_0**](DefaultApi.md#users_login_0) | **POST** /users/login | Получение JWT пользователя
[**users_register**](DefaultApi.md#users_register) | **POST** /users/register | Регистрация пользователя
[**users_register_0**](DefaultApi.md#users_register_0) | **POST** /users/register | Регистрация пользователя



## files_create_connection

> models::ConnectionResponse files_create_connection(connection_request, mode)
Получение параметров получения или отправки чанков

\"Пропуск\" к работе с файлами на сервере. Создаёт инструкцию для сервера, что конкретный файл будет использоваться (создание, изменение и т.д.). Без этой инструкции, работа с файлами не допускается.  Также передаёт пользователю информацию о количестве чанков и их размере, необходимые для сохранения файла.  Соединение существует 5 минут, либо пока файл не будет полностью сохранён (очевидно, при сохранении файла) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_request** | [**ConnectionRequest**](ConnectionRequest.md) |  | [required] |
**mode** | [**ConnectionMode**](ConnectionMode.md) |  | [required] |

### Return type

[**models::ConnectionResponse**](ConnectionResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_create_connection_0

> models::ConnectionResponse files_create_connection_0(connection_request, mode)
Получение параметров получения или отправки чанков

\"Пропуск\" к работе с файлами на сервере. Создаёт инструкцию для сервера, что конкретный файл будет использоваться (создание, изменение и т.д.). Без этой инструкции, работа с файлами не допускается.  Также передаёт пользователю информацию о количестве чанков и их размере, необходимые для сохранения файла.  Соединение существует 5 минут, либо пока файл не будет полностью сохранён (очевидно, при сохранении файла) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_request** | [**ConnectionRequest**](ConnectionRequest.md) |  | [required] |
**mode** | [**ConnectionMode**](ConnectionMode.md) |  | [required] |

### Return type

[**models::ConnectionResponse**](ConnectionResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_get_available_space

> i32 files_get_available_space()
Получить количество свободного места

### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_get_available_space_0

> i32 files_get_available_space_0()
Получить количество свободного места

### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_get_chunk

> String files_get_chunk(conn_id, chunk_id)
Получить часть файла

Запрос возвращает файл по чанкам.   Чтобы получить полный файл, необходимо отправить столько запросов, сколько записано в поле `chunksCount` при создании соединения 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conn_id** | **uuid::Uuid** | UUID соединения, созданного с помощью `/files/connect` | [required] |
**chunk_id** | **i32** | Индекс чанка файла, контрольную сумму нужно вернуть | [required] |

### Return type

**String**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_get_chunk_0

> String files_get_chunk_0(conn_id, chunk_id)
Получить часть файла

Запрос возвращает файл по чанкам.   Чтобы получить полный файл, необходимо отправить столько запросов, сколько записано в поле `chunksCount` при создании соединения 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conn_id** | **uuid::Uuid** | UUID соединения, созданного с помощью `/files/connect` | [required] |
**chunk_id** | **i32** | Индекс чанка файла, контрольную сумму нужно вернуть | [required] |

### Return type

**String**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_get_sum

> Vec<i32> files_get_sum(conn_id, chunk_id)
Получить контрольную сумму части файла

Используется проверки целлостности сохранённого файла.   Возвращает контрольную сумму чанка (не файла!),  поэтому, чтобы проверить целостность файла, необходимо сделать столько запросов, сколько указано в поле `chunksCount` при создании соединения.  Контрольная сумма считается по алгоритму SHA-256. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conn_id** | **uuid::Uuid** | UUID соединения, созданного с помощью `/files/connect` | [required] |
**chunk_id** | **i32** | Индекс чанка файла, контрольную сумму нужно вернуть | [required] |

### Return type

**Vec<i32>**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_get_sum_0

> Vec<i32> files_get_sum_0(conn_id, chunk_id)
Получить контрольную сумму части файла

Используется проверки целлостности сохранённого файла.   Возвращает контрольную сумму чанка (не файла!),  поэтому, чтобы проверить целостность файла, необходимо сделать столько запросов, сколько указано в поле `chunksCount` при создании соединения.  Контрольная сумма считается по алгоритму SHA-256. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conn_id** | **uuid::Uuid** | UUID соединения, созданного с помощью `/files/connect` | [required] |
**chunk_id** | **i32** | Индекс чанка файла, контрольную сумму нужно вернуть | [required] |

### Return type

**Vec<i32>**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_make_directory

> files_make_directory(dir)
Создать каталог

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dir** | **String** | Каталог с файлами на сервере.  Параметр всегда должен начинаться с корневого каталога: `/`  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_make_directory_0

> files_make_directory_0(dir)
Создать каталог

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dir** | **String** | Каталог с файлами на сервере.  Параметр всегда должен начинаться с корневого каталога: `/`  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_remove_directory

> files_remove_directory(dir)
Удалить каталог

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dir** | **String** | Каталог с файлами на сервере.  Параметр всегда должен начинаться с корневого каталога: `/`  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_remove_directory_0

> files_remove_directory_0(dir)
Удалить каталог

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dir** | **String** | Каталог с файлами на сервере.  Параметр всегда должен начинаться с корневого каталога: `/`  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_save_chunk

> files_save_chunk(save_chunk, conn_id)
Сохранить часть файла

Сохраняет часть файла (чанк) на сервере.   Чтобы сохранить полный файл, необходимо отправить запрос столько раз, сколько указано в поле `chunksCount` созданного соединения. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**save_chunk** | [**SaveChunk**](SaveChunk.md) |  | [required] |
**conn_id** | **uuid::Uuid** | UUID соединения, созданного с помощью `/files/connect` | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_save_chunk_0

> files_save_chunk_0(save_chunk, conn_id)
Сохранить часть файла

Сохраняет часть файла (чанк) на сервере.   Чтобы сохранить полный файл, необходимо отправить запрос столько раз, сколько указано в поле `chunksCount` созданного соединения. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**save_chunk** | [**SaveChunk**](SaveChunk.md) |  | [required] |
**conn_id** | **uuid::Uuid** | UUID соединения, созданного с помощью `/files/connect` | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_list

> Vec<models::FilesListInner> get_files_list(dir)
Получить список файлов в каталоге

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dir** | **String** | Каталог с файлами на сервере.  Параметр всегда должен начинаться с корневого каталога: `/`  | [required] |

### Return type

[**Vec<models::FilesListInner>**](FilesList_inner.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_list_0

> Vec<models::FilesListInner> get_files_list_0(dir)
Получить список файлов в каталоге

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dir** | **String** | Каталог с файлами на сервере.  Параметр всегда должен начинаться с корневого каталога: `/`  | [required] |

### Return type

[**Vec<models::FilesListInner>**](FilesList_inner.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ping

> String ping()
Приветствие

Используется для пинга сервера

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_login

> String users_login(user_login_request)
Получение JWT пользователя

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_login_request** | [**UserLoginRequest**](UserLoginRequest.md) |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_login_0

> String users_login_0(user_login_request)
Получение JWT пользователя

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_login_request** | [**UserLoginRequest**](UserLoginRequest.md) |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_register

> users_register(user_register_request)
Регистрация пользователя

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_register_request** | [**UserRegisterRequest**](UserRegisterRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_register_0

> users_register_0(user_register_request)
Регистрация пользователя

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_register_request** | [**UserRegisterRequest**](UserRegisterRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

