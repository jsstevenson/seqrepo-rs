use seqrepo::{Namespace, Query, SeqRepo};

fn main() {
    let sr = SeqRepo::new("/usr/local/share/seqrepo", "latest").unwrap();
    let alias_db = sr.alias_db();
    let query = Query {
        namespace: Some(Namespace::new("%")),
        seqid: Some("yVj-SzT0jvX2CBlpieemKnZe9FTd1zQl".to_string()),
        alias: Some("%".to_string()),
        current_only: false,
    };
    let mut data: Vec<String> = Vec::new();
    let _ = alias_db.find(&query, |record| {
        data.push(record.unwrap().alias);
    });
    println!("{:?}", data)
}
