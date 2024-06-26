// Copyright 2021 The rust-control-plane Authors. All rights reserved.
// Use of this source code is governed by the Apache License,
// Version 2.0, that can be found in the LICENSE file.

use std::env;
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(descriptor_path.clone())
        .compile_well_known_types(true)
        .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
        .extern_path(".google.protobuf", "::pbjson_types")
        .compile(
            &[
                "data-plane-api/envoy/admin/v3/certs.proto",
                "data-plane-api/envoy/admin/v3/clusters.proto",
                "data-plane-api/envoy/admin/v3/config_dump.proto",
                "data-plane-api/envoy/admin/v3/init_dump.proto",
                "data-plane-api/envoy/admin/v3/listeners.proto",
                "data-plane-api/envoy/admin/v3/memory.proto",
                "data-plane-api/envoy/admin/v3/metrics.proto",
                "data-plane-api/envoy/admin/v3/mutex_stats.proto",
                "data-plane-api/envoy/admin/v3/server_info.proto",
                "data-plane-api/envoy/admin/v3/tap.proto",
                "data-plane-api/envoy/config/accesslog/v3/accesslog.proto",
                "data-plane-api/envoy/config/bootstrap/v3/bootstrap.proto",
                "data-plane-api/envoy/config/cluster/v3/circuit_breaker.proto",
                "data-plane-api/envoy/config/cluster/v3/cluster.proto",
                "data-plane-api/envoy/config/cluster/v3/filter.proto",
                "data-plane-api/envoy/config/cluster/v3/outlier_detection.proto",
                "data-plane-api/envoy/config/common/key_value/v3/config.proto",
                "data-plane-api/envoy/config/common/matcher/v3/matcher.proto",
                "data-plane-api/envoy/config/common/mutation_rules/v3/mutation_rules.proto",
                "data-plane-api/envoy/config/core/v3/address.proto",
                "data-plane-api/envoy/config/core/v3/backoff.proto",
                "data-plane-api/envoy/config/core/v3/base.proto",
                "data-plane-api/envoy/config/core/v3/config_source.proto",
                "data-plane-api/envoy/config/core/v3/event_service_config.proto",
                "data-plane-api/envoy/config/core/v3/extension.proto",
                "data-plane-api/envoy/config/core/v3/grpc_method_list.proto",
                "data-plane-api/envoy/config/core/v3/grpc_service.proto",
                "data-plane-api/envoy/config/core/v3/health_check.proto",
                "data-plane-api/envoy/config/core/v3/http_uri.proto",
                "data-plane-api/envoy/config/core/v3/protocol.proto",
                "data-plane-api/envoy/config/core/v3/proxy_protocol.proto",
                "data-plane-api/envoy/config/core/v3/resolver.proto",
                "data-plane-api/envoy/config/core/v3/socket_option.proto",
                "data-plane-api/envoy/config/core/v3/substitution_format_string.proto",
                "data-plane-api/envoy/config/core/v3/udp_socket_config.proto",
                "data-plane-api/envoy/config/endpoint/v3/endpoint.proto",
                "data-plane-api/envoy/config/endpoint/v3/endpoint_components.proto",
                "data-plane-api/envoy/config/endpoint/v3/load_report.proto",
                "data-plane-api/envoy/config/grpc_credential/v3/aws_iam.proto",
                "data-plane-api/envoy/config/grpc_credential/v3/file_based_metadata.proto",
                "data-plane-api/envoy/config/listener/v3/api_listener.proto",
                "data-plane-api/envoy/config/listener/v3/listener.proto",
                "data-plane-api/envoy/config/listener/v3/listener_components.proto",
                "data-plane-api/envoy/config/listener/v3/quic_config.proto",
                "data-plane-api/envoy/config/listener/v3/udp_listener_config.proto",
                "data-plane-api/envoy/config/metrics/v3/metrics_service.proto",
                "data-plane-api/envoy/config/metrics/v3/stats.proto",
                "data-plane-api/envoy/config/overload/v3/overload.proto",
                "data-plane-api/envoy/config/ratelimit/v3/rls.proto",
                "data-plane-api/envoy/config/rbac/v3/rbac.proto",
                "data-plane-api/envoy/config/route/v3/route.proto",
                "data-plane-api/envoy/config/route/v3/route_components.proto",
                "data-plane-api/envoy/config/route/v3/scoped_route.proto",
                "data-plane-api/envoy/config/tap/v3/common.proto",
                "data-plane-api/envoy/config/trace/v3/datadog.proto",
                "data-plane-api/envoy/config/trace/v3/dynamic_ot.proto",
                "data-plane-api/envoy/config/trace/v3/http_tracer.proto",
                "data-plane-api/envoy/config/trace/v3/lightstep.proto",
                "data-plane-api/envoy/config/trace/v3/opencensus.proto",
                "data-plane-api/envoy/config/trace/v3/service.proto",
                "data-plane-api/envoy/config/trace/v3/skywalking.proto",
                "data-plane-api/envoy/config/trace/v3/trace.proto",
                "data-plane-api/envoy/config/trace/v3/xray.proto",
                "data-plane-api/envoy/config/trace/v3/zipkin.proto",
                "data-plane-api/envoy/data/accesslog/v3/accesslog.proto",
                "data-plane-api/envoy/data/cluster/v3/outlier_detection_event.proto",
                "data-plane-api/envoy/data/core/v3/health_check_event.proto",
                "data-plane-api/envoy/data/dns/v3/dns_table.proto",
                "data-plane-api/envoy/data/tap/v3/common.proto",
                "data-plane-api/envoy/data/tap/v3/http.proto",
                "data-plane-api/envoy/data/tap/v3/transport.proto",
                "data-plane-api/envoy/data/tap/v3/wrapper.proto",
                "data-plane-api/envoy/extensions/access_loggers/file/v3/file.proto",
                "data-plane-api/envoy/extensions/access_loggers/filters/cel/v3/cel.proto",
                "data-plane-api/envoy/extensions/access_loggers/grpc/v3/als.proto",
                "data-plane-api/envoy/extensions/access_loggers/open_telemetry/v3/logs_service.proto",
                "data-plane-api/envoy/extensions/access_loggers/stream/v3/stream.proto",
                "data-plane-api/envoy/extensions/access_loggers/wasm/v3/wasm.proto",
                "data-plane-api/envoy/extensions/cache/simple_http_cache/v3/config.proto",
                "data-plane-api/envoy/extensions/clusters/aggregate/v3/cluster.proto",
                "data-plane-api/envoy/extensions/clusters/dynamic_forward_proxy/v3/cluster.proto",
                "data-plane-api/envoy/extensions/clusters/redis/v3/redis_cluster.proto",
                "data-plane-api/envoy/extensions/common/dynamic_forward_proxy/v3/dns_cache.proto",
                "data-plane-api/envoy/extensions/common/matching/v3/extension_matcher.proto",
                "data-plane-api/envoy/extensions/common/ratelimit/v3/ratelimit.proto",
                "data-plane-api/envoy/extensions/common/tap/v3/common.proto",
                "data-plane-api/envoy/extensions/compression/brotli/compressor/v3/brotli.proto",
                "data-plane-api/envoy/extensions/compression/brotli/decompressor/v3/brotli.proto",
                "data-plane-api/envoy/extensions/compression/gzip/compressor/v3/gzip.proto",
                "data-plane-api/envoy/extensions/compression/gzip/decompressor/v3/gzip.proto",
                "data-plane-api/envoy/extensions/filters/common/dependency/v3/dependency.proto",
                "data-plane-api/envoy/extensions/filters/common/fault/v3/fault.proto",
                "data-plane-api/envoy/extensions/filters/common/matcher/action/v3/skip_action.proto",
                "data-plane-api/envoy/extensions/filters/http/adaptive_concurrency/v3/adaptive_concurrency.proto",
                "data-plane-api/envoy/extensions/filters/http/admission_control/v3/admission_control.proto",
                "data-plane-api/envoy/extensions/filters/http/alternate_protocols_cache/v3/alternate_protocols_cache.proto",
                "data-plane-api/envoy/extensions/filters/http/aws_lambda/v3/aws_lambda.proto",
                "data-plane-api/envoy/extensions/filters/http/aws_request_signing/v3/aws_request_signing.proto",
                "data-plane-api/envoy/extensions/filters/http/bandwidth_limit/v3/bandwidth_limit.proto",
                "data-plane-api/envoy/extensions/filters/http/buffer/v3/buffer.proto",
                "data-plane-api/envoy/extensions/filters/http/cache/v3/cache.proto",
                "data-plane-api/envoy/extensions/filters/http/cdn_loop/v3/cdn_loop.proto",
                "data-plane-api/envoy/extensions/filters/http/composite/v3/composite.proto",
                "data-plane-api/envoy/extensions/filters/http/compressor/v3/compressor.proto",
                "data-plane-api/envoy/extensions/filters/http/cors/v3/cors.proto",
                "data-plane-api/envoy/extensions/filters/http/csrf/v3/csrf.proto",
                "data-plane-api/envoy/extensions/filters/http/decompressor/v3/decompressor.proto",
                "data-plane-api/envoy/extensions/filters/http/dynamic_forward_proxy/v3/dynamic_forward_proxy.proto",
                "data-plane-api/envoy/extensions/filters/http/dynamo/v3/dynamo.proto",
                "data-plane-api/envoy/extensions/filters/http/ext_authz/v3/ext_authz.proto",
                "data-plane-api/envoy/extensions/filters/http/ext_proc/v3/ext_proc.proto",
                "data-plane-api/envoy/extensions/filters/http/ext_proc/v3/processing_mode.proto",
                "data-plane-api/envoy/extensions/filters/http/fault/v3/fault.proto",
                "data-plane-api/envoy/extensions/filters/http/grpc_http1_bridge/v3/config.proto",
                "data-plane-api/envoy/extensions/filters/http/grpc_http1_reverse_bridge/v3/config.proto",
                "data-plane-api/envoy/extensions/filters/http/grpc_json_transcoder/v3/transcoder.proto",
                "data-plane-api/envoy/extensions/filters/http/grpc_stats/v3/config.proto",
                "data-plane-api/envoy/extensions/filters/http/grpc_web/v3/grpc_web.proto",
                "data-plane-api/envoy/extensions/filters/http/gzip/v3/gzip.proto",
                "data-plane-api/envoy/extensions/filters/http/header_to_metadata/v3/header_to_metadata.proto",
                "data-plane-api/envoy/extensions/filters/http/health_check/v3/health_check.proto",
                "data-plane-api/envoy/extensions/filters/http/ip_tagging/v3/ip_tagging.proto",
                "data-plane-api/envoy/extensions/filters/http/jwt_authn/v3/config.proto",
                "data-plane-api/envoy/extensions/filters/http/kill_request/v3/kill_request.proto",
                "data-plane-api/envoy/extensions/filters/http/local_ratelimit/v3/local_rate_limit.proto",
                "data-plane-api/envoy/extensions/filters/http/lua/v3/lua.proto",
                "data-plane-api/envoy/extensions/filters/http/oauth2/v3/oauth.proto",
                "data-plane-api/envoy/extensions/filters/http/on_demand/v3/on_demand.proto",
                "data-plane-api/envoy/extensions/filters/http/original_src/v3/original_src.proto",
                "data-plane-api/envoy/extensions/filters/http/ratelimit/v3/rate_limit.proto",
                "data-plane-api/envoy/extensions/filters/http/rbac/v3/rbac.proto",
                "data-plane-api/envoy/extensions/filters/http/router/v3/router.proto",
                "data-plane-api/envoy/extensions/filters/http/set_metadata/v3/set_metadata.proto",
                "data-plane-api/envoy/extensions/filters/http/tap/v3/tap.proto",
                "data-plane-api/envoy/extensions/filters/http/wasm/v3/wasm.proto",
                "data-plane-api/envoy/extensions/filters/listener/http_inspector/v3/http_inspector.proto",
                "data-plane-api/envoy/extensions/filters/listener/original_dst/v3/original_dst.proto",
                "data-plane-api/envoy/extensions/filters/listener/original_src/v3/original_src.proto",
                "data-plane-api/envoy/extensions/filters/listener/proxy_protocol/v3/proxy_protocol.proto",
                "data-plane-api/envoy/extensions/filters/listener/tls_inspector/v3/tls_inspector.proto",
                "data-plane-api/envoy/extensions/filters/network/client_ssl_auth/v3/client_ssl_auth.proto",
                "data-plane-api/envoy/extensions/filters/network/connection_limit/v3/connection_limit.proto",
                "data-plane-api/envoy/extensions/filters/network/direct_response/v3/config.proto",
                "data-plane-api/envoy/extensions/filters/network/dubbo_proxy/router/v3/router.proto",
                "data-plane-api/envoy/extensions/filters/network/dubbo_proxy/v3/dubbo_proxy.proto",
                "data-plane-api/envoy/extensions/filters/network/dubbo_proxy/v3/route.proto",
                "data-plane-api/envoy/extensions/filters/network/echo/v3/echo.proto",
                "data-plane-api/envoy/extensions/filters/network/ext_authz/v3/ext_authz.proto",
                "data-plane-api/envoy/extensions/filters/network/http_connection_manager/v3/http_connection_manager.proto",
                "data-plane-api/envoy/extensions/filters/network/local_ratelimit/v3/local_rate_limit.proto",
                "data-plane-api/envoy/extensions/filters/network/meta_protocol_proxy/matcher/action/v3/action.proto",
                "data-plane-api/envoy/extensions/filters/network/meta_protocol_proxy/matcher/v3/matcher.proto",
                "data-plane-api/envoy/extensions/filters/network/meta_protocol_proxy/v3/meta_protocol_proxy.proto",
                "data-plane-api/envoy/extensions/filters/network/meta_protocol_proxy/v3/route.proto",
                "data-plane-api/envoy/extensions/filters/network/mongo_proxy/v3/mongo_proxy.proto",
                "data-plane-api/envoy/extensions/filters/network/ratelimit/v3/rate_limit.proto",
                "data-plane-api/envoy/extensions/filters/network/rbac/v3/rbac.proto",
                "data-plane-api/envoy/extensions/filters/network/redis_proxy/v3/redis_proxy.proto",
                "data-plane-api/envoy/extensions/filters/network/sni_cluster/v3/sni_cluster.proto",
                "data-plane-api/envoy/extensions/filters/network/sni_dynamic_forward_proxy/v3/sni_dynamic_forward_proxy.proto",
                "data-plane-api/envoy/extensions/filters/network/tcp_proxy/v3/tcp_proxy.proto",
                "data-plane-api/envoy/extensions/filters/network/thrift_proxy/filters/header_to_metadata/v3/header_to_metadata.proto",
                "data-plane-api/envoy/extensions/filters/network/thrift_proxy/filters/ratelimit/v3/rate_limit.proto",
                "data-plane-api/envoy/extensions/filters/network/thrift_proxy/router/v3/router.proto",
                "data-plane-api/envoy/extensions/filters/network/thrift_proxy/v3/route.proto",
                "data-plane-api/envoy/extensions/filters/network/thrift_proxy/v3/thrift_proxy.proto",
                "data-plane-api/envoy/extensions/filters/network/wasm/v3/wasm.proto",
                "data-plane-api/envoy/extensions/filters/network/zookeeper_proxy/v3/zookeeper_proxy.proto",
                "data-plane-api/envoy/extensions/filters/udp/dns_filter/v3/dns_filter.proto",
                "data-plane-api/envoy/extensions/filters/udp/udp_proxy/v3/udp_proxy.proto",
                "data-plane-api/envoy/extensions/formatter/metadata/v3/metadata.proto",
                "data-plane-api/envoy/extensions/formatter/req_without_query/v3/req_without_query.proto",
                "data-plane-api/envoy/extensions/health_checkers/redis/v3/redis.proto",
                "data-plane-api/envoy/extensions/http/header_formatters/preserve_case/v3/preserve_case.proto",
                "data-plane-api/envoy/extensions/http/original_ip_detection/custom_header/v3/custom_header.proto",
                "data-plane-api/envoy/extensions/http/original_ip_detection/xff/v3/xff.proto",
                "data-plane-api/envoy/extensions/internal_redirect/allow_listed_routes/v3/allow_listed_routes_config.proto",
                "data-plane-api/envoy/extensions/internal_redirect/previous_routes/v3/previous_routes_config.proto",
                "data-plane-api/envoy/extensions/internal_redirect/safe_cross_scheme/v3/safe_cross_scheme_config.proto",
                "data-plane-api/envoy/extensions/key_value/file_based/v3/config.proto",
                "data-plane-api/envoy/extensions/matching/common_inputs/environment_variable/v3/input.proto",
                "data-plane-api/envoy/extensions/matching/input_matchers/consistent_hashing/v3/consistent_hashing.proto",
                "data-plane-api/envoy/extensions/matching/input_matchers/ip/v3/ip.proto",
                "data-plane-api/envoy/extensions/network/dns_resolver/apple/v3/apple_dns_resolver.proto",
                "data-plane-api/envoy/extensions/network/dns_resolver/cares/v3/cares_dns_resolver.proto",
                "data-plane-api/envoy/extensions/network/socket_interface/v3/default_socket_interface.proto",
                "data-plane-api/envoy/extensions/quic/crypto_stream/v3/crypto_stream.proto",
                "data-plane-api/envoy/extensions/quic/proof_source/v3/proof_source.proto",
                "data-plane-api/envoy/extensions/rate_limit_descriptors/expr/v3/expr.proto",
                "data-plane-api/envoy/extensions/rbac/matchers/upstream_ip_port/v3/upstream_ip_port_matcher.proto",
                "data-plane-api/envoy/extensions/request_id/uuid/v3/uuid.proto",
                "data-plane-api/envoy/extensions/resource_monitors/fixed_heap/v3/fixed_heap.proto",
                "data-plane-api/envoy/extensions/resource_monitors/injected_resource/v3/injected_resource.proto",
                "data-plane-api/envoy/extensions/retry/host/omit_canary_hosts/v3/omit_canary_hosts.proto",
                "data-plane-api/envoy/extensions/retry/host/omit_host_metadata/v3/omit_host_metadata_config.proto",
                "data-plane-api/envoy/extensions/retry/host/previous_hosts/v3/previous_hosts.proto",
                "data-plane-api/envoy/extensions/retry/priority/previous_priorities/v3/previous_priorities_config.proto",
                "data-plane-api/envoy/extensions/stat_sinks/graphite_statsd/v3/graphite_statsd.proto",
                "data-plane-api/envoy/extensions/stat_sinks/wasm/v3/wasm.proto",
                "data-plane-api/envoy/extensions/transport_sockets/alts/v3/alts.proto",
                "data-plane-api/envoy/extensions/transport_sockets/proxy_protocol/v3/upstream_proxy_protocol.proto",
                "data-plane-api/envoy/extensions/transport_sockets/quic/v3/quic_transport.proto",
                "data-plane-api/envoy/extensions/transport_sockets/raw_buffer/v3/raw_buffer.proto",
                "data-plane-api/envoy/extensions/transport_sockets/s2a/v3/s2a.proto",
                "data-plane-api/envoy/extensions/transport_sockets/starttls/v3/starttls.proto",
                "data-plane-api/envoy/extensions/transport_sockets/tap/v3/tap.proto",
                "data-plane-api/envoy/extensions/transport_sockets/tcp_stats/v3/tcp_stats.proto",
                "data-plane-api/envoy/extensions/transport_sockets/tls/v3/cert.proto",
                "data-plane-api/envoy/extensions/transport_sockets/tls/v3/common.proto",
                "data-plane-api/envoy/extensions/transport_sockets/tls/v3/secret.proto",
                "data-plane-api/envoy/extensions/transport_sockets/tls/v3/tls.proto",
                "data-plane-api/envoy/extensions/transport_sockets/tls/v3/tls_spiffe_validator_config.proto",
                "data-plane-api/envoy/extensions/upstreams/http/generic/v3/generic_connection_pool.proto",
                "data-plane-api/envoy/extensions/upstreams/http/http/v3/http_connection_pool.proto",
                "data-plane-api/envoy/extensions/upstreams/http/tcp/v3/tcp_connection_pool.proto",
                "data-plane-api/envoy/extensions/upstreams/http/v3/http_protocol_options.proto",
                "data-plane-api/envoy/extensions/upstreams/tcp/generic/v3/generic_connection_pool.proto",
                "data-plane-api/envoy/extensions/wasm/v3/wasm.proto",
                "data-plane-api/envoy/extensions/watchdog/profile_action/v3/profile_action.proto",
                "data-plane-api/envoy/service/accesslog/v3/als.proto",
                "data-plane-api/envoy/service/auth/v3/attribute_context.proto",
                "data-plane-api/envoy/service/auth/v3/external_auth.proto",
                "data-plane-api/envoy/service/cluster/v3/cds.proto",
                "data-plane-api/envoy/service/discovery/v3/ads.proto",
                "data-plane-api/envoy/service/discovery/v3/discovery.proto",
                "data-plane-api/envoy/service/endpoint/v3/eds.proto",
                "data-plane-api/envoy/service/endpoint/v3/leds.proto",
                "data-plane-api/envoy/service/event_reporting/v3/event_reporting_service.proto",
                "data-plane-api/envoy/service/ext_proc/v3/external_processor.proto",
                "data-plane-api/envoy/service/extension/v3/config_discovery.proto",
                "data-plane-api/envoy/service/health/v3/hds.proto",
                "data-plane-api/envoy/service/listener/v3/lds.proto",
                "data-plane-api/envoy/service/load_stats/v3/lrs.proto",
                "data-plane-api/envoy/service/metrics/v3/metrics_service.proto",
                "data-plane-api/envoy/service/ratelimit/v3/rls.proto",
                "data-plane-api/envoy/service/route/v3/rds.proto",
                "data-plane-api/envoy/service/route/v3/srds.proto",
                "data-plane-api/envoy/service/runtime/v3/rtds.proto",
                "data-plane-api/envoy/service/secret/v3/sds.proto",
                "data-plane-api/envoy/service/status/v3/csds.proto",
                "data-plane-api/envoy/service/tap/v3/tap.proto",
                "data-plane-api/envoy/service/trace/v3/trace_service.proto",
                "data-plane-api/envoy/type/http/v3/path_transformation.proto",
                "data-plane-api/envoy/type/matcher/v3/http_inputs.proto",
                "data-plane-api/envoy/type/matcher/v3/metadata.proto",
                "data-plane-api/envoy/type/matcher/v3/node.proto",
                "data-plane-api/envoy/type/matcher/v3/number.proto",
                "data-plane-api/envoy/type/matcher/v3/path.proto",
                "data-plane-api/envoy/type/matcher/v3/regex.proto",
                "data-plane-api/envoy/type/matcher/v3/string.proto",
                "data-plane-api/envoy/type/matcher/v3/struct.proto",
                "data-plane-api/envoy/type/matcher/v3/value.proto",
                "data-plane-api/envoy/type/metadata/v3/metadata.proto",
                "data-plane-api/envoy/type/tracing/v3/custom_tag.proto",
                "data-plane-api/envoy/type/v3/hash_policy.proto",
                "data-plane-api/envoy/type/v3/http.proto",
                "data-plane-api/envoy/type/v3/http_status.proto",
                "data-plane-api/envoy/type/v3/percent.proto",
                "data-plane-api/envoy/type/v3/range.proto",
                "data-plane-api/envoy/type/v3/ratelimit_unit.proto",
                "data-plane-api/envoy/type/v3/semantic_version.proto",
                "data-plane-api/envoy/type/v3/token_bucket.proto",
                "data-plane-api/envoy/watchdog/v3/abort_action.proto",
            ],
            &[
                "data-plane-api/",
                "xds/",
                "protoc-gen-validate/",
                "googleapis/",
                "opencensus-proto/src/",
                "opentelemetry-proto/",
                "prometheus_client_model/",
            ],
        )?;

        build_pbjson(descriptor_path)?;

    Ok(())
}

#[cfg(feature = "json")]
fn build_pbjson(descriptor_path: PathBuf) -> Result<()> {
    let descriptor_set = std::fs::read(descriptor_path)?;
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)?
        .extern_path(".google.protobuf", "::pbjson_types")
        .build(&[
            ".envoy",
            ".xds",
            ".google.api",
            // ".google.protobuf",
            ".google.rpc",
            ".io.prometheus",
            ".opencensus",
            ".opentelemetry",
            ".udpa",
            ".validate",
        ])?;
    Ok(())
}

#[cfg(not(feature = "json"))]
fn build_pbjson(_: PathBuf) -> Result<()> { Ok(()) }
