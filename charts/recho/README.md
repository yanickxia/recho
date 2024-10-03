# Rust Echo

inspire by [Echo-Server](https://github.com/Ealenn/Echo-Server/)

## Adding the Repository

```bash
helm repo add recho https://yanickxia.github.io/recho
helm repo update
```

## Deploy

```bash
helm upgrade -i ${name} ealenn/echo-server --namespace ${namespace} --force
```

## Values

| Key                                        | Type   | Default             | Description                                                    |
|--------------------------------------------|--------|---------------------|----------------------------------------------------------------|
| affinity                                   | object | `{}`                |                                                                |
| configmap.enable.cookies                   | bool   | `true`              | Enable cookies in response                                     |
| configmap.enable.environment               | bool   | `true`              | Enable environment in response                                 |
| configmap.enable.file                      | bool   | `true`              | Enable file in response                                        |
| configmap.enable.header                    | bool   | `true`              | Enable custom header in response                               |
| configmap.enable.host                      | bool   | `true`              | Enable host in response                                        |
| configmap.enable.http                      | bool   | `true`              | Enable http in response                                        |
| configmap.enable.request                   | bool   | `true`              | Enable request in response                                     |
| configmap.enable.https                     | bool   | `true`              | Enable https                                                   |
| configmap.enable.grpc                      | bool   | `true`              | Enable plaintext grpc                                          |
| configmap.enable.grpcTls                   | bool   | `true`              | Enable grpc over tls                                           |
| configmap.http.port                        | int    | 80                  | http port                                                      |
| configmap.https.port                       | int    | 443                 | https port                                                     |
| configmap.https.private_key_file           | strig  | config/key.pem      | ssl key                                                        |
| configmap.https.certificate_chain_file     | strig  | config/cert.pem     | ssl cert                                                       |
| configmap.grpc.port                        | int    | 5001                | plaintext grpc port                                            |
| configmap.grpc.tls.port                    | int    | 5002                |                                                                |
| configmap.grpc.tls.private_key_file        | strig  | config/key.pem      | ssl key                                                        |
| configmap.grpc.tls.certificate_chain_file  | strig  | config/cert.pem     | ssl cert                                                       |
| fullnameOverride                           | string | `""`                |                                                                |
| image.pullPolicy                           | string | `"IfNotPresent"`    |                                                                |
| image.repository                           | string | `"yanickxia/recho"` | https://hub.docker.com/r/yanickxia/recho                       |
| image.tag                                  | string | `"latest"`          | https://github.com/yanickxia/recho/releases                    |
| imagePullSecrets                           | list   | `[]`                |                                                                |
| ingress.annotations                        | object | `{}`                | Example `kubernetes.io/ingress.class: nginx` for Nginx Ingress |
| ingress.enabled                            | bool   | `false`             | Enable ingress                                                 |
| ingress.hosts[0].host                      | string | `"cluster.local"`   |                                                                |
| ingress.hosts[0].paths[0]                  | string | `"/"`               |                                                                |
| ingress.ingressClassName                   | string | `""`                |                                                                |
| ingress.tls                                | list   | `[]`                |                                                                |
| livenessProbe.failureThreshold             | int    | `3`                 |                                                                |
| livenessProbe.httpGet.httpHeaders[0].name  | string | `"x-echo-code"`     |                                                                |
| livenessProbe.httpGet.httpHeaders[0].value | string | `"200"`             |                                                                |
| livenessProbe.httpGet.path                 | string | `"/ping"`           |                                                                |
| livenessProbe.initialDelaySeconds          | int    | `5`                 |                                                                |
| livenessProbe.periodSeconds                | int    | `10`                |                                                                |
| livenessProbe.successThreshold             | int    | `1`                 |                                                                |
| livenessProbe.timeoutSeconds               | int    | `2`                 |                                                                |
| nameOverride                               | string | `""`                |                                                                |
| nodeSelector                               | object | `{}`                |                                                                |
| podSecurityContext                         | object | `{}`                |                                                                |
| replicaCount                               | int    | `1`                 | Pod replicas                                                   |
| resources.limits.cpu                       | string | `"50m"`             |                                                                |
| resources.limits.memory                    | string | `"128Mi"`           |                                                                |
| resources.requests.cpu                     | string | `"50m"`             |                                                                |
| resources.requests.memory                  | string | `"128Mi"`           |                                                                |
| securityContext                            | object | `{}`                |                                                                |
| service.port                               | int    | `80`                | For k8s >= 1.19 use port number not name                       |
| service.type                               | string | `"ClusterIP"`       |                                                                |
| serviceAccount.create                      | bool   | `true`              |                                                                |
| serviceAccount.name                        | string | `""`                |                                                                |
| tolerations                                | list   | `[]`                |                                                                |
