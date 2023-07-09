use rocket::{
    fs::NamedFile,
    response::status::NotFound,
    serde::{Deserialize, Serialize},
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::entity::*;
use rocket::{serde::json::Json, State};
use sea_orm::{
    sea_query::Query, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    Select,
};

extern crate rocket;

// TODO: sanitize dangerous characters
#[get("/<path>")]
pub async fn serve_files(path: PathBuf) -> Result<NamedFile, std::io::Error> {
    let static_folder = std::env::var("STATIC_FOLDER").unwrap();

    let file = NamedFile::open(Path::new(&static_folder).join(path)).await;
    info!("FILE!! {:?}", file);
    file
}

#[get("/event")]
pub async fn get_events(db: &State<DatabaseConnection>) -> Json<Vec<event::Model>> {
    info!("GET /event hit");

    let db = db as &DatabaseConnection;

    let events = event::Entity::find().all(db).await.unwrap();

    info!("Returning {} events from GET /event", events.len());

    Json(events)
}

#[get("/session/current")]
pub async fn get_current_session(db: &State<DatabaseConnection>) -> Json<session::Model> {
    info!("GET /session/current hit");

    let db = db as &DatabaseConnection;

    let session = current_session(&db).await.unwrap();

    info!(
        "Returning {:?} current session from GET /session/current",
        session
    );

    Json(session)
}

#[get("/session/current/statistics")]
pub async fn get_current_session_statistics(
    db: &State<DatabaseConnection>,
) -> Result<Json<SessionStatisticsResponse>, NotFound<String>> {
    info!("GET /session/current/statistics hit");

    let db = db as &DatabaseConnection;

    let session = current_session_query()
        .find_with_related(event::Entity)
        .all(db)
        .await
        .unwrap();

    let (session, events) = &session[0];

    if events.is_empty() {
        return Err(NotFound("No events found for current session".to_owned()));
    };

    let mut time_per_app: HashMap<String, u32> = HashMap::new();

    let total_time_in_apps = events.last().unwrap().offset - events.first().unwrap().offset;

    let avg_time_in_app = total_time_in_apps / (events.len() as u32);

    let mut prev_offset = 0;
    let mut prev_app = String::from("");
    let mut prev_entry = 0;

    for event in events {
        if !prev_app.is_empty() {
            let spent = event.offset - prev_offset;
            time_per_app.insert(prev_app.clone(), prev_entry + spent);
        };

        let entry = *time_per_app.entry(event.app_title.clone()).or_insert(0);

        prev_offset = event.offset;
        prev_app = event.app_title.clone();
        prev_entry = entry;
    }

    let mut time_per_app: Vec<(String, u32)> = time_per_app
        .into_iter()
        .map(|(key, val)| (key, val))
        .collect();

    time_per_app.sort_by(|a, b| b.1.cmp(&a.1));

    Ok(Json(SessionStatisticsResponse {
        session: session.clone(),
        time_per_app,
        avg_time_in_app,
        total_time_in_apps,
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SessionStatisticsResponse {
    session: session::Model,
    time_per_app: Vec<(String, u32)>,
    avg_time_in_app: u32,
    total_time_in_apps: u32,
}

async fn current_session(db: &DatabaseConnection) -> Result<session::Model, DbErr> {
    let session = current_session_query().one(db).await?.unwrap();
    Ok(session)
}

fn current_session_query() -> Select<session::Entity> {
    session::Entity::find().filter(
        Condition::any().add(
            session::Column::Id.in_subquery(
                Query::select()
                    .expr(session::Column::Id.max())
                    .from(session::Entity)
                    .to_owned(),
            ),
        ),
    )
}
