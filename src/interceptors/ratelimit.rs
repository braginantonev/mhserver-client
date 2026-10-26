use crate::repository::ratelimit::{Limiter, Rate};

pub struct LimiterInterceptor {
    unary: Limiter,
    stream: Limiter  
}

impl tonic::service::Interceptor for LimiterInterceptor {
    fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        self.unary.wait();
        Ok(request)
    }
}

impl Default for LimiterInterceptor {
    fn default() -> Self {
        // temp. for now idk how to add interceptor for stream-requests.
        // so I use unary rate for stream requests.
        let rate = Rate::new(5, std::time::Duration::from_secs(1));
        Self { unary: Limiter::new(rate.clone()), stream: Limiter::new(rate) }
    }
}