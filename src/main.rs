#![feature(variant_count)]

use std::rc::Rc;

use backend::Backend;
use backend::SlideData;
use sycamore::prelude::*;

mod backend;

fn main() {
    sycamore::render(|cx| {
        let model: &Signal<Slider> = create_signal(cx, Slider::new());
        let slide: &ReadSignal<Slide> = create_memo(cx, || model.get().slide());

        view! { cx,
            div(class="slideshow-container") {
                Slide(slide=slide)
                Navigation(model=model)
            }
        }
    });
}

pub struct Slider {
    state: usize,
    backend: Rc<Backend>,
}

impl Slider {
    pub fn new() -> Slider {
        let state = 0;
        let backend = Rc::new(Backend::new());
        Slider { state, backend }
    }

    pub fn next_slide(&self) -> Slider {
        let all = self.backend.number_of_slides();
        let state = (self.state + 1) % all;
        let backend = self.backend.clone();
        Slider { state, backend }
    }

    pub fn prev_slide(&self) -> Slider {
        let all = self.backend.number_of_slides();
        let state = (self.state + all - 1) % all;
        let backend = self.backend.clone();
        Slider { state, backend }
    }

    pub fn slide(&self) -> Slide {
        use SlideData::*;
        let data = self.backend.fetch(self.state);
        if let Some(ImageText { src, label }) = data {
            Slide {
                header: self.number_text(),
                src: src.into(),
                label: label.unwrap_or("").into(),
            }
        } else if let Some(Text(label)) = data {
            Slide::text(self.number_text(), label.into())
        } else {
            Slide::empty(self.number_text())
        }
    }

    fn number_text(&self) -> String {
        format!("{} / {}", self.state + 1, self.backend.number_of_slides())
    }
}

pub struct Slide {
    header: String,
    src: String,
    label: String,
}

impl Slide {
    pub fn text(number_text: String, label: String) -> Self {
        Slide {
            header: number_text,
            src: "assets/black.png".into(),
            label,
        }
    }
    pub fn empty(number_text: String) -> Self {
        Slide {
            header: number_text,
            src: "assets/black.png".into(),
            label: "--".into(),
        }
    }
}

#[derive(Prop)]
pub struct SlideProps<'a> {
    slide: &'a ReadSignal<Slide>,
}

#[component]
pub fn Slide<'a, G: Html>(cx: Scope<'a>, props: SlideProps<'a>) -> View<G> {
    view! { cx,
        div {
          div(class="numbertext"){ (props.slide.get().header) }
          img(src=(props.slide.get().src), style="width:100%")
          div(class="text") { (props.slide.get().label) }
        }
    }
}

#[derive(Prop)]
pub struct NavigationProps<'a> {
    model: &'a Signal<Slider>,
}

#[component]
pub fn Navigation<'a, G: Html>(cx: Scope<'a>, props: NavigationProps<'a>) -> View<G> {
    view! { cx,
        a(class="prev", on:click=|_| { props.model.set(props.model.get().prev_slide()) }) { "❮" }
        a(class="next", on:click=|_| { props.model.set(props.model.get().next_slide()) }) { "❯" }
    }
}
