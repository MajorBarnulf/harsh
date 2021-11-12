use axum::extract::{Extension, Json};
use axum::routing::get;
use axum::Router;

use crate::lib::harsh::SharedState;

use super::models::*;

pub fn setup_routes(app: Router) -> Router {
	app.route("/user/create", get(create_user))
}

async fn create_user(Extension(state): Extension<SharedState>, Json(payload): Json<CreateUser>) {
	//
}
