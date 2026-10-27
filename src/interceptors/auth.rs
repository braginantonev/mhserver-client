use {
    std::{
        pin::Pin,
        future::Future,
    },
    tower::Service,
    std::str::FromStr,
    tonic::{
        metadata::AsciiMetadataValue,
        body::Body,
        codegen::http::request::Request,
    },
};

#[derive(Clone)]
pub struct AuthInterceptor {
    token: AsciiMetadataValue
}

impl AuthInterceptor {
    pub fn new(token: &str) -> Self {
        Self { token: AsciiMetadataValue::from_str(token).unwrap_or(AsciiMetadataValue::from_static("")), }
    }
    
    pub fn token(&self) -> AsciiMetadataValue {
        self.token.clone()    }
}

pub struct AuthLayer {
    interceptor: AuthInterceptor
}

impl AuthLayer {
    pub fn new(interceptor: AuthInterceptor) -> Self {
        Self { interceptor }
    }
}

impl<S> tower::Layer<S> for AuthLayer {
    type Service = AuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthService::new(inner, self.interceptor.clone())
    }
} 

#[derive(Clone)]
pub struct AuthService<S> {
    inner: S,
    interceptor: AuthInterceptor
}

impl<S> AuthService<S> {
    pub fn new(inner: S, interceptor: AuthInterceptor) -> Self {
        Self { inner, interceptor }
    }
}

impl<S> Service<Request<Body>> for AuthService<S>
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
    
    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let token = self.interceptor.token();

        Box::pin(async move {
            println!("{}", req.uri()); // temp

            let mut metadata = tonic::metadata::MetadataMap::from_headers(
                std::mem::take(req.headers_mut())
            );
            
            metadata.insert("authorization", token);
            *req.headers_mut() = metadata.into_headers();

            let response = inner.call(req).await?;
            Ok(response)
        })     
    }
} 