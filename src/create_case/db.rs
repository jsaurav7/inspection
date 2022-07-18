use sqlx::Row;
use uuid::Uuid;

use crate::create_case::agent_info::AgentInfo;
use crate::create_case::create_case_req::CreateCaseRequest;
use crate::error::Error;
use crate::user_details::User;
use crate::DB;

impl DB {
    #[tracing::instrument(name = "db::add_case", skip(self, user, create_case_req))]
    pub async fn add_case(
        &self,
        user: &User,
        create_case_req: &CreateCaseRequest,
    ) -> Result<i32, Error> {
        let agent_info = match &create_case_req.agent_phone_number {
            None => self
                .get_default_agent(user.branch_id)
                .await
                .unwrap_or_default(),
            Some(phone_number) => match self.get_agent_info(phone_number).await {
                Ok(agent_info) => agent_info,
                Err(_) => self
                    .add_agent(phone_number, user.branch_id)
                    .await
                    .unwrap_or_default(),
            },
        };

        sqlx::query(INSERT_CASE)
            .bind(&create_case_req.customer_name)
            .bind(&create_case_req.customer_phone_number)
            .bind(&create_case_req.customer_email)
            .bind(agent_info.name)
            .bind(agent_info.phone)
            .bind(agent_info.email)
            .bind(&create_case_req.vehicle_number)
            .bind(&create_case_req.fuel_type)
            .bind(&create_case_req.vehicle_type)
            .bind(&create_case_req.inspection_number)
            .bind(&create_case_req.chassis_number)
            .bind(&create_case_req.engine_number)
            .bind(&create_case_req.inspection_by_wimwisure)
            .bind(&user.company_name)
            .bind(user.branch_id)
            .bind(&create_case_req.purpose_of_inspection)
            .bind(Uuid::new_v4().to_string())
            .bind(&create_case_req.callback_url)
            .bind(&create_case_req.communication_by_wimwisure)
            .execute(&self.0)
            .await
            .and_then(|r| Ok(r.last_insert_id() as i32))
            .map_err(|e| {
                error!("Error adding case: {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::add_payment_mode", skip(self))]
    pub async fn add_payment_mode(&self, case_id: i32, paid_by: &str) -> Result<(), Error> {
        sqlx::query("INSERT INTO payment_details (case_id, paid_by) VALUES (?, ?)")
            .bind(case_id)
            .bind(paid_by)
            .execute(&self.0)
            .await
            .and_then(|_| Ok(()))
            .map_err(|e| {
                error!("Error adding payment mode: {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::add_required_photos", skip(self))]
    pub async fn add_required_photos(
        &self,
        case_id: i32,
        vehicle_type: &str,
        fuel_type: &str,
        purpose_of_inspection: &str,
    ) -> Result<(), Error> {
        let required_photos = self
            .get_required_photos(vehicle_type, fuel_type, purpose_of_inspection)
            .await?;

        info!("{:#?}", required_photos);

        let transaction_id = Uuid::new_v4().to_string();

        for photo in required_photos {
            let _ = sqlx::query("INSERT INTO cases_photos_req (case_id, transaction_id, photo_type_id, enabled) VALUES (?, ?, ?, 1)")
                .bind(case_id)
                .bind(&transaction_id)
                .bind(photo)
                .execute(&self.0)
                .await
                .and_then(|_| Ok(()))
                .map_err(|e| {
                    error!("Error adding required photo: {:?}", e);
                    Error::Sqlx { source : e}
                });
        }

        Ok(())
    }

    #[tracing::instrument(name = "db::get_required_photos", skip(self))]
    async fn get_required_photos(
        &self,
        vehicle_type: &str,
        fuel_type: &str,
        purpose_of_inspection: &str,
    ) -> Result<Vec<String>, Error> {
        sqlx::query("SELECT photo_type FROM vehicle_photo_req_superset WHERE vehicle_type = ? AND fuel_type = ? AND purpose_of_inspection = ?")
            .bind(vehicle_type)
            .bind(fuel_type)
            .bind(purpose_of_inspection)
            .fetch_all(&self.0)
            .await
            .and_then(|rows| {
                Ok(rows
                    .iter()
                    .map(|row| row.get("photo_type"))
                    .collect())
            })
            .map_err(|e| {
                error!("Error fetching photos on hold: {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::add_notification_users", skip(self))]
    pub async fn add_notification_users(
        &self,
        case_id: i32,
        notification_to: &str,
        notification_type: &str,
    ) -> Result<(), Error> {
        sqlx::query("INSERT INTO cases_notifications (case_id, notification_to, notification_type) VALUES (?, ?, ?)")
            .bind(case_id)
            .bind(notification_to)
            .bind(notification_type)
            .execute(&self.0)
            .await
            .and_then(|_| Ok(()))
            .map_err(|e| {
                error!("Error saving notification user_details: {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::add_online_case", skip(self))]
    pub async fn add_online_case(
        &self,
        case_id: i32,
        quote_number: &str,
        vehicle_number: &str,
        chassis_number: &str,
        engine_number: &str,
    ) -> Result<(), Error> {
        sqlx::query("INSERT INTO online_cases (case_id, quote_number, vehicle_number, chassis_number, engine_number) VALUES (?, ?, ?, ?, ?)")
            .bind(case_id)
            .bind(quote_number)
            .bind(vehicle_number)
            .bind(chassis_number)
            .bind(engine_number)
            .execute(&self.0)
            .await
            .and_then(|_| Ok(()))
            .map_err(|e| {
                error!("Error saving online case: {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::add_case_allocation", skip(self))]
    pub async fn add_case_allocation(
        &self,
        case_id: i32,
        username: &str,
        inspection_link: &str,
    ) -> Result<(), Error> {
        sqlx::query("INSERT INTO cases_user_allocation (case_id, user_id, inspection_link, stage) VALUES (?, ?, ?, 3)")
            .bind(case_id)
            .bind(username)
            .bind(inspection_link)
            .execute(&self.0)
            .await
            .and_then(|_| Ok(()))
            .map_err(|e| {
                error!("Error saving notification user_details: {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::add_permission", skip(self))]
    pub async fn add_permission(&self, username: &str, role: &str) -> Result<(), Error> {
        sqlx::query("INSERT INTO authorities VALUES (?, ?)")
            .bind(username)
            .bind(role)
            .execute(&self.0)
            .await
            .and_then(|_| Ok(()))
            .map_err(|e| {
                error!("Error saving permission: {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::add_customer", skip(self))]
    pub async fn add_customer(&self, phone_number: &str) -> Result<String, Error> {
        let username = Uuid::new_v4().to_string();

        sqlx::query(INSERT_CUSTOMER)
            .bind(&username)
            .bind(phone_number)
            .bind(phone_number)
            .execute(&self.0)
            .await
            .and_then(|_| Ok(()))
            .map_err(|e| {
                error!("Error saving customer: {:?}", e);
                Error::Sqlx { source: e }
            })?;

        let _ = self.add_permission(&username, "ROLE_CUSTOMER").await?;

        Ok(username)
    }

    #[tracing::instrument(name = "db::get_username", skip(self))]
    pub async fn get_username(&self, phone_number: &str) -> Result<String, Error> {
        sqlx::query("SELECT username FROM users WHERE phone_number = ?")
            .bind(phone_number)
            .fetch_one(&self.0)
            .await
            .and_then(|r| Ok(r.get("username")))
            .map_err(|e| {
                error!("Error fetching username: {:?}", e);
                Error::Sqlx { source: e }
            })
    }

    #[tracing::instrument(name = "db::get_agent_info", skip(self))]
    pub async fn get_agent_info(&self, phone_number: &str) -> Result<AgentInfo, Error> {
        sqlx::query_as::<_, AgentInfo>("SELECT first_name as name, phone_number as phone, email FROM users WHERE phone_number = ?")
            .bind(phone_number)
            .fetch_one(&self.0)
            .await
            .map_err(|e| {
                error!("Error fetching agent info: {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::get_default_agent", skip(self))]
    pub async fn get_default_agent(&self, branch_id: i32) -> Result<AgentInfo, Error> {
        let phone_number: String =
            sqlx::query("SELECT default_agent as phone_number from company_branch_division where id = ?")
                .bind(branch_id)
                .fetch_one(&self.0)
                .await
                .and_then(|r| Ok(r.get("phone_number")))
                .map_err(|e| {
                    error!("Error fetching default agent: {:?}", e);
                    Error::Sqlx { source: e }
                })?;

        self.get_agent_info(&phone_number).await
    }

    #[tracing::instrument(name = "db::add_agent", skip(self))]
    pub async fn add_agent(&self, phone_number: &str, branch_id: i32) -> Result<AgentInfo, Error> {
        let username = Uuid::new_v4().to_string();

        sqlx::query(INSERT_AGENT_INFO)
            .bind(&username)
            .bind(phone_number)
            .bind(phone_number)
            .execute(&self.0)
            .await
            .map_err(|e| {
                error!("Error saving agent info: {:?}", e);
                Error::Sqlx { source: e }
            })?;

        sqlx::query(
            "INSERT INTO agent_details (username, company_branch_division_id) VALUES (?, ?)",
        )
        .bind(&username)
        .bind(branch_id)
        .execute(&self.0)
        .await
        .map_err(|e| {
            error!("Error saving agent details: {:?}", e);
            Error::Sqlx { source: e }
        })?;

        let _ = self.add_permission(&username, "ROLE_AGENT").await?;

        Ok(AgentInfo {
            name: "".to_string(),
            phone: phone_number.to_string(),
            email: "invalid@wimwisure.com".to_string(),
        })
    }
}

const INSERT_CASE: &str = r#"
INSERT INTO cases(customer_name,
                  customer_phone,
                  customer_email,
                  requestor_name,
                  requestor_phone,
                  requestor_email,
                  vehicle_number,
                  vehicle_fuel_type,
                  vehicle_type_id,
                  inspection_number,
                  chassis_number,
                  engine_number,
                  inspection_by_wimwisure,
                  inspection_type,
                  insurance_company_id,
                  current_stage,
                  case_type,
                  inspection_time,
                  company_branch_division_id,
                  inspection_stage,
                  vehicle_yom,
                  purpose_of_inspection,
                  download_key,
                  callback_url,
                  app_version,
                  inspection_latitude,
                  inspection_longitude,
                  communication)
values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'ASSIGN_TO_CUSTOMER', ?, 3, 1, ADDTIME(now(), '05:30:00'), ?, 0, 9999, ?, ?, ?, 'v2', 0.0, 0.0, ?)"#;

const INSERT_AGENT_INFO: &str = r#"
INSERT INTO users
    (username,
     password,
     phone_number,
     email,
     phone_number_verified,
     email_verified,
     password_change_required,
     first_name,
     last_name,
     user_type,
     full_name
) VALUES (?, ?, ?, 'invalid@wimwisure.com', 1, 1, 0, '', '', 'AGENT', '')"#;

const INSERT_CUSTOMER: &str = r#"
INSERT INTO users 
    (username,
     password,
     phone_number,
     email,
     phone_number_verified,
     email_verified,
     password_change_required,
     first_name,
     last_name,
     user_type,
     full_name
) VALUES (?, ?, ?, 'invalid@wimwisure.com', 1, 1, 0, '', '', 'CUSTOMER', '')"#;