use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    HttpResponse, Responder,
};

use crate::templates;

pub struct HtmlResponseBuilder {
    response: HtmlResponse,
}

impl HtmlResponseBuilder {
    /// Set the status code
    pub fn status(&mut self, status: StatusCode) -> &mut Self {
        self.response.status = status;
        return self;
    }

    /// Set the head title
    pub fn title(&mut self, title: &'static str) -> &mut Self {
        self.response.head.title = title;
        return self;
    }

    /// Set the meta description
    pub fn description(&mut self, description: &'static str) -> &mut Self {
        self.response.head.description = description;
        return self;
    }

    /// Set the content of the <main></main> tag
    pub fn body(&mut self, body: &'static str) -> &mut Self {
        self.response.body = body;
        return self;
    }

    /// Build the HTML response
    pub fn build(&self) -> HtmlResponse {
        return self.response.clone();
    }
}

#[derive(Clone)]
pub struct Head {
    title: &'static str,
    description: &'static str,
}

#[derive(Clone)]
pub struct HtmlResponse {
    status: StatusCode,
    head: Head,
    body: &'static str,
}

impl HtmlResponse {
    pub fn builder() -> HtmlResponseBuilder {
        return HtmlResponseBuilder {
            response: HtmlResponse {
                status: StatusCode::OK,
                head: Head {
                    title: "jacobmatthe.ws",
                    description: "The blog of Jacob Matthews",
                },
                body: "",
            },
        };
    }
}

impl Responder for HtmlResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        return HttpResponse::build(self.status)
            .content_type(ContentType::html())
            .body(templates::page::new(
                self.head.title,
                self.head.description,
                self.body,
            ));
    }
}
