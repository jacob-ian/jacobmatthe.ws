use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    HttpResponse, Responder,
};

pub struct HtmlResponseBuilder {
    response: HtmlResponse,
}

impl HtmlResponseBuilder {
    pub fn status(&mut self, status: StatusCode) -> &mut Self {
        self.response.status = status;
        return self;
    }
    pub fn title(&mut self, title: String) -> &mut Self {
        self.response.head.title = title;
        return self;
    }
    pub fn description(&mut self, description: String) -> &mut Self {
        self.response.head.description = description;
        return self;
    }
    pub fn body(&mut self, body: String) -> &mut Self {
        self.response.body = body;
        return self;
    }
    pub fn build(&self) -> HtmlResponse {
        return self.response.clone();
    }
}

#[derive(Clone)]
pub struct Head {
    pub title: String,
    pub description: String,
}

#[derive(Clone)]
pub struct HtmlResponse {
    status: StatusCode,
    head: Head,
    body: String,
}

impl HtmlResponse {
    pub fn builder() -> HtmlResponseBuilder {
        return HtmlResponseBuilder {
            response: HtmlResponse {
                status: StatusCode::OK,
                head: Head {
                    title: String::from("jacobmatthe.ws"),
                    description: String::from("The blog of Jacob Matthews"),
                },
                body: String::new(),
            },
        };
    }
}

impl Responder for HtmlResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        return HttpResponse::build(self.status)
            .content_type(ContentType::html())
            .body(format!(
                r#"
                <!DOCTYPE html>
                <html>
                    <head>
                        <meta charset="utf-8">
                        <meta name="viewport" content="width=device-width, initial-scale=1" />
                        <title>{title}</title>
                        <meta name="description" content="{description}" />
                    </head>
                    <body>{body}</body>
                </html>
                "#,
                title = self.head.title,
                description = self.head.description,
                body = self.body
            ));
    }
}
