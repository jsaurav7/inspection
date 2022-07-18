use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::Row;

use crate::db::DB;
use crate::error::Error;
use crate::events::customer_info::CustomerInfo;

impl DB {
    #[tracing::instrument(name = "db::get_case_customer_info", skip(self))]
    pub async fn get_case_customer_info(&self, case_id: i32) -> Result<CustomerInfo, Error> {
        sqlx::query_as::<_, CustomerInfo>(r#"
            select customer_name as name, customer_phone as phone, vehicle_number, message_key as insurance_company, ic.phone_number as support_phone
            from cases join insurance_companies ic on cases.insurance_company_id = ic.id
            where cases.id = ? and cases.app_version = 'v2'
        "#)
            .bind(case_id)
            .fetch_one(&self.0)
            .await
            .map_err(|e| {
                error!("Error fetching case customer info: {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::get_case_inspector_phone", skip(self))]
    pub async fn get_case_inspector_phone(&self, case_id: i32) -> Result<String, Error> {
        sqlx::query("SELECT phone_number FROM cases LEFT JOIN users ON username = inspection_by WHERE id = ?")
            .bind(case_id)
            .fetch_one(&self.0)
            .await
            .and_then(|r| Ok(r.try_get(0)?))
            .map_err(|e| {
                error!("Error fetching case inspector phone: {:?}", e);
                e.into()
            })
    }
}
