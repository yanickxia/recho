# Rust Echo

inspire by [Echo-Server](https://github.com/Ealenn/Echo-Server/)

Available:

![](https://img.shields.io/badge/linux-amd64-blue?style=flat-square&logo=docker)
[![Artifact Hub](https://img.shields.io/endpoint?url=https://artifacthub.io/badge/repository/recho)](https://artifacthub.io/packages/search?repo=recho)

- GET / POST / PUT / PATCH / DELETE
- Request (Query, Body, IPs, Host, Urls...)
- Request Headers / Response Headers
- Environment variables
- Control via Headers/Query
- Folders and Files
- Monitoring

## Configuration

**HTTP & Metrics Server Always Enabled**

| Environment                     | Helm                                   | Default           | Desc                                |
|---------------------------------|----------------------------------------|-------------------|-------------------------------------|
| HTTP_PORT                       | configmap.http.port                    | `80`              | http listen port                    |
| HTTP_METRICS                    | configmap.metrics.port                 | `9091`            | http metrics port                   |
| HTTPS_PORT                      | configmap.https.port                   | `443`             | https listen port                   |
| HTTPS_PRIVATE_KEY_FILE          | configmap.https.private_key_file       | `config/key.pem`  | https tls private key file          |
| HTTPS_CERTIFICATE_CHAIN_FILE    | configmap.https.certificate_chain_file | `config/cert.pem` | https tls certificate file          |
| GRPC_PORT                       | configmap.grpc.port                    | `5001`            | grpc plaintext port                 |
| GRPC_TLS_PORT                   | configmap.grpc.TLS.port                | `5002`            | grpc tls port                       |
| GRPC_TLS_PRIVATE_KEY_FILE       | configmap.grpc.TLS.port                | `config/key.pem`  | grpc tls private key file           |
| GRPC_TLS_CERTIFICATE_CHAIN_FILE | configmap.grpc.TLS.port                | `config/cert.pem` | grpc tls certificate file           |
| Module Enable                   |                                        |                   |                                     |
| ENABLE_HOST                     | configmap.enable.host                  | `true`            | http(s)/grpc(tls): return Host info |
| ENABLE_HTTP                     | configmap.enable.http                  | `true`            | http(s): return http info           |
| ENABLE_REQUEST                  | configmap.enable.request               | `true`            | http(s): return request info        |
| ENABLE_HEADER                   | configmap.enable.header                | `true`            | http(s): return headers             |
| ENABLE_ENVIRONMENT              | configmap.enable.environment           | `true`            | http(s)/grpc(tls): return env info  |
| ENABLE_FILE                     | configmap.enable.file                  | `true`            | http(s): support return file        |
| ENABLE_HTTPS                    | configmap.enable.https                 | `true`            | enable https server                 |
| ENABLE_GRPC_TLS                 | configmap.enable.grpc_tls              | `true`            | enable grpc plaintext server        |
| ENABLE_GRPC                     | configmap.enable.grpc                  | `true`            | enable grpc tls server              |

### Custom responses

| Query           | Header          | Content                                              | Conditions                |
|-----------------|-----------------|------------------------------------------------------|---------------------------|
| ?echo_code=     | X-ECHO-CODE     | HTTP code `200`, `404`   ,`404-401` or `200-500-301` | 200 <= `CODE` <= 599      |
| ?echo_body=     | X-ECHO-BODY     | Body message                                         |                           |
| ?echo_env_body= | X-ECHO-ENV-BODY | The key of environment variable                      | Enable environment `true` |
| ?echo_header=   | X-ECHO-HEADER   | Response Header `Lang: en-US`                        | Enable header `true`      |
| ?echo_time=     | X-ECHO-TIME     | Wait time in `ms`                                    | 0 < `TIME` <= 30s         |
| ?echo_file=     | X-ECHO-FILE     | Path of Directory or File                            | Enable file `true`        |

#### Custom HTTP Status Code

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

#### Custom Body

```bash
➜ curl --header 'X-ECHO-BODY: amazing' localhost:8080
➜ curl localhost:8080/?echo_body=amazing

"amazing"
```

#### Custom Body with Environment variable value

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

#### Custom Headers

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

#### Custom response latency

```bash
➜ curl --header 'X-ECHO-TIME: 5000' localhost:8080
➜ curl "localhost:8080/?echo_time=5000"

⏳... 5000 ms
```

## GRPC

### ProtoBuf

| Field   | Desc                    |
|---------|-------------------------|
| message | relay same message      |
| delay   | how time delay to relay |

### plaintext

```bash
➜ grpcurl -plaintext -use-reflection  -d '{"message": "1234"}' localhost:5001 echo.Echo/Echo
{
  "message": "1234"
}
```

### TLS

```bash
➜ grpcurl -insecure -use-reflection  -d '{"message": "1234", "delay": 1000 }' localhost:5001 echo.Echo/Echo # delay after 1s
{
  "message": "1234"
}
```

## Metrics

default metrics port `9091`, path `/metrics`

```bash
➜ curl localhost:9091/metrics
# HELP http_requests Number of HTTP requests received.
# TYPE http_requests counter
http_requests_total{method="GET",path="/foo",protocol="http",status_code="200"} 1
http_requests_total{method="GET",path="/foo",protocol="https",status_code="200"} 2
http_requests_total{method="GET",path="/bar",protocol="http",status_code="200"} 1
http_requests_total{method="GET",path="/bar",protocol="https",status_code="200"} 2
# HELP http_requests Number of HTTP requests received.
# TYPE http_requests counter
# HELP grpc_requests Number of GRPC requests received.
# TYPE grpc_requests counter
grpc_requests_total{method="/echo.Echo/Echo",protocol="tls"} 4
grpc_requests_total{method="/grpc.reflection.v1alpha.ServerReflection/ServerReflectionInfo",protocol="plaintext"} 3
grpc_requests_total{method="/grpc.reflection.v1alpha.ServerReflection/ServerReflectionInfo",protocol="tls"} 4
grpc_requests_total{method="/echo.Echo/Echo",protocol="plaintext"} 3
```

## Setting up

### Docker

```bash
docker run -p 8080:80 yanickxia/recho
```

### Docker-Compose

**Sample**

```yaml
services:
    recho:
        image: yanickxia/recho
        environment:
            HTTP_PORT: 8080
        ports:
            - 8080:8080
```

### Kubernetes

```bash
curl -sL https://raw.githubusercontent.com/yanickxia/recho/master/docs/examples/kube.yaml | kubectl apply -f -
```

### Kubernetes with Helm

<div class="artifacthub-widget" data-url="https://artifacthub.io/packages/helm/recho/recho" data-theme="light" data-header="true" data-stars="true" data-responsive="false"><blockquote><p lang="en" dir="ltr"><b>recho</b>: echo service</p>&mdash; Open in <a href="https://artifacthub.io/packages/helm/recho/recho">Artifact Hub</a></blockquote></div>

```bash
helm repo add recho https://yanickxia.github.io/recho/
helm repo update
helm install my-recho recho/recho
```
