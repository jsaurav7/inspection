use itertools::Itertools;
use rocket::State;

use crate::case_details::case_info::CaseInfo;
use crate::create_case::create_case_req::CreateCaseRequest;
use crate::error::Error;
use crate::user_details::User;
use crate::{AppState, DB, events};

#[tracing::instrument(name = "service::create_case", skip(db, state, create_case_req, user))]
pub async fn create_case(
    db: &DB,
    state: &State<AppState>,
    user: User,
    create_case_req: CreateCaseRequest,
) -> Result<CaseInfo, Error> {
    let case_id = db.add_case(&user, &create_case_req).await?;

    db.add_payment_mode(case_id, &create_case_req.paid_by)
        .await?;

    db.add_required_photos(
        case_id,
        &create_case_req.vehicle_type,
        &create_case_req.fuel_type,
        &create_case_req.purpose_of_inspection,
    )
    .await?;
    db.add_online_case(
        case_id,
        &create_case_req.quote_number,
        &create_case_req.vehicle_number,
        &create_case_req.chassis_number,
        &create_case_req.engine_number,
    )
    .await?;

    for phone in &create_case_req.notify_phone {
        db.add_notification_users(case_id, phone, "PHONE").await?;
    }

    for email in &create_case_req.notify_email {
        db.add_notification_users(case_id, email, "EMAIL").await?;
    }

    let customer_phone = create_case_req.customer_phone_number.as_str();
    let phone_numbers_to_allocate: Vec<&str> = create_case_req
        .inspectors
        .iter()
        .map(|inspector| inspector.as_str())
        .chain(vec![customer_phone].into_iter())
        .unique()
        .collect();

    for phone in phone_numbers_to_allocate {
        let _ = allocation_case(db, state, case_id, phone).await;
    }

    let _ = events::service::handle_event(db, state, case_id, "CREATE.CASE".to_string()).await;

    let case_info = db.get_case_info(&user, case_id).await?;

    Ok(case_info)
}

#[tracing::instrument(name = "service::allocate_case", skip(db, state))]
async fn allocation_case(
    db: &DB,
    state: &State<AppState>,
    case_id: i32,
    phone_number: &str,
) -> Result<(), Error> {
    let username = match db.get_username(phone_number).await {
        Ok(username) => username,
        Err(_) => db.add_customer(phone_number).await?,
    };

    let inspection_link = state
        .http_client
        .get_inspection_link(case_id, &username)
        .await
        .unwrap_or_default();

    db.add_case_allocation(case_id, &username, &inspection_link)
        .await
}
