# \DefaultApi

All URIs are relative to *https://my.best.server/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**files_connect_post**](DefaultApi.md#files_connect_post) | **POST** /files/connect | Получение параметров получения или отправки чанков
[**files_connect_post_0**](DefaultApi.md#files_connect_post_0) | **POST** /files/connect | Получение параметров получения или отправки чанков
[**files_get**](DefaultApi.md#files_get) | **GET** /files | Получить список файлов в каталоге
[**files_get_0**](DefaultApi.md#files_get_0) | **GET** /files | Получить список файлов в каталоге
[**files_get_get**](DefaultApi.md#files_get_get) | **GET** /files/get | Получить часть файла
[**files_get_get_0**](DefaultApi.md#files_get_get_0) | **GET** /files/get | Получить часть файла
[**files_mkdir_post**](DefaultApi.md#files_mkdir_post) | **POST** /files/mkdir | Создать каталог
[**files_mkdir_post_0**](DefaultApi.md#files_mkdir_post_0) | **POST** /files/mkdir | Создать каталог
[**files_rmdir_post**](DefaultApi.md#files_rmdir_post) | **POST** /files/rmdir | Удалить каталог
[**files_rmdir_post_0**](DefaultApi.md#files_rmdir_post_0) | **POST** /files/rmdir | Удалить каталог
[**files_save_post**](DefaultApi.md#files_save_post) | **POST** /files/save | Сохранить часть файла
[**files_save_post_0**](DefaultApi.md#files_save_post_0) | **POST** /files/save | Сохранить часть файла
[**files_space_get**](DefaultApi.md#files_space_get) | **GET** /files/space | Получить количество свободного места
[**files_space_get_0**](DefaultApi.md#files_space_get_0) | **GET** /files/space | Получить количество свободного места
[**files_sum_get**](DefaultApi.md#files_sum_get) | **GET** /files/sum | Получить контрольную сумму части файла
[**files_sum_get_0**](DefaultApi.md#files_sum_get_0) | **GET** /files/sum | Получить контрольную сумму части файла
[**users_login_post**](DefaultApi.md#users_login_post) | **POST** /users/login | Получение JWT пользователя
[**users_login_post_0**](DefaultApi.md#users_login_post_0) | **POST** /users/login | Получение JWT пользователя
[**users_register_post**](DefaultApi.md#users_register_post) | **POST** /users/register | Регистрация пользователя
[**users_register_post_0**](DefaultApi.md#users_register_post_0) | **POST** /users/register | Регистрация пользователя



## files_connect_post

> models::ConnectionResponse files_connect_post(connection_request, mode)
Получение параметров получения или отправки чанков

\"Пропуск\" к работе с файлами на сервере. Создаёт инструкцию для сервера, что конкретный файл будет использоваться (создание, изменение и т.д.). Без этой инструкции, работа с файлами не допускается.  Также передаёт пользователю информацию о количестве чанков и их размере, необходимые для сохранения файла.  Соединение существует 5 минут, либо пока файл не будет полностью сохранён (очевидно, при сохранении файла) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_request** | [**ConnectionRequest**](ConnectionRequest.md) |  | [required] |
**mode** | **String** | Тип подключения.  `RDWR` - подключение для создания, редактирования и чтения файлов. При использовании - обязательно указывать размер файла в теле запроса. `RDONLY` - подключение только для чтения фаилов. При использовании - указывать размер файла не обязательно.  | [required] |

### Return type

[**models::ConnectionResponse**](ConnectionResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_connect_post_0

> models::ConnectionResponse files_connect_post_0(connection_request, mode)
Получение параметров получения или отправки чанков

\"Пропуск\" к работе с файлами на сервере. Создаёт инструкцию для сервера, что конкретный файл будет использоваться (создание, изменение и т.д.). Без этой инструкции, работа с файлами не допускается.  Также передаёт пользователю информацию о количестве чанков и их размере, необходимые для сохранения файла.  Соединение существует 5 минут, либо пока файл не будет полностью сохранён (очевидно, при сохранении файла) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_request** | [**ConnectionRequest**](ConnectionRequest.md) |  | [required] |
**mode** | **String** | Тип подключения.  `RDWR` - подключение для создания, редактирования и чтения файлов. При использовании - обязательно указывать размер файла в теле запроса. `RDONLY` - подключение только для чтения фаилов. При использовании - указывать размер файла не обязательно.  | [required] |

### Return type

[**models::ConnectionResponse**](ConnectionResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_get

> Vec<models::FilesListInner> files_get(dir)
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
- **Accept**: application/json, plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_get_0

> Vec<models::FilesListInner> files_get_0(dir)
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
- **Accept**: application/json, plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_get_get

> Vec<i32> files_get_get(conn_id, chunk_id)
Получить часть файла

Запрос возвращает файл по чанкам.   Чтобы получить полный файл, необходимо отправить столько запросов, сколько записано в поле `chunksCount` при создании соединения 

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
- **Accept**: application/octet-stream, plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_get_get_0

> Vec<i32> files_get_get_0(conn_id, chunk_id)
Получить часть файла

Запрос возвращает файл по чанкам.   Чтобы получить полный файл, необходимо отправить столько запросов, сколько записано в поле `chunksCount` при создании соединения 

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
- **Accept**: application/octet-stream, plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_mkdir_post

> files_mkdir_post(dir)
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
- **Accept**: plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_mkdir_post_0

> files_mkdir_post_0(dir)
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
- **Accept**: plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_rmdir_post

> files_rmdir_post(dir)
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
- **Accept**: plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_rmdir_post_0

> files_rmdir_post_0(dir)
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
- **Accept**: plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_save_post

> files_save_post(save_chunk, conn_id)
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
- **Accept**: plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_save_post_0

> files_save_post_0(save_chunk, conn_id)
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
- **Accept**: plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_space_get

> i32 files_space_get()
Получить количество свободного места

### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_space_get_0

> i32 files_space_get_0()
Получить количество свободного места

### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_sum_get

> Vec<i32> files_sum_get(conn_id, chunk_id)
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
- **Accept**: application/octet-stream, plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_sum_get_0

> Vec<i32> files_sum_get_0(conn_id, chunk_id)
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
- **Accept**: application/octet-stream, plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_login_post

> String users_login_post(user_login_request)
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
- **Accept**: plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_login_post_0

> String users_login_post_0(user_login_request)
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
- **Accept**: plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_register_post

> users_register_post(user_register_request)
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
- **Accept**: plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_register_post_0

> users_register_post_0(user_register_request)
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
- **Accept**: plain/text

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

