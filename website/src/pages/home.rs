use actix_web::Responder;

use crate::html::HtmlResponse;

pub async fn home() -> impl Responder {
    return HtmlResponse::builder()
        .title("Home | Jacob Matthews")
        .body(
            r#"
            <h1 class="font-extrabold before:content-['$'] before:mr-2 lowercase">Welcome</h1>
            <br/>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <p>you can learn about me and some of my work <a class="text-sky-600 font-bold hover:" href="/about" alt="About">here</a></p>
            <br />
            <br />
            <p>you can read some of my thoughts below:</p>
            "#
            .to_string(),
        )
        .build();
}
