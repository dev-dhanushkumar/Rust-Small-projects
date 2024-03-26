use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article{
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main(){
    let json = r#"
    {
        "article": "This is a sample blog",
        "author": "John Doe",
        "paragraph": [
            {
                "name":"First Paragraph"
            },{
                "name":"Second Paragraph"
            },{
                "name":"end of the paragraph"
            }
        ]
    }"#;
    let parsed: Article = read_json_typed(json);
    for i in 0..3{
    println!("\n\n The name of the first paharagraph is: {}", parsed.paragraph[i].name);
    }
}

fn read_json_typed(raw_json: &str) -> Article{
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed
}