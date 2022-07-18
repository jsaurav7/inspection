use crate::clients::sms_client::SmsClient;
use crate::error::Error;
use crate::events::customer_info::CustomerInfo;
use crate::models::inspector::Inspector;

impl SmsClient {
    #[tracing::instrument(name = "Send sms", skip(self, inspectors, customer_info))]
    pub async fn send_create_case_sms(
        &self,
        inspectors: &Vec<Inspector>,
        customer_info: &CustomerInfo,
    ) -> Result<(), Error> {
        for inspector in inspectors {
            let message = format!("Dear Customer, Self Inspection request received for vehicle {} from {}. Download application from {} to complete self inspection in next 24 hours. The policy will start only after successful inspection. Call {} for any support.", customer_info.vehicle_number, customer_info.insurance_company, inspector.inspection_link, customer_info.support_phone);

            self
                .client
                .get(format!("https://smsapi.24x7sms.com/api_2.0/SendSMS.aspx?APIKey={}&ServiceName=TEMPLATE_BASED&SenderID=WSSURE&MobileNo={}&Message={message}&DLTTemplateID=1107159853951691105", self.api_key, inspector.phone_number))
                .send()
                .await
                .map(|_| ())
                .map_err(|e| {
                    error!("Error sending sms: {}", e);
                    e
                })?;
        }

        Ok(())
    }
}
