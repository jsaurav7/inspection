use lettre::{AsyncTransport, Message};
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::message::header::ContentType;
use crate::clients::mail_client::MailClient;

use crate::error::Error;

impl MailClient {
    fn mailbox(&self, emails: Vec<String>) -> String {
        emails
            .into_iter()
            .filter(|email| !email.is_empty())
            .map(|email| format!("<{email}>"))
            .collect::<Vec<String>>()
            .join("")
    }

    #[tracing::instrument(name = "Send Mail Template", skip(self, to))]
    async fn send_mail_template(
        &self,
        to: Vec<String>,
        subject: String,
        html: String,
    ) -> Result<(), Error> {
        let to = self.mailbox(to);

        if to.is_empty() {
            return Ok(())
        }

        let message = Message::builder()
            .from("support@wimwisure.com".parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .multipart(MultiPart::mixed().singlepart(SinglePart::html(html)))
            .unwrap();

        self.client.send(message)
            .await
            .map(|_| ())
            .map_err(|e| {
                error!("Error sending email: {}", e);
                e.into()
            })
    }

    #[tracing::instrument(name = "Send Hold Case Mail", skip(self, to))]
    pub async fn send_hold_case_mail(
        &self,
        to: Vec<String>,
        case_id: i32,
        vehicle_number: &str,
    ) -> Result<(), Error> {
        self.send_mail_template(
            to,
            format!("Inspection Case {case_id} on hold for Vehicle: {vehicle_number}"),
            format!("<html><body>Hi, <br/> <br/> Inspection Case {case_id} on hold for Vehicle: {vehicle_number} </body></html>"),
        ).await
    }

    #[tracing::instrument(name = "Send Inspection Completed Mail", skip(self, to))]
    pub async fn send_inspection_complete_mail(
        &self,
        to: Vec<String>,
        case_id: i32,
        vehicle_number: &str,
        inspection_by: &str,
        ) -> Result<(), Error> {
        self.send_mail_template(
            to,
            format!("Inspection completed for Case {case_id}, Vehicle number {vehicle_number}"),
            format!("<html><body>Hi, <br/> <br/> Inspection completed for Case {case_id}, Vehicle number {vehicle_number} by {inspection_by} </body></html>"),
        ).await
    }

    #[tracing::instrument(name = "Send Case Closed Mail", skip(self, to))]
    pub async fn send_case_closed_mail(
        &self,
        to: Vec<String>,
        case_id: i32,
        vehicle_number: &str,
    ) -> Result<(), Error> {
        self.send_mail_template(
            to,
            format!("Inspection Case {case_id} closed for Vehicle: {vehicle_number}"),
            format!("<html><body>Hi, <br/> <br/>  Inspection Case {case_id} closed for Vehicle: {vehicle_number} </body></html>"),
        ).await
    }

    #[tracing::instrument(name = "Send Case Created Mail", skip(self, to))]
    pub async fn send_case_created_mail(
        &self,
        to: Vec<String>,
        case_id: i32,
        vehicle_number: &str,
    ) -> Result<(), Error> {
        self.send_mail_template(
            to,
            format!("Inspection Case {case_id} created for Vehicle: {vehicle_number}"),
            format!("<html><body>Hi, <br/> <br/> Inspection Case {case_id} has been created for Vehicle: {vehicle_number} </body></html>"),
        ).await
    }

    #[tracing::instrument(name = "Send QC Completed Mail", skip(self, report))]
    pub async fn send_qc_completed_mail(
        &self,
        report: Vec<u8>,
        to: Vec<String>,
        case_id: i32,
        vehicle_number: &str,
    ) -> Result<(), Error> {
        let report = Attachment::new("report.pdf".to_string())
            .body(report, ContentType::parse("application/pdf").unwrap());

        let to = self.mailbox(to);

        if to.is_empty() {
            return Ok(());
        }

        let message = Message::builder()
            .from("support@wimwisure.com".parse().unwrap())
            .to(to.parse().unwrap())
            .subject(format!("QC completed for Inspection Case {case_id}, Vehicle: {vehicle_number}"))
            .multipart(
                MultiPart::mixed()
                    .singlepart(report)
                    .singlepart(SinglePart::html(format!("<html><body>Hi, <br/> <br/> QC completed for Inspection Case {case_id}, Vehicle: {vehicle_number} </body></html>")))
            )
            .unwrap();

        self.client.send(message)
            .await
            .map(|_| ())
            .map_err(|e| {
                error!("Error sending email: {}", e);
                e.into()
            })
    }
}
