use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

struct Paragraph{
    name: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Article{
    artile: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let article: Article = Article{
        artile: String::from("JSON in rust"),
        author: String::from("Nitin"),
        paragraph: vec![
            Paragraph {
                name: String::from("First Line")
            },
            Paragraph {
                name: String::from("Second Line")
            },
            Paragraph {
                name: String::from("Third Line")
            }
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("The json is: {}", json);
}