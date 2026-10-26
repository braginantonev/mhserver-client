use {
    crate::repository::transport::Channel as InterceptedChannel,
    tonic::transport::Channel as InnerChannel,
};

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
