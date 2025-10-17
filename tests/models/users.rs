use crate::prepare::users::{create_random_user, create_random_user_with_password};
use chrono::{offset::Local, Duration};
use fake::{faker::internet::en::Password, Fake};
use insta::assert_debug_snapshot;
use loco_nuxt_template::{
    app::App,
    models::users::{self, Model, RegisterParams},
};
use loco_rs::testing::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue, IntoActiveModel};
use serial_test::parallel;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("users");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[parallel]
async fn test_can_validate_model() {
    configure_insta!();

    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");

    let invalid_user = users::ActiveModel {
        name: ActiveValue::set("1".to_string()),
        email: ActiveValue::set("invalid-email".to_string()),
        ..Default::default()
    };

    let res = invalid_user.insert(&boot.app_context.db).await;

    assert_debug_snapshot!(res);
}

#[tokio::test]
#[parallel]
async fn can_create_with_password() {
    configure_insta!();

    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");

    let params = RegisterParams {
        email: "test@framework.com".to_string(),
        password: "1234".to_string(),
        name: "framework".to_string(),
    };

    let res = Model::create_with_password(&boot.app_context.db, &params).await;

    insta::with_settings!({
        filters => cleanup_user_model()
    }, {
        assert_debug_snapshot!(res);
    });
}

#[tokio::test]
#[parallel]
async fn handle_create_with_password_with_duplicate() -> anyhow::Result<()> {
    configure_insta!();

    let boot = boot_test::<App>()
        .await
        .expect("Failed to boot test application");

    let new_user = create_random_user(&boot.app_context.db).await?;

    let duplicate_user = Model::create_with_password(
        &boot.app_context.db,
        &RegisterParams {
            email: new_user.email,
            password: "3221".to_string(),
            name: "John".to_string(),
        },
    )
    .await;

    assert_debug_snapshot!(duplicate_user);

    Ok(())
}

#[tokio::test]
#[parallel]
async fn can_find_by_email() -> anyhow::Result<()> {
    configure_insta!();

    let boot = boot_test::<App>().await?;

    let params = RegisterParams {
        email: "can_find_by_email@example.com".to_string(),
        password: "12341234".to_string(),
        name: "John Doe".to_string(),
    };
    let user = Model::create_with_password(&boot.app_context.db, &params).await?;

    let existing_user = Model::find_by_email(&boot.app_context.db, &user.email).await;
    let non_existing_user_results =
        Model::find_by_email(&boot.app_context.db, "un@existing-email.com").await;

    insta::with_settings!({
        filters => cleanup_user_model()
    }, {
        assert_debug_snapshot!(existing_user);
    });

    insta::with_settings!({
        filters => cleanup_user_model()
    }, {
        assert_debug_snapshot!(non_existing_user_results);
    });

    Ok(())
}

#[tokio::test]
#[parallel]
async fn can_find_by_pid() -> anyhow::Result<()> {
    configure_insta!();

    let boot = boot_test::<App>().await?;
    let params = RegisterParams {
        email: "can_find_by_pid@example.com".to_string(),
        password: "12341234".to_string(),
        name: "John Doe".to_string(),
    };

    let user = Model::create_with_password(&boot.app_context.db, &params).await?;

    let existing_user = Model::find_by_pid(&boot.app_context.db, &user.pid.to_string()).await;
    let non_existing_user_results =
        Model::find_by_pid(&boot.app_context.db, "23232323-2323-2323-2323-232323232323").await;

    insta::with_settings!({
        filters => cleanup_user_model()
    }, {
        assert_debug_snapshot!(existing_user);
    });
    insta::with_settings!({
        filters => cleanup_user_model()
    }, {
        assert_debug_snapshot!(non_existing_user_results);
    });

    Ok(())
}

#[tokio::test]
#[parallel]
async fn can_verification_token() -> anyhow::Result<()> {
    configure_insta!();

    let boot = boot_test::<App>().await?;

    let user = create_random_user(&boot.app_context.db).await?;

    assert!(user.email_verification_sent_at.is_none());
    assert!(user.email_verification_token.is_none());

    let pid = user.pid.to_string();
    assert!(user
        .into_active_model()
        .set_email_verification_sent(&boot.app_context.db)
        .await
        .is_ok());

    let user = Model::find_by_pid(&boot.app_context.db, &pid).await?;

    assert!(user.email_verification_sent_at.is_some());
    assert!(user.email_verification_token.is_some());

    Ok(())
}

#[tokio::test]
#[parallel]
async fn can_set_forgot_password_sent() -> anyhow::Result<()> {
    configure_insta!();

    let boot = boot_test::<App>().await?;

    let user = create_random_user(&boot.app_context.db).await?;
    let pid = user.pid.to_string();

    assert!(user.reset_sent_at.is_none());
    assert!(user.reset_token.is_none());

    assert!(user
        .into_active_model()
        .set_forgot_password_sent(&boot.app_context.db)
        .await
        .is_ok());

    let user = Model::find_by_pid(&boot.app_context.db, &pid).await?;

    assert!(user.reset_sent_at.is_some());
    assert!(user.reset_token.is_some());

    Ok(())
}

#[tokio::test]
#[parallel]
async fn can_verified() -> anyhow::Result<()> {
    configure_insta!();

    let boot = boot_test::<App>().await?;
    let user = create_random_user(&boot.app_context.db).await?;
    let pid = user.pid.to_string();

    assert!(user.email_verified_at.is_none());

    assert!(user
        .into_active_model()
        .verified(&boot.app_context.db)
        .await
        .is_ok());

    let user = Model::find_by_pid(&boot.app_context.db, &pid).await?;

    assert!(user.email_verified_at.is_some());

    Ok(())
}

#[tokio::test]
#[parallel]
async fn can_reset_password() -> anyhow::Result<()> {
    configure_insta!();

    let boot = boot_test::<App>().await?;

    let password: String = Password(8..12).fake();
    let user = create_random_user_with_password(&boot.app_context.db, &password).await?;
    let pid = user.pid.to_string();

    assert!(user.verify_password(&password));

    assert!(user
        .clone()
        .into_active_model()
        .reset_password(&boot.app_context.db, "new-password")
        .await
        .is_ok());

    assert!(Model::find_by_pid(&boot.app_context.db, &pid)
        .await?
        .verify_password("new-password"));

    Ok(())
}

#[tokio::test]
#[parallel]
async fn magic_link() -> anyhow::Result<()> {
    let boot = boot_test::<App>().await?;

    let user = create_random_user(&boot.app_context.db).await?;
    let pid = user.pid.to_string();

    assert!(
        user.magic_link_token.is_none(),
        "Magic link token should be initially unset"
    );
    assert!(
        user.magic_link_expiration.is_none(),
        "Magic link expiration should be initially unset"
    );

    let create_result = user
        .into_active_model()
        .create_magic_link(&boot.app_context.db)
        .await;

    assert!(
        create_result.is_ok(),
        "Failed to create magic link: {:?}",
        create_result.unwrap_err()
    );

    let updated_user = Model::find_by_pid(&boot.app_context.db, &pid)
        .await
        .expect("Failed to refetch user after magic link creation");

    assert!(
        updated_user.magic_link_token.is_some(),
        "Magic link token should be set after creation"
    );

    let magic_link_token = updated_user.magic_link_token.unwrap();
    assert_eq!(
        magic_link_token.len(),
        users::MAGIC_LINK_LENGTH as usize,
        "Magic link token length does not match expected length"
    );

    assert!(
        updated_user.magic_link_expiration.is_some(),
        "Magic link expiration should be set after creation"
    );

    let now = Local::now();
    let should_expired_at = now + Duration::minutes(users::MAGIC_LINK_EXPIRATION_MIN.into());
    let actual_expiration = updated_user.magic_link_expiration.unwrap();

    assert!(
        actual_expiration >= now,
        "Magic link expiration should be in the future or now"
    );

    assert!(
        actual_expiration <= should_expired_at,
        "Magic link expiration exceeds expected maximum expiration time"
    );

    Ok(())
}
