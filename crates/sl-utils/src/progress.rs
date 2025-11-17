#[derive(Debug, Clone, Copy)]
pub enum ProgressEvent<'a> {
    Begin(&'a str),
    End,
    Progress(ProgressReport<'a>),
    /// When all progress events should be stopped, the progress receiver shall be closed.
    StopAll,
    StopForUrl(&'a str),
}

#[derive(Debug, Clone, Copy)]
pub struct ProgressReport<'a> {
    url: &'a str,
    total: u64,
    current: u64,
}

impl<'a> ProgressReport<'a> {
    pub const fn new(url: &'a str, total: u64, current: u64) -> Self {
        ProgressReport {
            url,
            total,
            current,
        }
    }

    pub const fn with_current(mut self, current: u64) -> Self {
        self.current = current;
        self
    }

    pub const fn total(&self) -> u64 {
        self.total
    }

    pub const fn current(&self) -> u64 {
        self.current
    }

    pub const fn url(&self) -> &str {
        self.url
    }
}

pub struct ProgressReceiver {
    callback: Box<dyn Fn(ProgressEvent) + Send + Sync + 'static>,
}

impl Drop for ProgressReceiver {
    fn drop(&mut self) {
        (self.callback)(ProgressEvent::StopAll);
    }
}

impl ProgressReceiver {
    pub fn new<F: Fn(ProgressEvent) + Send + Sync + 'static>(callback: F) -> Self {
        ProgressReceiver {
            callback: Box::new(callback),
        }
    }

    fn send_progress(&self, report: ProgressReport) {
        (self.callback)(ProgressEvent::Progress(report));
    }

    fn begin(&self, message: &str) {
        (self.callback)(ProgressEvent::Begin(message));
    }

    fn end(&self) {
        (self.callback)(ProgressEvent::End);
    }

    pub fn begin_sending<'a>(&'a self, msg: &str) -> ProgressSender<'a> {
        self.begin(msg);
        ProgressSender { receiver: self }
    }
}

pub struct ProgressSender<'a> {
    receiver: &'a ProgressReceiver,
}

impl<'a> ProgressSender<'a> {
    pub fn send(&self, report: ProgressReport) {
        self.receiver.send_progress(report);
    }

    pub fn stop_url(&self, url: &str) {
        (self.receiver.callback)(ProgressEvent::StopForUrl(url));
    }
}

impl<'a> Drop for ProgressSender<'a> {
    fn drop(&mut self) {
        self.receiver.end();
    }
}
