// Copyright 2021 The rust-control-plane Authors. All rights reserved.
// Use of this source code is governed by the Apache License,
// Version 2.0, that can be found in the LICENSE file.

pub use pbjson_types_any;
pub use prost;
pub use prost_wkt_types;

// list of files generated with:
// $ find target/debug/build/rust-control-plane-4a01c79a4c642ce5/out -name '*.rs' | sort | while read f; do echo "pub mod $(basename $f | perl -lpe '$_=substr($_,0,-3) if eof') { include!(concat!(env!(""OUT_DIR""), \"$(basename $f)\")); }"; done | pbcopy
// (and then heavily modified by hand)

pub mod envoy {
    pub mod admin {
        pub mod v3 {
            include!(concat!(env!("OUT_DIR"), "/envoy.admin.v3.rs"));
            include!(concat!(env!("OUT_DIR"), "/envoy.admin.v3.serde.rs"));
        }
    }
    pub mod annotations {
        include!(concat!(env!("OUT_DIR"), "/envoy.annotations.rs"));
        include!(concat!(env!("OUT_DIR"), "/envoy.annotations.serde.rs"));
    }
    pub mod config {
        pub mod accesslog {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.config.accesslog.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.config.accesslog.v3.serde.rs"));
            }
        }
        pub mod bootstrap {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.config.bootstrap.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.config.bootstrap.v3.serde.rs"));
            }
        }
        pub mod cluster {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.config.cluster.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.config.cluster.v3.serde.rs"));
            }
        }
        pub mod common {
            pub mod key_value {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.config.common.key_value.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.config.common.key_value.v3.serde.rs"));
                }
            }
            pub mod matcher {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.config.common.matcher.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.config.common.matcher.v3.serde.rs"));
                }
            }
            pub mod mutation_rules {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.config.common.mutation_rules.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.config.common.mutation_rules.v3.serde.rs"));
                }
            }
        }
        pub mod core {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.config.core.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.config.core.v3.serde.rs"));
            }
        }
        pub mod endpoint {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.config.endpoint.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.config.endpoint.v3.serde.rs"));
            }
        }
        pub mod grpc_credential {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.config.grpc_credential.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.config.grpc_credential.v3.serde.rs"));
            }
        }
        pub mod listener {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.config.listener.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.config.listener.v3.serde.rs"));
            }
        }
        pub mod metrics {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.config.metrics.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.config.metrics.v3.serde.rs"));
            }
        }
        pub mod overload {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.config.overload.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.config.overload.v3.serde.rs"));
            }
        }
        pub mod ratelimit {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.config.ratelimit.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.config.ratelimit.v3.serde.rs"));
            }
        }
        pub mod rbac {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.config.rbac.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.config.rbac.v3.serde.rs"));
            }
        }
        pub mod route {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.config.route.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.config.route.v3.serde.rs"));
            }
        }
        pub mod tap {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.config.tap.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.config.tap.v3.serde.rs"));
            }
        }
        pub mod trace {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.config.trace.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.config.trace.v3.serde.rs"));
            }
        }
    }
    pub mod data {
        pub mod accesslog {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.data.accesslog.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.data.accesslog.v3.serde.rs"));
            }
        }
        pub mod cluster {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.data.cluster.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.data.cluster.v3.serde.rs"));
            }
        }
        pub mod core {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.data.core.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.data.core.v3.serde.rs"));
            }
        }
        pub mod dns {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.data.dns.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.data.dns.v3.serde.rs"));
            }
        }
        pub mod tap {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.data.tap.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.data.tap.v3.serde.rs"));
            }
        }
    }
    pub mod extensions {
        pub mod access_loggers {
            pub mod file {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.access_loggers.file.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.access_loggers.file.v3.serde.rs"));
                }
            }
            pub mod filters {
                pub mod cel {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.access_loggers.filters.cel.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.access_loggers.filters.cel.v3.serde.rs"));
                    }
                }
            }
            pub mod grpc {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.access_loggers.grpc.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.access_loggers.grpc.v3.serde.rs"));
                }
            }
            pub mod open_telemetry {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.access_loggers.open_telemetry.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.access_loggers.open_telemetry.v3.serde.rs"));
                }
            }
            pub mod stream {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.access_loggers.stream.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.access_loggers.stream.v3.serde.rs"));
                }
            }
            pub mod wasm {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.access_loggers.wasm.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.access_loggers.wasm.v3.serde.rs"));
                }
            }
        }
        pub mod cache {
            pub mod simple_http_cache {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.cache.simple_http_cache.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.cache.simple_http_cache.v3.serde.rs"));
                }
            }
        }
        pub mod common {
            pub mod dynamic_forward_proxy {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.common.dynamic_forward_proxy.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.common.dynamic_forward_proxy.v3.serde.rs"));
                }
            }
            pub mod matching {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.common.matching.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.common.matching.v3.serde.rs"));
                }
            }
            pub mod ratelimit {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.common.ratelimit.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.common.ratelimit.v3.serde.rs"));
                }
            }
            pub mod tap {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.common.tap.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.common.tap.v3.serde.rs"));
                }
            }
        }
        pub mod clusters {
            pub mod aggregate {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.clusters.aggregate.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.clusters.aggregate.v3.serde.rs"));
                }
            }
            pub mod dynamic_forward_proxy {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.clusters.dynamic_forward_proxy.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.clusters.dynamic_forward_proxy.v3.serde.rs"));
                }
            }
            pub mod redis {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.clusters.redis.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.clusters.redis.v3.serde.rs"));
                }
            }
        }
        pub mod compression {
            pub mod brotli {
                pub mod compressor {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.compression.brotli.compressor.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.compression.brotli.compressor.v3.serde.rs"));
                    }
                }
                pub mod decompressor {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.compression.brotli.decompressor.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.compression.brotli.decompressor.v3.serde.rs"));
                    }
                }
            }
            pub mod gzip {
                pub mod compressor {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.compression.gzip.compressor.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.compression.gzip.compressor.v3.serde.rs"));
                    }
                }
                pub mod decompressor {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.compression.gzip.decompressor.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.compression.gzip.decompressor.v3.serde.rs"));
                    }
                }
            }
        }
        pub mod filters {
            pub mod common {
                pub mod dependency {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.common.dependency.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.common.dependency.v3.serde.rs"));
                    }
                }
                pub mod fault {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.common.fault.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.common.fault.v3.serde.rs"));
                    }
                }
                pub mod matcher {
                    pub mod action {
                        pub mod v3 {
                            include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.common.matcher.action.v3.rs"));
                            include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.common.matcher.action.v3.serde.rs"));
                        }
                    }
                }
            }
            pub mod http {
                pub mod adaptive_concurrency {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.adaptive_concurrency.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.adaptive_concurrency.v3.serde.rs"));
                    }
                }
                pub mod admission_control {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.admission_control.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.admission_control.v3.serde.rs"));
                    }
                }
                pub mod alternate_protocols_cache {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.alternate_protocols_cache.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.alternate_protocols_cache.v3.serde.rs"));
                    }
                }
                pub mod aws_lambda {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.aws_lambda.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.aws_lambda.v3.serde.rs"));
                    }
                }
                pub mod aws_request_signing {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.aws_request_signing.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.aws_request_signing.v3.serde.rs"));
                    }
                }
                pub mod bandwidth_limit {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.bandwidth_limit.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.bandwidth_limit.v3.serde.rs"));
                    }
                }
                pub mod buffer {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.buffer.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.buffer.v3.serde.rs"));
                    }
                }
                pub mod cache {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.cache.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.cache.v3.serde.rs"));
                    }
                }
                pub mod cdn_loop {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.cdn_loop.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.cdn_loop.v3.serde.rs"));
                    }
                }
                pub mod composite {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.composite.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.composite.v3.serde.rs"));
                    }
                }
                pub mod compressor {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.compressor.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.compressor.v3.serde.rs"));
                    }
                }
                pub mod cors {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.cors.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.cors.v3.serde.rs"));
                    }
                }
                pub mod csrf {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.csrf.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.csrf.v3.serde.rs"));
                    }
                }
                pub mod decompressor {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.decompressor.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.decompressor.v3.serde.rs"));
                    }
                }
                pub mod dynamic_forward_proxy {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.dynamic_forward_proxy.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.dynamic_forward_proxy.v3.serde.rs"));
                    }
                }
                pub mod dynamo {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.dynamo.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.dynamo.v3.serde.rs"));
                    }
                }
                pub mod ext_authz {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.ext_authz.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.ext_authz.v3.serde.rs"));
                    }
                }
                pub mod ext_proc {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.ext_proc.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.ext_proc.v3.serde.rs"));
                    }
                }
                pub mod fault {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.fault.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.fault.v3.serde.rs"));
                    }
                }
                pub mod grpc_http1_bridge {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.grpc_http1_bridge.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.grpc_http1_bridge.v3.serde.rs"));
                    }
                }
                pub mod grpc_http1_reverse_bridge {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.grpc_http1_reverse_bridge.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.grpc_http1_reverse_bridge.v3.serde.rs"));
                    }
                }
                pub mod grpc_json_transcoder {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.grpc_json_transcoder.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.grpc_json_transcoder.v3.serde.rs"));
                    }
                }
                pub mod grpc_stats {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.grpc_stats.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.grpc_stats.v3.serde.rs"));
                    }
                }
                pub mod grpc_web {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.grpc_web.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.grpc_web.v3.serde.rs"));
                    }
                }
                pub mod gzip {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.gzip.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.gzip.v3.serde.rs"));
                    }
                }
                pub mod header_to_metadata {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.header_to_metadata.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.header_to_metadata.v3.serde.rs"));
                    }
                }
                pub mod health_check {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.health_check.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.health_check.v3.serde.rs"));
                    }
                }
                pub mod ip_tagging {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.ip_tagging.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.ip_tagging.v3.serde.rs"));
                    }
                }
                pub mod jwt_authn {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.jwt_authn.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.jwt_authn.v3.serde.rs"));
                    }
                }
                pub mod kill_request {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.kill_request.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.kill_request.v3.serde.rs"));
                    }
                }
                pub mod local_ratelimit {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.local_ratelimit.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.local_ratelimit.v3.serde.rs"));
                    }
                }
                pub mod lua {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.lua.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.lua.v3.serde.rs"));
                    }
                }
                pub mod oauth2 {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.oauth2.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.oauth2.v3.serde.rs"));
                    }
                }
                pub mod on_demand {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.on_demand.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.on_demand.v3.serde.rs"));
                    }
                }
                pub mod original_src {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.original_src.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.original_src.v3.serde.rs"));
                    }
                }
                pub mod ratelimit {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.ratelimit.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.ratelimit.v3.serde.rs"));
                    }
                }
                pub mod rbac {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.rbac.v3.rs"));
                        // include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.rbac.v3.serde.rs"));
                    }
                }
                pub mod router {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.router.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.router.v3.serde.rs"));
                    }
                }
                pub mod set_metadata {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.set_metadata.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.set_metadata.v3.serde.rs"));
                    }
                }
                pub mod tap {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.tap.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.tap.v3.serde.rs"));
                    }
                }
                pub mod wasm {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.wasm.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.http.wasm.v3.serde.rs"));
                    }
                }
            }
            pub mod listener {
                pub mod http_inspector {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.listener.http_inspector.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.listener.http_inspector.v3.serde.rs"));
                    }
                }
                pub mod original_dst {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.listener.original_dst.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.listener.original_dst.v3.serde.rs"));
                    }
                }
                pub mod original_src {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.listener.original_src.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.listener.original_src.v3.serde.rs"));
                    }
                }
                pub mod proxy_protocol {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.listener.proxy_protocol.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.listener.proxy_protocol.v3.serde.rs"));
                    }
                }
                pub mod tls_inspector {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.listener.tls_inspector.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.listener.tls_inspector.v3.serde.rs"));
                    }
                }
            }
            pub mod network {
                pub mod client_ssl_auth {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.client_ssl_auth.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.client_ssl_auth.v3.serde.rs"));
                    }
                }
                pub mod connection_limit {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.connection_limit.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.connection_limit.v3.serde.rs"));
                    }
                }
                pub mod direct_response {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.direct_response.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.direct_response.v3.serde.rs"));
                    }
                }
                pub mod dubbo_proxy {
                    pub mod router {
                        pub mod v3 {
                            include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.dubbo_proxy.router.v3.rs"));
                            include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.dubbo_proxy.router.v3.serde.rs"));
                        }
                    }
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.dubbo_proxy.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.dubbo_proxy.v3.serde.rs"));
                    }
                }
                pub mod echo {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.echo.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.echo.v3.serde.rs"));
                    }
                }
                pub mod ext_authz {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.ext_authz.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.ext_authz.v3.serde.rs"));
                    }
                }
                pub mod http_connection_manager {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.http_connection_manager.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.http_connection_manager.v3.serde.rs"));
                    }
                }
                pub mod local_ratelimit {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.local_ratelimit.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.local_ratelimit.v3.serde.rs"));
                    }
                }
                pub mod meta_protocol_proxy {
                    pub mod matcher {
                        pub mod action {
                            pub mod v3 {
                                include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.meta_protocol_proxy.matcher.action.v3.rs"));
                                include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.meta_protocol_proxy.matcher.action.v3.serde.rs"));
                            }
                        }
                        pub mod v3 {
                            include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.meta_protocol_proxy.matcher.v3.rs"));
                            include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.meta_protocol_proxy.matcher.v3.serde.rs"));
                        }
                    }
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.meta_protocol_proxy.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.meta_protocol_proxy.v3.serde.rs"));
                    }
                }
                pub mod mongo_proxy {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.mongo_proxy.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.mongo_proxy.v3.serde.rs"));
                    }
                }
                pub mod ratelimit {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.ratelimit.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.ratelimit.v3.serde.rs"));
                    }
                }
                pub mod rbac {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.rbac.v3.rs"));
                        // include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.rbac.v3.serde.rs"));
                    }
                }
                pub mod redis_proxy {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.redis_proxy.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.redis_proxy.v3.serde.rs"));
                    }
                }
                pub mod sni_cluster {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.sni_cluster.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.sni_cluster.v3.serde.rs"));
                    }
                }
                pub mod sni_dynamic_forward_proxy {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.sni_dynamic_forward_proxy.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.sni_dynamic_forward_proxy.v3.serde.rs"));
                    }
                }
                pub mod tcp_proxy {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.tcp_proxy.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.tcp_proxy.v3.serde.rs"));
                    }
                }
                pub mod thrift_proxy {
                    pub mod filters {
                        pub mod header_to_metadata {
                            pub mod v3 {
                                include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.thrift_proxy.filters.header_to_metadata.v3.rs"));
                                include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.thrift_proxy.filters.header_to_metadata.v3.serde.rs"));
                            }
                        }
                        pub mod ratelimit {
                            pub mod v3 {
                                include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.thrift_proxy.filters.ratelimit.v3.rs"));
                                include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.thrift_proxy.filters.ratelimit.v3.serde.rs"));
                            }
                        }
                    }
                    pub mod router {
                        pub mod v3 {
                            include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.thrift_proxy.router.v3.rs"));
                            include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.thrift_proxy.router.v3.serde.rs"));
                        }
                    }
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.thrift_proxy.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.thrift_proxy.v3.serde.rs"));
                    }
                }
                pub mod wasm {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.wasm.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.wasm.v3.serde.rs"));
                    }
                }
                pub mod zookeeper_proxy {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.zookeeper_proxy.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.network.zookeeper_proxy.v3.serde.rs"));
                    }
                }
            }
            pub mod udp {
                pub mod dns_filter {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.udp.dns_filter.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.udp.dns_filter.v3.serde.rs"));
                    }
                }
                pub mod udp_proxy {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.udp.udp_proxy.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.filters.udp.udp_proxy.v3.serde.rs"));
                    }
                }
            }
        }
        pub mod formatter {
            pub mod metadata {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.formatter.metadata.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.formatter.metadata.v3.serde.rs"));
                }
            }
            pub mod req_without_query {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.formatter.req_without_query.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.formatter.req_without_query.v3.serde.rs"));
                }
            }
        }
        pub mod health_checkers {
            pub mod redis {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.health_checkers.redis.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.health_checkers.redis.v3.serde.rs"));
                }
            }
        }
        pub mod http {
            pub mod header_formatters {
                pub mod preserve_case {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.http.header_formatters.preserve_case.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.http.header_formatters.preserve_case.v3.serde.rs"));
                    }
                }
            }
            pub mod original_ip_detection {
                pub mod custom_header {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.http.original_ip_detection.custom_header.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.http.original_ip_detection.custom_header.v3.serde.rs"));
                    }
                }
                pub mod xff {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.http.original_ip_detection.xff.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.http.original_ip_detection.xff.v3.serde.rs"));
                    }
                }
            }
        }
        pub mod internal_redirect {
            pub mod allow_listed_routes {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.internal_redirect.allow_listed_routes.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.internal_redirect.allow_listed_routes.v3.serde.rs"));
                }
            }
            pub mod previous_routes {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.internal_redirect.previous_routes.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.internal_redirect.previous_routes.v3.serde.rs"));
                }
            }
            pub mod safe_cross_scheme {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.internal_redirect.safe_cross_scheme.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.internal_redirect.safe_cross_scheme.v3.serde.rs"));
                }
            }
        }
        pub mod key_value {
            pub mod file_based {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.key_value.file_based.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.key_value.file_based.v3.serde.rs"));
                }
            }
        }
        pub mod matching {
            pub mod common_inputs {
                pub mod environment_variable {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.matching.common_inputs.environment_variable.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.matching.common_inputs.environment_variable.v3.serde.rs"));
                    }
                }
            }
            pub mod input_matchers {
                pub mod consistent_hashing {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.matching.input_matchers.consistent_hashing.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.matching.input_matchers.consistent_hashing.v3.serde.rs"));
                    }
                }
                pub mod ip {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.matching.input_matchers.ip.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.matching.input_matchers.ip.v3.serde.rs"));
                    }
                }
            }
        }
        pub mod network {
            pub mod dns_resolver {
                pub mod apple {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.network.dns_resolver.apple.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.network.dns_resolver.apple.v3.serde.rs"));
                    }
                }
                pub mod cares {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.network.dns_resolver.cares.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.network.dns_resolver.cares.v3.serde.rs"));
                    }
                }
            }
            pub mod socket_interface {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.network.socket_interface.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.network.socket_interface.v3.serde.rs"));
                }
            }
        }
        pub mod quic {
            pub mod crypto_stream {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.quic.crypto_stream.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.quic.crypto_stream.v3.serde.rs"));
                }
            }
            pub mod proof_source {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.quic.proof_source.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.quic.proof_source.v3.serde.rs"));
                }
            }
        }
        pub mod rate_limit_descriptors {
            pub mod expr {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.rate_limit_descriptors.expr.v3.rs"));
                    // include!(concat!(env!("OUT_DIR"), "/envoy.extensions.rate_limit_descriptors.expr.v3.serde.rs"));
                }
            }
        }
        pub mod rbac {
            pub mod matchers {
                pub mod upstream_ip_port {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.rbac.matchers.upstream_ip_port.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.rbac.matchers.upstream_ip_port.v3.serde.rs"));
                    }
                }
            }
        }
        pub mod request_id {
            pub mod uuid {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.request_id.uuid.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.request_id.uuid.v3.serde.rs"));
                }
            }
        }
        pub mod resource_monitors {
            pub mod fixed_heap {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.resource_monitors.fixed_heap.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.resource_monitors.fixed_heap.v3.serde.rs"));
                }
            }
            pub mod injected_resource {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.resource_monitors.injected_resource.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.resource_monitors.injected_resource.v3.serde.rs"));
                }
            }
        }
        pub mod retry {
            pub mod host {
                pub mod omit_canary_hosts {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.retry.host.omit_canary_hosts.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.retry.host.omit_canary_hosts.v3.serde.rs"));
                    }
                }
                pub mod omit_host_metadata {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.retry.host.omit_host_metadata.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.retry.host.omit_host_metadata.v3.serde.rs"));
                    }
                }
                pub mod previous_hosts {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.retry.host.previous_hosts.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.retry.host.previous_hosts.v3.serde.rs"));
                    }
                }
            }
            pub mod priority {
                pub mod previous_priorities {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.retry.priority.previous_priorities.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.retry.priority.previous_priorities.v3.serde.rs"));
                    }
                }
            }
        }
        pub mod stats_sinks {
            pub mod graphite_statsd {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.stat_sinks.graphite_statsd.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.stat_sinks.graphite_statsd.v3.serde.rs"));
                }
            }
            pub mod wasm {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.stat_sinks.wasm.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.stat_sinks.wasm.v3.serde.rs"));
                }
            }
        }
        pub mod transport_sockets {
            pub mod alts {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.alts.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.alts.v3.serde.rs"));
                }
            }
            pub mod proxy_protocol {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.proxy_protocol.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.proxy_protocol.v3.serde.rs"));
                }
            }
            pub mod quic {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.quic.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.quic.v3.serde.rs"));
                }
            }
            pub mod raw_buffer {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.raw_buffer.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.raw_buffer.v3.serde.rs"));
                }
            }
            pub mod s2a {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.s2a.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.s2a.v3.serde.rs"));
                }
            }
            pub mod starttls {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.starttls.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.starttls.v3.serde.rs"));
                }
            }
            pub mod tap {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.tap.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.tap.v3.serde.rs"));
                }
            }
            pub mod tcp_stats {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.tcp_stats.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.tcp_stats.v3.serde.rs"));
                }
            }
            pub mod tls {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.tls.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.transport_sockets.tls.v3.serde.rs"));
                }
            }
        }
        pub mod upstreams {
            pub mod http {
                pub mod generic {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.upstreams.http.generic.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.upstreams.http.generic.v3.serde.rs"));
                    }
                }
                pub mod http {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.upstreams.http.http.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.upstreams.http.http.v3.serde.rs"));
                    }
                }
                pub mod tcp {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.upstreams.http.tcp.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.upstreams.http.tcp.v3.serde.rs"));
                    }
                }
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.upstreams.http.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.upstreams.http.v3.serde.rs"));
                }
            }
            pub mod tcp {
                pub mod generic {
                    pub mod v3 {
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.upstreams.tcp.generic.v3.rs"));
                        include!(concat!(env!("OUT_DIR"), "/envoy.extensions.upstreams.tcp.generic.v3.serde.rs"));
                    }
                }
            }
        }
        pub mod wasm {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.extensions.wasm.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.extensions.wasm.v3.serde.rs"));
            }
        }
        pub mod watchdog {
            pub mod profile_action {
                pub mod v3 {
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.watchdog.profile_action.v3.rs"));
                    include!(concat!(env!("OUT_DIR"), "/envoy.extensions.watchdog.profile_action.v3.serde.rs"));
                }
            }
        }
    }
    pub mod r#type {
        pub mod http {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.r#type.http.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.type.http.v3.serde.rs"));
            }
        }
        pub mod matcher {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.r#type.matcher.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.type.matcher.v3.serde.rs"));
            }
        }
        pub mod metadata {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.r#type.metadata.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.type.metadata.v3.serde.rs"));
            }
        }
        pub mod tracing {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.r#type.tracing.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/envoy.type.tracing.v3.serde.rs"));
            }
        }
        pub mod v3 {
            include!(concat!(env!("OUT_DIR"), "/envoy.r#type.v3.rs"));
            include!(concat!(env!("OUT_DIR"), "/envoy.type.v3.serde.rs"));
        }
    }
    pub mod service {
        pub mod accesslog {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.accesslog.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.accesslog.v3.serde.rs"));
            }
        }
        pub mod auth {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.auth.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.auth.v3.serde.rs"));
            }
        }
        pub mod cluster {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.cluster.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.cluster.v3.serde.rs"));
            }
        }
        pub mod discovery {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.discovery.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.discovery.v3.serde.rs"));
            }
        }
        pub mod endpoint {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.endpoint.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.endpoint.v3.serde.rs"));
            }
        }
        pub mod event_reporting {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.event_reporting.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.event_reporting.v3.serde.rs"));
            }
        }
        pub mod ext_proc {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.ext_proc.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.ext_proc.v3.serde.rs"));
            }
        }
        pub mod extension {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.extension.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.extension.v3.serde.rs"));
            }
        }
        pub mod health {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.health.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.health.v3.serde.rs"));
            }
        }
        pub mod listener {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.listener.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.listener.v3.serde.rs"));
            }
        }
        pub mod load_stats {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.load_stats.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.load_stats.v3.serde.rs"));
            }
        }
        pub mod metrics {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.metrics.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.metrics.v3.serde.rs"));
            }
        }
        pub mod ratelimit {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.ratelimit.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.ratelimit.v3.serde.rs"));
            }
        }
        pub mod route {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.route.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.route.v3.serde.rs"));
            }
        }
        pub mod runtime {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.runtime.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.runtime.v3.serde.rs"));
            }
        }
        pub mod secret {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.secret.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.secret.v3.serde.rs"));
            }
        }
        pub mod status {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.status.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.status.v3.serde.rs"));
            }
        }
        pub mod tap {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.tap.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.tap.v3.serde.rs"));
            }
        }
        pub mod trace {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/envoy.service.trace.v3.rs"));
                // include!(concat!(env!("OUT_DIR"), "/envoy.service.trace.v3.serde.rs"));
            }
        }
    }
    pub mod watchdog {
        pub mod v3 {
            include!(concat!(env!("OUT_DIR"), "/envoy.watchdog.v3.rs"));
            include!(concat!(env!("OUT_DIR"), "/envoy.watchdog.v3.serde.rs"));
        }
    }
}
pub mod google {
    pub mod api {
        pub mod expr {
            pub mod v1alpha1 {
                include!(concat!(env!("OUT_DIR"), "/google.api.expr.v1alpha1.rs"));
                // include!(concat!(env!("OUT_DIR"), "/google.api.expr.v1alpha1.serde.rs"));
            }
        }
        include!(concat!(env!("OUT_DIR"), "/google.api.rs"));
        // include!(concat!(env!("OUT_DIR"), "/google.api.serde.rs"));
    }
    pub mod protobuf {
        include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
        // include!(concat!(env!("OUT_DIR"), "/google.protobuf.serde.rs"));
    }
    pub mod rpc {
        include!(concat!(env!("OUT_DIR"), "/google.rpc.rs"));
        // include!(concat!(env!("OUT_DIR"), "/google.rpc.serde.rs"));
    }
}
pub mod io {
    pub mod prometheus {
        pub mod client {
            include!(concat!(env!("OUT_DIR"), "/io.prometheus.client.rs"));
            include!(concat!(env!("OUT_DIR"), "/io.prometheus.client.serde.rs"));
        }
    }
}
pub mod opencensus {
    pub mod proto {
        pub mod resource {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/opencensus.proto.resource.v1.rs"));
                include!(concat!(env!("OUT_DIR"), "/opencensus.proto.resource.v1.serde.rs"));
            }
        }
        pub mod trace {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/opencensus.proto.trace.v1.rs"));
                include!(concat!(env!("OUT_DIR"), "/opencensus.proto.trace.v1.serde.rs"));
            }
        }
    }
}
pub mod opentelemetry {
    pub mod proto {
        pub mod common {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/opentelemetry.proto.common.v1.rs"));
                include!(concat!(env!("OUT_DIR"), "/opentelemetry.proto.common.v1.serde.rs"));
            }
        }
    }
}
pub mod udpa {
    pub mod annotations {
        include!(concat!(env!("OUT_DIR"), "/udpa.annotations.rs"));
        include!(concat!(env!("OUT_DIR"), "/udpa.annotations.serde.rs"));
    }
}
pub mod validate {
    include!(concat!(env!("OUT_DIR"), "/validate.rs"));
    include!(concat!(env!("OUT_DIR"), "/validate.serde.rs"));
}
pub mod xds {
    pub mod annotations {
        pub mod v3 {
            include!(concat!(env!("OUT_DIR"), "/xds.annotations.v3.rs"));
            include!(concat!(env!("OUT_DIR"), "/xds.annotations.v3.serde.rs"));
        }
    }
    pub mod core {
        pub mod v3 {
            include!(concat!(env!("OUT_DIR"), "/xds.core.v3.rs"));
            include!(concat!(env!("OUT_DIR"), "/xds.core.v3.serde.rs"));
        }
    }
    pub mod r#type {
        pub mod matcher {
            pub mod v3 {
                include!(concat!(env!("OUT_DIR"), "/xds.r#type.matcher.v3.rs"));
                include!(concat!(env!("OUT_DIR"), "/xds.type.matcher.v3.serde.rs"));
            }
        }
    }
}

#[test]
fn test_any_json_roundtrip() {
    use crate::envoy::config::bootstrap::v3::bootstrap::StaticResources;
    use crate::envoy::config::bootstrap::v3::Bootstrap;
    use crate::envoy::config::core::v3::address;
    use crate::envoy::config::core::v3::data_source::Specifier;
    use crate::envoy::config::core::v3::socket_address::PortSpecifier;
    use crate::envoy::config::core::v3::transport_socket::ConfigType;
    use crate::envoy::config::core::v3::{Address, DataSource, SocketAddress, TransportSocket};
    use crate::envoy::config::listener::v3::{FilterChain, Listener};
    use crate::envoy::extensions::transport_sockets::tls::v3::common_tls_context::ValidationContextType;
    use crate::envoy::extensions::transport_sockets::tls::v3::subject_alt_name_matcher::SanType;
    use crate::envoy::extensions::transport_sockets::tls::v3::tls_parameters::TlsProtocol;
    use crate::envoy::extensions::transport_sockets::tls::v3::{CertificateValidationContext, SubjectAltNameMatcher};
    use crate::envoy::extensions::transport_sockets::tls::v3::{CommonTlsContext, DownstreamTlsContext, TlsCertificate, TlsParameters};
    use crate::envoy::r#type::matcher::v3::string_matcher::MatchPattern;
    use crate::envoy::r#type::matcher::v3::StringMatcher;
    use crate::pbjson_types_any::BoolValue;
    use crate::prost_wkt_types::Any;

    const BOOTSTRAP_JSON: &str = r#"{
  "staticResources": {
    "listeners": [
      {
        "name": "server-1",
        "address": {
          "socketAddress": {
            "address": "127.0.0.1",
            "portValue": 9000
          }
        },
        "filterChains": [
          {
            "filters": [],
            "transportSocket": {
              "name": "envoy.transport_sockets.tls",
              "typedConfig": {
                "@type": "type.googleapis.com/envoy.extensions.transport_sockets.tls.v3.DownstreamTlsContext",
                "requireClientCertificate": { "value": true },
                "commonTlsContext": {
                  "tlsParams": {
                    "tlsMinimumProtocolVersion": "TLSv1_3",
                    "tlsMaximumProtocolVersion": "TLSv1_3"
                  },
                  "validationContext": {
                    "trustedCa": {
                      "filename": "./certs/ca.crt"
                    },
                    "matchTypedSubjectAltNames": [
                      {
                        "sanType": "DNS",
                        "matcher": {
                          "exact": "client.test"
                        }
                      }
                    ]
                  },
                  "tlsCertificates": [
                    {
                      "certificateChain": {
                        "filename": "./certs/server.test.ecdsa-p256.crt"
                      },
                      "privateKey": {
                        "filename": "./certs/server.test.ecdsa-p256.key"
                      }
                    }
                  ]
                }
              }
            }
          }
        ]
      }
    ]
  }
}
    "#;

    let expected_tls_context = DownstreamTlsContext {
        require_client_certificate: Some(BoolValue { value: true }),
        common_tls_context: Some(CommonTlsContext {
            tls_params: Some(TlsParameters {
                tls_minimum_protocol_version: TlsProtocol::TlSv13 as i32,
                tls_maximum_protocol_version: TlsProtocol::TlSv13 as i32,
                ..Default::default()
            }),
            validation_context_type: Some(ValidationContextType::ValidationContext(CertificateValidationContext {
                trusted_ca: Some(DataSource {
                    specifier: Some(Specifier::Filename("./certs/ca.crt".to_string())),
                }),
                match_typed_subject_alt_names: vec![SubjectAltNameMatcher {
                    san_type: SanType::Dns as i32,
                    matcher: Some(StringMatcher {
                        ignore_case: false,
                        match_pattern: Some(MatchPattern::Exact("client.test".to_string())),
                    }),
                }],
                ..Default::default()
            })),
            tls_certificates: vec![TlsCertificate {
                certificate_chain: Some(DataSource {
                    specifier: Some(Specifier::Filename("./certs/server.test.ecdsa-p256.crt".to_string())),
                }),
                private_key: Some(DataSource {
                    specifier: Some(Specifier::Filename("./certs/server.test.ecdsa-p256.key".to_string())),
                }),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };

    let bootstrap: Bootstrap = serde_json::from_str(BOOTSTRAP_JSON).unwrap();
    let expected = Bootstrap {
        static_resources: Some(StaticResources {
            listeners: vec![Listener {
                name: "server-1".to_string(),
                address: Some(Address {
                    address: Some(address::Address::SocketAddress(SocketAddress {
                        address: "127.0.0.1".to_string(),
                        port_specifier: Some(PortSpecifier::PortValue(9000)),
                        ..Default::default()
                    })),
                }),
                filter_chains: vec![FilterChain {
                    filters: vec![],
                    transport_socket: Some(TransportSocket {
                        name: "envoy.transport_sockets.tls".to_string(),
                        config_type: Some(ConfigType::TypedConfig(Any::try_pack(expected_tls_context.clone()).unwrap())),
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };

    use pbjson_any::prost_wkt::MessageSerde;
    use prost::Message;

    let downstream_tls_context_type_url = DownstreamTlsContext::default().type_url();

    let expected_tls_any = expected.static_resources.as_ref().unwrap().listeners[0].filter_chains[0]
        .transport_socket
        .as_ref()
        .unwrap()
        .config_type
        .as_ref()
        .unwrap();
    let ConfigType::TypedConfig(tls_any) = expected_tls_any;
    if &tls_any.type_url == downstream_tls_context_type_url {
        let ctx = DownstreamTlsContext::decode(&*tls_any.value).unwrap();
        assert_eq!(&expected_tls_context, &ctx);
    } else {
        panic!("unknown envoy.transport_sockets.tls typed config: {}", &tls_any.type_url);
    }

    let bootstrap_tls_any = bootstrap.static_resources.as_ref().unwrap().listeners[0].filter_chains[0]
        .transport_socket
        .as_ref()
        .unwrap()
        .config_type
        .as_ref()
        .unwrap();
    let ConfigType::TypedConfig(tls_any) = bootstrap_tls_any;
    if &tls_any.type_url == downstream_tls_context_type_url {
        let ctx = DownstreamTlsContext::decode(&*tls_any.value).unwrap();
        assert_eq!(&expected_tls_context, &ctx);
    } else {
        panic!("unknown envoy.transport_sockets.tls typed config: {}", &tls_any.type_url);
    }

    assert_eq!(expected, bootstrap);
}
