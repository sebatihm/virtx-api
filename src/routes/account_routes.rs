use actix_web::{delete, get, middleware::from_fn, post, put, web::{self, scope, ServiceConfig}, HttpMessage, HttpRequest, HttpResponse};
use entity::user::Model;
use sea_orm::{ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};
use sha256::digest;
use crate::{middleware, utils::{app_state::AppState, jwt::Claims} };
use sea_orm::ActiveModelTrait;
use sea_orm::ModelTrait;




pub fn config (config: &mut ServiceConfig){
    config.service(scope("/account")
        .wrap(from_fn(middleware::auth_middleware::check_auth_middleware))
        .service(get_account_info)
        .service(update_account_info)
        .service(change_password)
        .service(delete_account)
    );
}

async fn find_auth_user(req: &HttpRequest, app_state: &AppState) -> Option<Model>{
    let account_id = req.extensions().get::<Claims>().unwrap().id;
    let auth_user = entity::user::Entity::find_by_id(account_id).one(&app_state.db).await.unwrap();
    

    return auth_user;
}

#[get("")]
pub async fn get_account_info(req: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse{
    let user_account = find_auth_user(&req, &app_state).await;

    if user_account == None{
        return HttpResponse::NotFound().body("The user wasn't found");
    }else{
        return HttpResponse::Ok().json(user_account.unwrap());
    }
}

#[derive(Serialize,Deserialize)]
pub struct AccountForm{
    pub name: String,
    pub password: String
}

#[put("")]
pub async fn update_account_info(req: HttpRequest, account_json: web::Json<AccountForm>, app_state: web::Data<AppState>) -> HttpResponse{
    let user_account = find_auth_user(&req, &app_state).await;

    if user_account == None{
        return HttpResponse::NotFound().body("The user wasn't found");
    }else{

        if digest(account_json.password.clone()) != user_account.clone().unwrap().password{
            return HttpResponse::Unauthorized()
                .body("The credentials do not match");
        }
        let mut account: entity::user::ActiveModel = user_account.unwrap().into();

        if account_json.name != "" {
            account.name = Set(account_json.name.to_owned());
        }

        let account: entity::user::Model = account.update(&app_state.db).await.unwrap();

        return HttpResponse::Ok()
            .json(account);
    }

}

#[derive(Serialize,Deserialize)]
pub struct PasswordResetForm{
    pub old_password: String,
    pub new_password: String
}

#[post("/password-reset")]
pub async fn change_password(req: HttpRequest, pwd_reset_json: web::Json<PasswordResetForm>, app_state: web::Data<AppState>) -> HttpResponse{
    let user_account = find_auth_user(&req, &app_state).await;

    if user_account == None{
        return HttpResponse::NotFound().body("The user wasn't found");
    }else{
        let user_account = user_account.unwrap();
        
        if user_account.password != digest(pwd_reset_json.old_password.clone()) {
            return HttpResponse::Unauthorized().body("The passwords dont match");
        }

        let mut user_account: entity::user::ActiveModel = user_account.into();
        user_account.password = Set(digest(pwd_reset_json.new_password.clone()));

        let user_account:entity::user::Model = user_account.update(&app_state.db).await.unwrap();

        return HttpResponse::Ok()
            .json(user_account);        
    }
}

#[delete("")]
pub async fn delete_account(req: HttpRequest, app_state: web::Data<AppState>) -> HttpResponse{
    let user_account = find_auth_user(&req, &app_state).await;

    if user_account == None{
        return HttpResponse::NotFound().body("The user wasn't found");
    }else{
        let user_account = user_account.unwrap();
        let delete_operation = user_account.delete(&app_state.db).await.unwrap();

        if delete_operation.rows_affected == 1 {
            return HttpResponse::NoContent().body("The user was deleted succesfully");
        }else {
            return HttpResponse::InternalServerError()
                .body("Something went wrong, while deleting the user");
        }
    }
}