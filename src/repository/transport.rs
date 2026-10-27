use {
    crate::interceptors::*,
};

// Why not in di? Just because
// But services also need a Channel type for clients, but them doesn't know about app and di of course.
// So, there is here
pub type Channel = ratelimit::LimiterService<
    auth::AuthService<tonic::transport::Channel>
>;