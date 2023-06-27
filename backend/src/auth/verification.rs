use rand::{self, distributions::Alphanumeric, Rng};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::Config,
    db::{self, email_verifications, users},
    email::{self, Address, Email},
    errors::Error,
};

pub async fn verify_email_by_code(pool: &PgPool, user_id: Uuid, code: String) -> Result<(), Error> {
    let is_valid = email_verifications::get_verification_by_code(pool, user_id, code)
        .await
        .is_ok();

    if !is_valid {
        return Err(Error::BadRequestError(format!("Invalid code")));
    }

    users::set_email_verified(pool, user_id).await?;
    Ok(())
}

pub async fn send_verification_email(
    config: &Config,
    pool: &PgPool,
    user_id: Uuid,
) -> Result<(), Error> {
    let code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    email_verifications::save_verification_code(pool, user_id, code.clone()).await?;

    let user = db::users::get_user_by_id(pool, user_id).await?;

    email::send_email(
        config,
        Email {
            recipient: Address {
                first_name: user.first_name.clone(),
                last_name: user.last_name,
                email_address: user.email,
            },
            sender: Address {
                first_name: String::from("Jacob"),
                last_name: String::from("Matthews"),
                email_address: config.from_email.clone(),
            },
            subject: String::from("Verify Your Email | jacobmatthe.ws"),
            text: format!(
                "Please use the following code to verify your email: {}",
                code
            ),
            html: format!(
                "
                <html>
                    <p>Hi {},</p>
                    <p>Please copy the following code to verify your email address:</p>
                    <br />
                    <pre>{}</pre>
                    <br />
                    <p>Thanks,</p>
                    <p>Jacob</p>
                </html>
                ",
                user.first_name, code
            ),
        },
    )
}