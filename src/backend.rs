#[derive(Clone,Debug)]
pub enum SlideData {
    ImageText {
        src: &'static str,
        label: Option<&'static str>,
    },
    Text(&'static str),
}

#[derive(Debug)]
pub struct Backend {
    data: Vec<SlideData>,
}

impl Backend {
    pub fn new() -> Backend {
        use SlideData::*;
        Backend {
            data: vec![
                ImageText {
                    src: "assets/berge.png",
                    label: Some("Berge"),
                },
                ImageText {
                    src: "assets/berge.png",
                    label: None,
                },
                Text("TODO"),
                ImageText {
                    src: "assets/schnee.png",
                    label: Some("Schnee"),
                },
                ImageText {
                    src: "assets/wald.png",
                    label: Some("Wald"),
                },
                Text("Ende")
       ],
        }
    }

    pub fn fetch(&self, i: usize) -> Option<SlideData> {
        self.data.get(i).cloned()
    }

    pub fn number_of_slides(&self) -> usize {
        self.data.len()
    }
}
