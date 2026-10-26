use {
    crate::interceptors::*,
    tonic::service::interceptor::InterceptedService,
};

// Why not in di? Just because
// But services also need a Channel type for clients, but them doesn't know about app and di of course.
// So, there is here
pub type Channel = InterceptedService<
    InterceptedService<tonic::transport::Channel, auth::AuthInterceptor>,
    ratelimit::LimiterInterceptor
>;