use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    HttpResponse, Responder,
};

use crate::components;

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
    pub fn body(&mut self, body: String) -> &mut Self {
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
    pub title: &'static str,
    pub description: &'static str,
}

#[derive(Clone)]
pub struct HtmlResponse {
    status: StatusCode,
    pub head: Head,
    pub body: String,
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
            .body(components::page::from_response(&self));
    }
}
