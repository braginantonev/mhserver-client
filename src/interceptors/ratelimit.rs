use {
    crate::repository::ratelimit::{Limiter, Rate},
    std::{
        pin::Pin,
        future::Future,
        sync::Arc,
    },
    tower::Service,
    tonic::{
        body::Body,
        codegen::http::{Request, Response},
    },
};

#[derive(Clone)]
pub struct LimiterInterceptor {
    unary: Limiter,
    stream: Limiter  
}

impl LimiterInterceptor {
    pub fn new(unary_rate: Rate, stream_rate: Rate) -> Self {
        Self { unary: Limiter::new(unary_rate), stream: Limiter::new(stream_rate) }
    }
}

pub struct LimiterLayer {
    interceptor: LimiterInterceptor
}

impl LimiterLayer {
    pub fn new(interceptor: LimiterInterceptor) -> Self {
        Self { interceptor }
    }
}

impl<S> tower::Layer<S> for LimiterLayer {
    type Service = LimiterService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LimiterService::new(inner, self.interceptor.clone())
    }
}

#[derive(Clone)]
pub struct LimiterService<S> {
    inner: S,
    interceptor: Arc<LimiterInterceptor>,
}

impl<S> LimiterService<S> {
    pub fn new(service: S, interceptor: LimiterInterceptor) -> Self {
        Self { inner: service, interceptor: Arc::new(interceptor) }
    }
}

impl<S> Service<Request<Body>> for LimiterService<S>
where
    S: Service<Request<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{ 
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    
    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx) 
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let interceptor = self.interceptor.clone();

        Box::pin(async move {
            println!("{}", req.uri());
            interceptor.unary.wait().await;
            let response = inner.call(req).await?;
            Ok(response)
        })     
    }
}
