use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{future::Future, path::Path, sync::Arc};
use tokio::sync::mpsc::{self, Receiver};
use tracing::error;

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (tx, rx) = mpsc::channel(1);

    let handle = tokio::runtime::Handle::current();
    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            handle.block_on(async {
                if tx.send(res).await.ok().is_none() {
                    error!("reload notify channel closed - can't send");
                }
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

pub trait CallbackTrait<Ret: Future<Output = ()> + Send + Sync> {
    fn process_event(&self, event: Event, dir: &'static str) -> Ret;
}

pub struct CallbackNoParams<
    F: Fn(Event, &'static str) -> Ret + 'static + Send + Sync,
    Ret: Future<Output = ()> + Send + Sync,
> {
    pub callback: &'static F,
}

impl<
        F: Fn(Event, &'static str) -> Ret + 'static + Send + Sync,
        Ret: Future<Output = ()> + Send + Sync,
    > CallbackNoParams<F, Ret>
{
    fn new(callback: &'static F) -> Self {
        Self { callback }
    }

    pub fn wrap(callback: &'static F) -> Arc<Self> {
        Arc::new(Self::new(callback))
    }
}

impl<
        F: Fn(Event, &'static str) -> Ret + 'static + Send + Sync,
        Ret: Future<Output = ()> + Send + Sync,
    > CallbackTrait<Ret> for CallbackNoParams<F, Ret>
{
    fn process_event(&self, event: Event, dir: &'static str) -> Ret {
        let x = self.callback;
        x(event, dir)
    }
}

pub struct CallbackOneParam<
    F: Fn(Event, &'static str, &'static TParam) -> Ret + 'static + Send + Sync,
    Ret: Future<Output = ()> + Send + Sync,
    TParam: 'static + Send + Sync,
> {
    pub callback: &'static F,
    pub param: &'static TParam,
}

impl<
        F: Fn(Event, &'static str, &'static TParam) -> Ret + 'static + Send + Sync,
        Ret: Future<Output = ()> + Send + Sync,
        TParam: 'static + Send + Sync,
    > CallbackOneParam<F, Ret, TParam>
{
    #[allow(unused)]
    fn new(callback: &'static F, param: &'static TParam) -> Self {
        Self { callback, param }
    }

    #[allow(unused)]
    pub fn wrap(callback: &'static F, param: &'static TParam) -> Arc<Self> {
        Arc::new(Self::new(callback, param))
    }
}

impl<
        F: Fn(Event, &'static str, &'static TParam) -> Ret + 'static + Send + Sync,
        Ret: Future<Output = ()> + Send + Sync,
        TParam: Send + Sync,
    > CallbackTrait<Ret> for CallbackOneParam<F, Ret, TParam>
{
    fn process_event(&self, event: Event, dir: &'static str) -> Ret {
        let x = self.callback;
        x(event, dir, self.param)
    }
}

pub fn watch_directory<
    Ret: Future<Output = ()> + Send + Sync,
    C: CallbackTrait<Ret> + 'static + Send + Sync,
>(
    dir: &'static str,
    callback: Arc<C>,
) {
    //https://old.reddit.com/r/rust/comments/q6nyc6/async_file_watcher_like_notifyrs/
    tokio::task::spawn(async move {
        #[cfg(feature = "log")]
        let Ok(log_subscription) = dxp_logging::subscribe_thread_with_default() else {
            return;
        };

        async {
            loop {
                // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                if let Err(e) = async_watch(dir, std::path::Path::new(dir), callback.clone()).await
                {
                    error!("error watch_directory reload: {:?}", e)
                }
            }
        }
        .await;

        #[cfg(feature = "log")]
        drop(log_subscription);
    });
}

pub async fn async_watch<
    Ret: Future<Output = ()> + Send + Sync,
    C: CallbackTrait<Ret> + 'static + Send + Sync,
    P: AsRef<Path>,
>(
    dir: &'static str,
    path: P,
    callback: Arc<C>,
) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.recv().await {
        match res {
            Ok(event) => {
                if event.kind
                    == notify::EventKind::Modify(notify::event::ModifyKind::Data(
                        notify::event::DataChange::Any,
                    ))
                {
                    continue;
                }

                callback.process_event(event, dir).await;
            }
            Err(e) => error!("watch error: {e:?}"),
        }
    }

    Ok(())
}
