# Rust Echo

inspire by [Echo-Server](https://github.com/Ealenn/Echo-Server/)

Available:

![](https://img.shields.io/badge/linux-amd64-blue?style=flat-square&logo=docker)

- GET / POST / PUT / PATCH / DELETE
- Request (Query, Body, IPs, Host, Urls...)
- Request Headers / Response Headers
- Environment variables
- Control via Headers/Query
- Folders and Files
- Monitoring

## Configuration

| Environment        | Helm(not yet ready)          | Default |
|--------------------|------------------------------|---------|
| HTTP_PORT          | configmap.http.port          | `80`    |
| ENABLE_HOST        | configmap.enable.host        | `true`  |
| ENABLE_HTTP        | configmap.enable.http        | `true`  |
| ENABLE_REQUEST     | configmap.enable.request     | `true`  |
| ENABLE_COOKIES     | configmap.enable.cookies     | `true`  |
| ENABLE_HEADER      | configmap.enable.header      | `true`  |
| ENABLE_ENVIRONMENT | configmap.enable.environment | `true`  |
| ENABLE_FILE        | configmap.enable.file        | `true`  |

### Custom responses

| Query           | Header          | Content                         | Conditions                |
|-----------------|-----------------|---------------------------------|---------------------------|
| ?echo_code=     | X-ECHO-CODE     | HTTP code `200`, `404`          | 200 <= `CODE` <= 599      |
|                 |                 | `404-401` or `200-500-301`      |                           |
| ?echo_body=     | X-ECHO-BODY     | Body message                    |                           |
| ?echo_env_body= | X-ECHO-ENV-BODY | The key of environment variable | Enable environment `true` |
| ?echo_header=   | X-ECHO-HEADER   | Response Header `Lang: en-US`   | Enable header `true`      |
| ?echo_time=     | X-ECHO-TIME     | Wait time in `ms`               | 0 < `TIME` <= 30s         |
| ?echo_file=     | X-ECHO-FILE     | Path of Directory or File       | Enable file `true`        |

#### <a name='CustomHTTPStatusCode'></a>Custom HTTP Status Code

```bash
➜ curl -I --header 'X-ECHO-CODE: 404' localhost:8080
➜ curl -I localhost:8080/?echo_code=404

HTTP/1.1 404 Not Found
```

```bash
➜ curl -I --header 'X-ECHO-CODE: 404-300' localhost:8080
➜ curl -I localhost:8080/?echo_code=404-300

HTTP/1.1 404 Not Found
HTTP/1.1 300 Multiple Choices
```

```bash
➜ for i in {1..10}
➜ do
➜    curl -I localhost:8080/?echo_code=200-400-500
➜ done

HTTP/1.1 500 Internal Server Error
HTTP/1.1 400 Bad Request
HTTP/1.1 200 OK
HTTP/1.1 500 Internal Server Error
HTTP/1.1 200 OK
HTTP/1.1 500 Internal Server Error
```

#### <a name='CustomBody'></a>Custom Body

```bash
➜ curl --header 'X-ECHO-BODY: amazing' localhost:8080
➜ curl localhost:8080/?echo_body=amazing

"amazing"
```

#### <a name='CustomBodywithEnvironmentvariablevalue'></a>Custom Body with Environment variable value

```bash
➜ curl --header 'X-ECHO-ENV-BODY: HOSTNAME' localhost:8080
➜ curl localhost:8080/?echo_env_body=HOSTNAME

"c53a9ed79fa2"
```

```bash
➜ for i in {1..10}
➜ do
➜    curl localhost:8080/?echo_env_body=HOSTNAME
➜ done

"c53a9ed79fa2"
"f10c3af61e40"
"c53a9ed79fa2"
"f10c3af61e40"
"c53a9ed79fa2"
```

#### <a name='CustomHeaders'></a>Custom Headers

```bash
➜ curl --header 'X-ECHO-HEADER: One:1' localhost:8080
➜ curl localhost:8080/?echo_header=One:1

HTTP/1.1 200 OK
One: 1
```

```bash
➜ curl --header 'X-ECHO-HEADER: One:1, Two:2' localhost:8080
➜ curl "localhost:8080/?echo_header=One:1,%20Two:2"

HTTP/1.1 200 OK
One: 1
Two: 2
```

#### <a name='Customresponselatency'></a>Custom response latency

```bash
➜ curl --header 'X-ECHO-TIME: 5000' localhost:8080
➜ curl "localhost:8080/?echo_time=5000"

⏳... 5000 ms
```
