//! Save the GitHub App's secrets to the .env file

use std::env::current_dir;
use std::fs::File;
use std::io::{Read, Write};

use anyhow::{Context, Error};

use super::app::App;

/// Saves the GitHub App's secrets to the .env file
///
/// This function saves the GitHub App's secrets to the .env file. It reads the existing .env file
/// and filters out the existing GitHub App secrets. Then it appends the new GitHub App secrets to
/// the end of the file.
pub(crate) fn save_to_env(app: &App) -> Result<(), Error> {
    let path = format!(
        "{}/.env",
        current_dir()
            .context("failed to read the current directory")?
            .display()
    );

    let mut file = File::options()
        .create(true)
        .read(true)
        .truncate(true)
        .write(true)
        .open(path)
        .context("failed to open the .env file")?;

    let mut old_dotenv = String::new();
    file.read_to_string(&mut old_dotenv)
        .context("failed to read the .env file")?;

    let new_dotenv = update_env(&old_dotenv, app);
    file.write_all(new_dotenv.as_bytes())
        .context("failed to write the .env file")?;

    Ok(())
}

/// Updates the .env file with the GitHub App's secrets
///
/// This function updates the .env file with the GitHub App's secrets. It reads the existing .env
/// file and filters out the existing GitHub App secrets. Then it appends the new GitHub App
/// secrets to the end of the file. The filter skips the existing keys, so that the function can
/// insert them at the end of the file.
fn update_env(old_env: &str, app: &App) -> String {
    let mut new_env: String = old_env
        .lines()
        .filter(|line| {
            for variable in [
                "GITHUB_APP_ID",
                "GITHUB_APP_NAME",
                "GITHUB_CLIENT_ID",
                "GITHUB_CLIENT_SECRET",
                "GITHUB_WEBHOOK_SECRET",
                "GITHUB_PRIVATE_KEY",
            ] {
                if line.starts_with(variable) {
                    return false;
                }
            }

            true
        })
        .collect();

    if !new_env.is_empty() {
        new_env.push('\n');
    }

    new_env.push_str(&format!("GITHUB_APP_ID={}\n", app.id()));
    new_env.push_str(&format!("GITHUB_APP_NAME={}\n", app.name()));
    new_env.push_str(&format!("GITHUB_CLIENT_ID=\"{}\"\n", app.client_id()));
    new_env.push_str(&format!(
        "GITHUB_CLIENT_SECRET={}\n",
        app.client_secret().expose()
    ));

    if let Some(webhook_secret) = app.webhook_secret() {
        new_env.push_str(&format!(
            "GITHUB_WEBHOOK_SECRET={}\n",
            webhook_secret.expose()
        ));
    }

    new_env.push_str(&format!(
        "GITHUB_PRIVATE_KEY=\"{}\"\n",
        app.pem().expose().escape_default()
    ));

    new_env
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn replaces_github_app() {
        let dotenv = indoc! {r#"
            GITHUB_APP_ID=1
            DATABASE_URL=postgres://localhost/db
        "#};

        let app = App::builder()
            .id(1)
            .name("app")
            .client_id("client_id")
            .client_secret("client_secret")
            .webhook_secret("webhook_secret")
            .pem("pem")
            .build();

        let new_env = update_env(dotenv, &app);

        assert_eq!(
            indoc! {r#"
            DATABASE_URL=postgres://localhost/db
            GITHUB_APP_ID=1
            GITHUB_APP_NAME=app
            GITHUB_CLIENT_ID="client_id"
            GITHUB_CLIENT_SECRET=client_secret
            GITHUB_WEBHOOK_SECRET=webhook_secret
            GITHUB_PRIVATE_KEY="pem"
        "#},
            new_env
        );
    }
}
