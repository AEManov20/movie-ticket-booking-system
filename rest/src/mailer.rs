use std::sync::Arc;

use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use tokio::sync::Mutex;
use tokio::sync::mpsc::{channel, error::SendError, Receiver, Sender};
use tokio::task::JoinHandle;

#[derive(Clone)]
pub struct MailerConfig {
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub password: String,
}

pub struct Mailer {
    config: MailerConfig,
    message_sender: Option<Sender<Message>>,
    killer: Arc<Mutex<Option<Sender<()>>>>,
    task: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl Mailer {
    pub fn new(config: MailerConfig) -> Self {
        Self {
            config,
            killer: Arc::new(Mutex::new(None)),
            message_sender: None,
            task: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn queue_mail(
        &self,
        msg: Message,
    ) -> Result<(), either::Either<SendError<Message>, ()>> {
        match &self.message_sender {
            Some(v) => v.send(msg).await.map_err(either::Either::Left),
            None => Err(either::Either::Right(())),
        }
    }

    pub async fn start(&mut self) {
        let (message_sender, message_receiver) = channel::<Message>(1024);
        // TODO: refactor to oneshot
        let (killer, kill_signal) = channel::<()>(1);

        *self.killer.lock().await = Some(killer);
        self.message_sender = Some(message_sender);
        let task = Self::loop_;
        let config = self.config.clone();

        *self.task.lock().await = Some(tokio::spawn(async move {
            match task(config, message_receiver, kill_signal).await {
                Ok(_) => log::info!("Mailing thread exited successfully"),
                Err(e) => log::error!("Mailing thread exited with error: {:?}", e),
            };
        }));
    }

    pub async fn stop(&mut self) {
        log::info!("Exiting mail thread...");
        let Some(ref killer) = *self.killer.lock().await else {
            return;
        };

        // let Some(ref task) = *self.task.lock().await else {
        //     return;
        // };

        killer.send(()).await.unwrap();
    }

    async fn loop_(
        config: MailerConfig,
        message_receiver: Receiver<Message>,
        mut kill_signal: Receiver<()>,
    ) -> Result<(), std::io::Error> {
        tokio::select! {
            output = Self::actual_loop(config, message_receiver) => output,
            _ = kill_signal.recv() => Ok(()),
        }
    }

    async fn actual_loop(
        config: MailerConfig,
        mut message_receiver: Receiver<Message>,
    ) -> Result<(), std::io::Error> {
        let Ok(mailer) = SmtpTransport::relay(&config.host) else {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Mailer cannot be created"));
        };

        let mut mailer = mailer.credentials(Credentials::new(config.user, config.password));
        let mut failed_queue: Vec<Message> = vec![];

        if let Some(port) = config.port {
            mailer = mailer.port(port);
        }

        let mailer = mailer.build();

        log::info!("Mailing thread started");

        loop {
            for el in failed_queue.iter() {
                let _ = send_mail(mailer.clone(), el.to_owned()).await;
            }

            failed_queue.clear();

            if let Some(v) = message_receiver.recv().await {
                if let Ok(Some(failed_v)) = send_mail(mailer.clone(), v).await {
                    log::error!("Pushing to failed queue");
                    failed_queue.push(failed_v);
                }
            }
        }
    }
}

fn send_mail(mailer: SmtpTransport, msg: Message) -> JoinHandle<Option<Message>> {
    tokio::task::spawn_blocking(move || {
        match mailer.send(&msg) {
            Ok(v) => {
                log::info!("Sent an email, response: {:?}", v);
                None
            }
            Err(e) => {
                log::error!("Error when emailing: {:?}", e);
                Some(msg)
            }
        }
    })
}
