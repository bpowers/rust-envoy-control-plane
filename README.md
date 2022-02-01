`envoy-control-plane` for Rust
==============================

This package exposes the Envoy xDS protos and gRPC services via [prost](https://github.com/tokio-rs/prost).
It also enables reading JSON and YAML bootstrap files into prost-generated structs, e.g. if you have a `bootstrap.yaml` containing:

```yaml
---
admin:
  address:
    socketAddress:
      address: "127.0.0.1"
      portValue: 9901
node:
  id: envoy-test-1
  cluster: envoy-test-cluster-1
staticResources:
  listeners:
    - name: server-1
      address:
        socketAddress:
          address: 127.0.0.1
          portValue: 9000
      filterChains:
        - filters:
          - name: envoy.filters.network.http_connection_manager
            typedConfig:
              "@type": type.googleapis.com/envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager
              statPrefix: ingress_http
              httpFilters:
                - name: envoy.filters.http.router
              routeConfig:
                name: local_route
                virtualHosts:
                  - name: local_service
                    domains: ["*"]
                    routes:
                      - match:
                          prefix: "/"
                        route:
                          cluster: local-srv
          transportSocket:
            name: envoy.transport_sockets.tls
            typedConfig:
              '@type': type.googleapis.com/envoy.extensions.transport_sockets.tls.v3.DownstreamTlsContext
              requireClientCertificate:
                value: true
              commonTlsContext:
                tlsParams:
                  tlsMinimumProtocolVersion: TLSv1_3
                  tlsMaximumProtocolVersion: TLSv1_3
                validationContext:
                  trustedCa:
                    filename: ./certs/ca.crt
                  matchTypedSubjectAltNames:
                    - sanType: DNS
                      matcher:
                        exact: client.test
                tlsCertificates:
                  - certificateChain:
                      filename: ./certs/server.test.ecdsa-p256.crt
                    privateKey:
                      filename: ./certs/server.test.ecdsa-p256.key
  clusters:
    - name: local-srv
      type: STATIC
      lbPolicy: ROUND_ROBIN
      loadAssignment:
        clusterName: local-srv
        endpoints:
          - lbEndpoints:
            - endpoint:
                address:
                  socketAddress:
                    address: "127.0.0.1"
                    portValue: 9110
```

You can read that Bootstrap config in with:

```rust
use envoy_control_plane::envoy::config::bootstrap::v3::Bootstrap;

let config_contents = read_all(&args.config_path).await?;
let config_ext = args.config_path.extension().unwrap_or_default();
let bootstrap: Bootstrap = if config_ext == "yaml" || config_ext == "yml" {
    eprintln!("WARNING: YAML support is incomplete (e.g. durations don't work)");
    serde_yaml::from_str(&config_contents)?
} else {
    serde_json::from_str(&config_contents)?
};
```

Envoy uses a lot of protobuf `Any` values for polymorphism.   As you can see above we read them off disk just fine, but there is an additional hoop to jump through in order to actually access a typed instance:

```rust
let downstream_tls_context_type_url = DownstreamTlsContext::default().type_url();

// this works for the bootstrap config above, but real code wouldn't hardcode pulling
// the value out in such a fragile way.
let bootstrap_tls_config = bootstrap
    .static_resources
    .as_ref()
    .unwrap()
    .listeners[0]
    .filter_chains[0]
    .transport_socket
    .as_ref()
    .unwrap()
    .config_type
    .as_ref()
    .unwrap();
// there are not other `ConfigType` enum variants, so this let works fine.
let ConfigType::TypedConfig(tls_any) = bootstrap_tls_config;
// because the `TypedConfig` is `Any`, we need to check the type_url
if &tls_any.type_url == downstream_tls_context_type_url {
    let ctx = DownstreamTlsContext::decode(&*tls_any.value).unwrap();
    // store or do something with the `DownstreamTlsContext` instance
} else {
    panic!("unsupported typed config: {}", &tls_any.type_url);
}
```

## Caveats

* Envoy's JSON/YAML support allows both camelCase and snake_case for field names, but we only support snake case (e.g. `lbPolicy` and not `lb_policy`).
* We're using a [hacked-up fork of `pbjson`](https://github.com/bpowers/pbjson/tree/any-support) to correctly read in `Any` values from JSON/YAML.

## License

The code in this package is available under the Apache 2.0 license, as found in the LICENSE file.
Envoy itself (and its API protos) are also licensed under Apache 2.0.
