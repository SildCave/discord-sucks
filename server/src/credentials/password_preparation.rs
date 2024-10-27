
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use anyhow::Result;
use tokio::task::JoinHandle;
use tracing::info;

use super::password::{self, Password};

use thiserror::Error;


#[derive(Error, Debug)]
pub enum PasswordPreparationError {
    #[error("Failed to hash password")]
    HashError(String),
}


pub struct PreparedPassword {
    pub salt: String,
    pub password_hash: String,
}

fn hash_password<'a>(
    salt: &'a SaltString,
    password: &'a str
) -> Result<PasswordHash<'a>, argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), salt);
    password_hash
}

pub enum SaltMode<'a> {
    Generate,
    FromString(&'a str),
}

impl Password<'_> {
    pub async fn hash_and_salt_password(
        &self,
        salt: &SaltMode<'_>
    ) -> Result<PreparedPassword> {
        let password = self.get_password();
        let salt = match salt {
            SaltMode::FromString(salt) => {
                info!("Salt: {}", salt);
                let salt = SaltString::from_b64(&salt).unwrap();
                salt
            },
            SaltMode::Generate => {
                SaltString::generate(&mut OsRng)
            }
        };
        let salt_string = salt.to_string();

        let task: JoinHandle<Result<String, argon2::password_hash::Error>> = tokio::task::spawn_blocking({
            let password = password.to_string();
            move || {
                let pass = hash_password(&salt, &password);
                match pass {
                    Ok(pass) => Ok(pass.to_string()),
                    Err(e) => Err(e)
                }
            }
        });

        //let password_hash = argon2.hash_password(password.as_bytes(), &salt);
        let password_hash = task.await?;
        let password_hash = password_hash.map_err(|e| PasswordPreparationError::HashError(e.to_string()))?;
        Ok(PreparedPassword {
            salt: salt_string,
            password_hash,
        })
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Alphanumeric, Rng};
    
    #[tokio::test]
    async fn test_hash_and_salt_password() {
        let password: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let requirements = password::PasswordRequirements::default();
        let password = Password::new(&password, &requirements);
        let salt = "YOtX2//7NoD/owm8RZ8llw".to_string();
        let prepared_password = password.hash_and_salt_password(&SaltMode::FromString(&salt)).await.unwrap();

        let hash_to_check = password.hash_and_salt_password(&SaltMode::FromString(&salt)).await.unwrap();

        assert!(prepared_password.salt == hash_to_check.salt);
    }
}