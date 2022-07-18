use rocket::{State, tokio};

use crate::{AppState, DB};
use crate::error::Error;
use crate::events::customer_info::CustomerInfo;
use crate::models::notification_users::NotificationUsers;
use crate::models::inspector::Inspector;
use crate::models::status::Status;

struct EventDetails {
    case_id: i32,
    inspector_phone: String,
    inspectors: Vec<Inspector>,
    customer_info: CustomerInfo,
    notification_users: NotificationUsers,
}

#[tracing::instrument(name = "service::handle_event", skip(db, state))]
pub async fn handle_event(
    db: &DB,
    state: &State<AppState>,
    case_id: i32,
    event_type: String,
) -> Result<Status, Error> {
    let event_details = get_event_details(db, case_id).await?;
    let callback_details = db.get_callback_details(case_id).await.ok();
    let state = state.inner().clone();

    tokio::spawn(async move {
        if let Some(callback_details) = callback_details {
            let _ = state.http_client.send_callback(callback_details).await;
        }

        let _ = match event_type.as_str() {
            "CREATE.CASE" => case_create(event_details, state).await,
            "INSPECTION_COMPLETED.CASE" => inspection_completed(event_details, state).await,
            "QC_COMPLETED.CASE" => case_completed(event_details, state).await,
            "CLOSE.CASE" => case_close(event_details, state).await,
            "HOLD.CASE" => case_hold(event_details, state).await,
            _ => Ok(()),
        };
    });

    Ok(Status::success())
}

#[tracing::instrument(name = "service::get_event", skip(db))]
async fn get_event_details(db: &DB, case_id: i32) -> Result<EventDetails, Error> {
    Ok(EventDetails {
        case_id,
        customer_info: db.get_case_customer_info(case_id).await?,
        inspectors: db.get_case_inspectors(case_id).await?,
        notification_users: db.get_notification_users(case_id).await?,
        inspector_phone: db
            .get_case_inspector_phone(case_id)
            .await
            .unwrap_or("".to_string()),
    })
}

#[tracing::instrument(name = "service::case_create", skip(event_details, state))]
async fn case_create(event_details: EventDetails, state: AppState) -> Result<(), Error> {
    let _ = state
        .mailer
        .send_case_created_mail(
            event_details.notification_users.emails,
            event_details.case_id,
            &event_details.customer_info.vehicle_number,
        )
        .await;

    let _ = state
        .whatsapp_client
        .send_create_case_whatsapp(&event_details.inspectors, &event_details.customer_info)
        .await;

    state
        .sms_client
        .send_create_case_sms(&event_details.inspectors, &event_details.customer_info)
        .await
}

#[tracing::instrument(name = "service::inspection_completed", skip(event_details, state))]
async fn inspection_completed(event_details: EventDetails, state: AppState) -> Result<(), Error> {
    let _ = state
        .mailer
        .send_inspection_complete_mail(
            event_details.notification_users.emails,
            event_details.case_id,
            &event_details.customer_info.vehicle_number,
            &event_details.inspector_phone,
        )
        .await;

    state
        .whatsapp_client
        .send_upload_case_whatsapp(&event_details.customer_info)
        .await
}

#[tracing::instrument(name = "service::case_completed", skip(event_details, state))]
async fn case_completed(event_details: EventDetails, state: AppState) -> Result<(), Error> {
    let report = state
        .s3_client
        .get_file(format!(
            "cases/{case_id}/{case_id}.pdf",
            case_id = event_details.case_id
        ))
        .await?;

    state
        .mailer
        .send_qc_completed_mail(
            report,
            event_details.notification_users.emails,
            event_details.case_id,
            &event_details.customer_info.vehicle_number,
        )
        .await
}

#[tracing::instrument(name = "service::case_hold", skip(event_details, state))]
async fn case_hold(event_details: EventDetails, state: AppState) -> Result<(), Error> {
    state
        .mailer
        .send_hold_case_mail(
            event_details.notification_users.emails,
            event_details.case_id,
            &event_details.customer_info.vehicle_number,
        )
        .await
}

#[tracing::instrument(name = "service::case_close", skip(event_details, state))]
async fn case_close(event_details: EventDetails, state: AppState) -> Result<(), Error> {
    state
        .mailer
        .send_case_closed_mail(
            event_details.notification_users.emails,
            event_details.case_id,
            &event_details.customer_info.vehicle_number,
        )
        .await
}
