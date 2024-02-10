use grammers_client::{Client, Config, InitParams, SignInError};
use grammers_session::Session;
use rand::{distributions::Alphanumeric, Rng};
use std::path::{Path, PathBuf};
use std::fs;

use crate::utils::prompt::prompt;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn sign_in(client: &Client) -> Result<()> {
    log::info!("Signing in...");

    let phone = prompt("Enter your phone number (international format): ")?;
    let login_token = client.request_login_code(&phone).await?;
    let code = prompt("Enter the code you received: ")?;

    let signed_in = client.sign_in(&login_token, &code).await;
    match signed_in {
        Err(SignInError::PasswordRequired(password_token)) => {
            let hint = password_token.hint();
            let hint_message = match hint {
                Option::Some(hint) => {
                    format!("hint: \"{}\"", hint)
                }
                Option::None => String::from("no hint"),
            };

            for _ in 0..=3 {
                let prompt_message = format!("Enter the password ({}): ", hint_message);
                let password = prompt(&prompt_message)?;

                match client
                    .check_password(password_token.clone(), password)
                    .await
                {
                    Ok(_) => {
                        println!("The password matches!");
                        break;
                    }
                    Err(SignInError::InvalidPassword) => {}
                    Err(e) => println!("Error checking the password, {}", e),
                }
            }
        }
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    };

    println!("Singed in!");

    Ok(())
}

pub struct SessionsManager<'a> {
    sessions_folder: &'a Path,
    present_sessions_files: fs::ReadDir,
}

impl<'a> SessionsManager<'a> {
    pub fn new(sessions_folder: &'a Path) -> Self {
        Self {
            sessions_folder,
            present_sessions_files: fs::read_dir(sessions_folder).unwrap(),
        }
    }

    pub fn get_session_file(&mut self) -> PathBuf {
        match self.present_sessions_files.next() {
            Some(Ok(present_file_name)) => present_file_name.path().to_owned(),
            None | Some(Err(_)) => {
                let rng = &mut rand::thread_rng().sample_iter(&Alphanumeric);
                let mut new_path: Option<PathBuf> = None;

                while new_path.is_none() || new_path.as_ref().unwrap().exists() {
                    let random_string = rng.take(7).map(char::from).collect::<String>();

                    let file_name = format!("session-{}.session", random_string);
                    let mut base = self.sessions_folder.to_owned();
                    base.push(file_name);

                    new_path = Some(base);
                }

                new_path.unwrap().to_owned()
            }
        }
    }
}

pub struct ClientFactory<'a> {
    api_id: i32,
    api_hash: String,
    sessions_manager: SessionsManager<'a>,
}

impl<'a> ClientFactory<'a> {
    pub fn new(sessions_folder: &'a Path, api_id: i32, api_hash: String) -> Self {
        let sessions_manager = SessionsManager::new(sessions_folder);

        Self {
            api_id,
            api_hash,
            sessions_manager,
        }
    }

    pub async fn make_client(&mut self) -> Result<Client> {
        let session_file = self.sessions_manager.get_session_file();
        log::info!(
            "Discovered session file at: {}",
            session_file.to_str().unwrap()
        );

        let client = Client::connect(Config {
            session: Session::load_file_or_create(&session_file)?,
            api_id: self.api_id,
            api_hash: self.api_hash.clone(),
            params: InitParams {
                catch_up: true,
                ..Default::default()
            },
        })
        .await?;

        log::info!("Connected! The session file is {:?}", session_file.to_str());

        if !client.is_authorized().await? {
            sign_in(&client).await?;

            match client.session().save_to_file(session_file) {
                Ok(_) => {}
                Err(e) => {
                    println!("NOTE: failed to save the session: {}", e);
                }
            }
        }

        Ok(client)
    }
}
