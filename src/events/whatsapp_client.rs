use crate::clients::whatsapp_client::WhatsappClient;
use crate::error::Error;
use crate::events::customer_info::CustomerInfo;
use crate::events::whatsapp::WhatsappMessage;
use crate::models::inspector::Inspector;

impl WhatsappClient {
    #[tracing::instrument(
        name = "Send Create Case Message",
        skip(self, inspectors, customer_info)
    )]
    pub async fn send_create_case_whatsapp(
        &self,
        inspectors: &Vec<Inspector>,
        customer_info: &CustomerInfo,
    ) -> Result<(), Error> {
        for inspector in inspectors {
            let inspector_phone = format_phone(&inspector.phone_number);

            let params = vec![
                customer_info.name.as_str(),
                customer_info.vehicle_number.as_str(),
                customer_info.insurance_company.as_str(),
                inspector.inspection_link.as_str(),
                "https://youtu.be/W0-XVrW-SiE",
                customer_info.support_phone.as_str(),
                "www.wimwisure.com",
            ];

            let message = WhatsappMessage::new(
                "case_create_post_cust_2_",
                vec![&inspector_phone],
                params,
                None,
            );

            self.send_whatsapp_message(message).await?;
        }

        Ok(())
    }

    #[tracing::instrument(name = "Send upload Case Message", skip(self, customer_info))]
    pub async fn send_upload_case_whatsapp(
        &self,
        customer_info: &CustomerInfo,
    ) -> Result<(), Error> {
        let params = vec![
            customer_info.name.as_str(),
            customer_info.vehicle_number.as_str(),
            customer_info.support_phone.as_str(),
            "www.wimwisure.com",
        ];

        let message =
            WhatsappMessage::new("case_uploaded", vec![&customer_info.phone], params, None);

        self.send_whatsapp_message(message).await
    }

    #[tracing::instrument(name = "Send Whatsapp Message", skip(self, message))]
    async fn send_whatsapp_message(&self, message: WhatsappMessage<'_>) -> Result<(), Error> {
        self.client
            .post("https://api.freshchat.com/v2/outbound-messages/whatsapp")
            .header("Authorization", &self.fresh_chat_auth_token)
            .json(&message)
            .send()
            .await
            .map(|_| ())
            .map_err(|e| {
                error!("Error sending whatsapp message: {}", e);
                e.into()
            })
    }
}

fn format_phone(phone: &str) -> String {
    if phone.len() < 10 {
        return phone.to_string();
    }

    let phone = &phone[..10];

    format!("+91{}", phone)
}
