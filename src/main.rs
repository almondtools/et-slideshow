#![feature(variant_count)]

use crate::Slides::{First, Second, Third};
use std::mem;
use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| {
        let slide: &Signal<Slides> = create_signal(cx, First);
        let slide_info: &ReadSignal<SlideInfo> = create_memo(cx, || slide.get().info());

        view! { cx,
            div(class="slideshow-container") {
                Slide(info=slide_info)
                a(class="next", on:click=|_| { next(slide) }) { "❯" }
            }
        }
    });
}

pub enum Slides {
    First,
    Second,
    Third,
}

impl Slides {
    pub fn next(&self) -> Slides {
        match self {
            First => Second,
            Second => Third,
            Third => First,
        }
    }

    pub fn info(&self) -> SlideInfo {
        match self {
            First => Self::first_slide_info(),
            Second => Self::second_slide_info(),
            Third => Self::third_slide_info(),
        }
    }

    fn first_slide_info() -> SlideInfo {
        SlideInfo {
            number_text: Self::number_text(1),
            src: "assets/wald.png".into(),
            caption_text: "Wald".into(),
        }
    }

    fn second_slide_info() -> SlideInfo {
        SlideInfo {
            number_text: Self::number_text(2),
            src: "assets/schnee.png".into(),
            caption_text: "Schnee".into(),
        }
    }

    fn third_slide_info() -> SlideInfo {
        SlideInfo {
            number_text: Self::number_text(3),
            src: "assets/berge.png".into(),
            caption_text: "Berge".into(),
        }
    }

    fn number_text(number: u32) -> String {
        format!("{} / {}", number, mem::variant_count::<Slides>())
    }
}

fn next(slide: &Signal<Slides>) {
    slide.set(slide.get().next());
}

#[derive(Prop)]
pub struct SlideProps<'a> {
    info: &'a ReadSignal<SlideInfo>,
}

pub struct SlideInfo {
    number_text: String,
    src: String,
    caption_text: String,
}

//noinspection RsFunctionNaming
#[component]
pub fn Slide<'a, G: Html>(cx: Scope<'a>, props: SlideProps<'a>) -> View<G> {
    view! { cx,
        div {
          div(class="numbertext"){ (props.info.get().number_text) }
          img(src=(props.info.get().src), style="width:100%")
          div(class="text") { (props.info.get().caption_text) }
        }
    }
}
