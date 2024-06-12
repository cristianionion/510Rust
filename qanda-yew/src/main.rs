
extern crate serde;
use gloo_net::http::Request;
extern crate wasm_bindgen_futures;

use yew::prelude::*;
use yew::use_effect_with_deps;
use serde::Deserialize;




#[derive(Deserialize,Clone,PartialEq)]
struct Question{
    id: i32,
    title:String,
    content:String,
    tags:Vec<String>,
}

#[function_component(App)]
fn app()->Html {
    let questions = use_state(|| Vec::new());

    {
        let questions = questions.clone();

        // gets the questions from backend
        use_effect_with_deps(move |_|{
            wasm_bindgen_futures::spawn_local(async move{
                let fetched_questions: Vec<Question> =Request::get("http://localhost:4000/questions")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                questions.set(fetched_questions);
            });
            ||()
        }, ());
    }

    // html to display the questions
    html! {
        <>
            <h1>{ "Qs" }</h1>
            <ul>
                { for (*questions).iter().map(|question| html! {
                    <li key={question.id}>
                    <h2>{ format!("id: {}", question.id) }</h2>
                    <h3>{ format!("title: {}", question.title) }</h3>
                        <p>{ &question.content }</p>
                        <small>{ format!("tags: [{}]", question.tags.join(", ")) }</small>
                    </li>
                })}
            </ul>
        </>
    }
}

fn main(){
    yew::Renderer::<App>::new().render();
}

//trunk serve --address 127.0.0.1 --port 3000 
// fix to display the questions in a better way / show tags