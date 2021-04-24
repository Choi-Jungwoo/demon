pub mod errors;
pub mod scheme;
pub mod work_state;

use work_state::WorkState;
use bytes::BytesMut;
use crossbeam_channel::{select, unbounded};
use errors::DownloadError;
use rayon;
use scheme::Scheme;
use std::{
    fmt::{self, Display},
    sync::Arc,
};

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Clone)]
pub struct Downloader<S: Scheme>(Arc<S>);

impl<S> Downloader<S>
where
    S: Scheme + 'static,
{
    pub fn read(&self, num_threads: usize) -> Result<BytesMut, DownloadError> {
        let buf = BytesMut::new();
        let mut bufs: Vec<(usize, BytesMut)> = Vec::new();
        let (bufs_sender, bufs_receiver) = unbounded::<(usize, BytesMut)>();
        let (pool_result_sender, pool_result_receiver) = unbounded::<()>();
        let mut work_state = WorkState::init();

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .unwrap();

        let each_len = (self.0.get_length()? - 1) / num_threads;

        for i in 0..num_threads - 1 {
            let scheme = self.0.clone();
            let bufs_sender = bufs_sender.clone();
            let pool_result_sender= pool_result_sender.clone();
            let (start, end) = (i * each_len, (i + 1) * each_len);

            pool.spawn(move || {
                let mut buf = BytesMut::with_capacity(end - start);
                scheme.download(&mut buf, start, end).unwrap();
                bufs_sender.send((i, buf)).unwrap();
                pool_result_sender.send(()).unwrap();
            });

            work_state.ongoing_work();
        }

        loop {
            select! {
                recv(bufs_receiver) -> msg => {
                    bufs.push(msg.unwrap());
                },

                recv(pool_result_receiver) -> _ => {
                    work_state.done_ongoing_work();

                    if work_state.nomore_works() {
                        break;
                    }
                }
            }
        }

        println!("{:#?}", bufs);

        Ok(buf)
    }
}

impl<S> From<S> for Downloader<S>
where
    S: Scheme,
{
    fn from(scheme: S) -> Self {
        Downloader(Arc::new(scheme))
    }
}

impl<S> AsRef<S> for Downloader<S>
where
    S: Scheme,
{
    fn as_ref(&self) -> &S {
        &self.0
    }
}

impl<S: Scheme> Display for Downloader<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
