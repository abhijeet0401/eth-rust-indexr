use actix_web::{web, HttpResponse, Responder};

pub async fn get(app_data: web::Data<crate::AppState>) -> impl Responder {
    let result = app_data
        .service_container
        .lock()
        .unwrap()
        .user
        .get_all_users()
        .await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
