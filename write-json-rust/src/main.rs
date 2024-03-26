use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Paragraph{
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let article: Article = Article{
        article: String::from("how to work json in Rust"),
        author: String::from("kumar"),
        paragraph: vec![
            Paragraph{
                name: String::from("first sentance")
            }, Paragraph{
                name: "body of paragraph".to_string()
            }, Paragraph{
                name: "end of the paragraph".to_string()
            }
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("\n\nThe json is: {}", json);
}