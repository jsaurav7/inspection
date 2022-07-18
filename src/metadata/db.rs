use rocket_db_pools::sqlx;

use crate::db::DB;
use crate::error::Error;
use crate::metadata::case_metadata::{CaseMetadata, Metadata};

impl DB {
    #[tracing::instrument(name = "db::get_case_metadata", skip(self))]
    pub async fn get_case_metadata(&self, company: &str) -> Result<CaseMetadata, Error> {
        Ok(CaseMetadata {
            fuel_types: self.get_metadata("SELECT id, name FROM vehicle_fuel_types").await?,
            vehicle_types: self.get_metadata("SELECT id, name FROM vehicle_types").await?,
            purpose_of_inspections: self.get_purpose_of_inspections(company).await?,
            payment_methods: self.get_payment_methods().await?,
        })
    }

    #[tracing::instrument(name = "db::get_metadata", skip(self))]
    pub async fn get_metadata(&self, query: &str) -> Result<Vec<Metadata>, Error> {
        sqlx::query_as::<_, Metadata>(query)
            .fetch_all(&self.0)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch metadata {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::get_purpose_of_inspections", skip(self))]
    pub async fn get_purpose_of_inspections(&self, company: &str) -> Result<Vec<Metadata>, Error> {
        sqlx::query_as::<_, Metadata>("SELECT id, name FROM purpose_of_inspections_v2 WHERE insurance_company_id = ?")
            .bind(company)
            .fetch_all(&self.0)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch purpose of inspection {:?}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "db::get_payment_methods", skip(self))]
    pub async fn get_payment_methods(&self) -> Result<Vec<Metadata>, Error> {
        Ok(vec![
            Metadata {
                id: "PAID_BY_COMPANY".to_string(),
                name: "Paid by Company".to_string(),
            },
            Metadata {
                id: "PAID_BY_CUSTOMER".to_string(),
                name: "Paid by Customer".to_string(),
            },
            Metadata {
                id: "PAID_BY_AGENT".to_string(),
                name: "Paid by Agent".to_string(),
            },
        ])
    }
}
