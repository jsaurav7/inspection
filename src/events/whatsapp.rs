use serde::Serialize;

#[derive(Serialize)]
pub struct WhatsappMessage<'r> {
    from: Phone<'r>,
    to: Vec<Phone<'r>>,
    data: MessageData<'r>,
}

#[derive(Serialize)]
struct Phone<'r> {
    phone_number: &'r str,
}

#[derive(Serialize)]
struct MessageData<'r> {
    message_template: MessageTemplate<'r>,
}

#[derive(Serialize)]
struct MessageTemplate<'r> {
    storage: &'static str,
    namespace: &'static str,
    template_name: &'static str,
    language: MessageLanguage,
    rich_template_data: TemplateData<'r>,
}

impl<'r> MessageTemplate<'r> {
    fn new(
        template_name: &'static str,
        parameters: Vec<&'r str>,
        header: Option<TemplateHeader>,
    ) -> Self {
        Self {
            storage: "none",
            namespace: "ba6479d9_c8bb_4f6c_a2fa_7117c77f009d",
            template_name,
            language: MessageLanguage::new(),
            rich_template_data: TemplateData::new(parameters, header),
        }
    }
}

#[derive(Serialize)]
struct TemplateData<'r> {
    body: TemplateParameter<'r>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<TemplateHeader>,
}

impl<'r> TemplateData<'r> {
    fn new(parameters: Vec<&'r str>, header: Option<TemplateHeader>) -> Self {
        Self {
            body: TemplateParameter::new(parameters),
            header,
        }
    }
}

#[derive(Serialize)]
pub struct TemplateHeader {
    #[serde(rename = "type")]
    pub header_type: String,
    pub media_url: String,
}

#[derive(Serialize)]
struct TemplateParameter<'r> {
    params: Vec<TemplateParameterData<'r>>,
}

#[derive(Serialize)]
struct TemplateParameterData<'r> {
    data: &'r str,
}

impl<'r> TemplateParameter<'r> {
    fn new(parameters: Vec<&'r str>) -> Self {
        Self {
            params: parameters
                .into_iter()
                .map(|param| TemplateParameterData { data: param })
                .collect(),
        }
    }
}

#[derive(Serialize)]
struct MessageLanguage {
    policy: &'static str,
    code: &'static str,
}

impl MessageLanguage {
    fn new() -> Self {
        Self {
            policy: "deterministic",
            code: "en",
        }
    }
}

impl<'r> WhatsappMessage<'r> {
    pub fn new(
        template_name: &'static str,
        to: Vec<&'r str>,
        parameters: Vec<&'r str>,
        header: Option<TemplateHeader>,
    ) -> Self {
        Self {
            from: Phone {
                phone_number: "+918755754394",
            },
            to: to
                .into_iter()
                .map(|phone| Phone {
                    phone_number: phone,
                })
                .collect(),
            data: MessageData {
                message_template: MessageTemplate::new(template_name, parameters, header),
            },
        }
    }
}
