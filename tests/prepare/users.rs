use fake::{
    faker::{
        internet::en::{FreeEmail, Password},
        name::en::Name,
    },
    Fake,
};
use loco_nuxt_template::models::users::{self, Model, RegisterParams};
use loco_rs::prelude::*;

pub async fn create_random_user(db: &DatabaseConnection) -> anyhow::Result<users::Model> {
    let registration_params = RegisterParams {
        email: FreeEmail().fake(),
        password: Password(8..20).fake(),
        name: Name().fake(),
    };

    let user = Model::create_with_password(&db, &registration_params).await?;

    Ok(user)
}

pub async fn create_random_user_with_password(
    db: &DatabaseConnection,
    password: &str,
) -> anyhow::Result<users::Model> {
    let registration_params = RegisterParams {
        email: FreeEmail().fake(),
        password: password.to_string(),
        name: Name().fake(),
    };

    let user = Model::create_with_password(&db, &registration_params).await?;

    Ok(user)
}
