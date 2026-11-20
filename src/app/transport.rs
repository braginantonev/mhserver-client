use {
    crate::{
        repository::transport::Channel as InterceptedChannel,
        config::app::read_pem_cert,
    },
    tonic::transport::{Channel as InnerChannel, Certificate, ClientTlsConfig},
};

pub fn init_tls_config() -> Result<ClientTlsConfig, std::io::Error> {
    read_pem_cert().map(|pem| ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(pem)))
}

pub struct Channel {
    inner: InnerChannel,
    intercepted: InterceptedChannel,
}

impl Channel {
    pub fn new(inner: InnerChannel, intercepted: InterceptedChannel) -> Self {
        Self { inner, intercepted }
    }

    pub fn inner(&self) -> InnerChannel {
        self.inner.clone()
    }

    pub fn intercepted(&self) -> InterceptedChannel {
        self.intercepted.clone()
    }
}
