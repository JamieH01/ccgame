use std::{fs, collections::HashMap};

use toml::{Table, value::*, Value, map::Map};

const INDEX_START:&str = r#"<!DOCTYPE html>
<html>
    <head>
        <title>CCG Card Archive</title>
    </head>
    <body>
        <h1>CCG Card Archive</h1>"#;
const INDEX_END:&str = r#"    </body>
</html>"#;

const CARD_START:&str = r#"<!DOCTYPE html>
<html>
    <body>
        <h1>${0}</h1>"#;


fn main() {
    //so, because github pages sucks, this is basically gonna
    //be a script that just generates all the html files
    let mut index = INDEX_START.to_owned(); 

    let mut sets: HashMap<String, Vec<usize>> = HashMap::new();

    let table = fs::read_to_string("./files/cards.toml").unwrap().parse::<Table>().unwrap();
    let elem = table.iter().next().unwrap().1;
    if let Value::Array(cards) = elem {
        let list:Vec<CardEntry> = cards.iter().map(|e| match e {
            Value::Table(it) => it,
            _ => unreachable!()
        })
        .filter_map(|e| CardEntry::try_from(e).ok())
        .collect();


        for (i, card) in list.clone().into_iter().enumerate() {
            let mut card_page = CARD_START.replace("${0}", &card.name);
            card_page.push_str(&format!("<img src={}>", card.img));
            card_page.push_str(&format!("<p>type: {}</p>", card.ctype));
            if let Some(v) = card.subtype {
                card_page.push_str(&format!("subtype: {v}</p>"))
            }
            
            if let Some(v) = card.class {
                card_page.push_str(&format!("class: {v}</p>"))
            }

            if let Some(v) = card.power {
                card_page.push_str(&format!("power: {v}</p>"))
            }

            if let Some(v) = card.atk {
                card_page.push_str(&format!("atk: {v}</p>"))
            }

            if let Some(v) = card.def {
                card_page.push_str(&format!("def: {v}</p>"))
            }

            if let Some(v) = card.author {
                card_page.push_str(&format!("author: {v}</p>"))
            }

            if card.families.len() > 0 {
                card_page.push_str("<h2>Card Families</h2>");
                for set in &card.families {
                    match sets.get_mut(set) {
                        Some(v) => v.push(i),
                        None => {sets.insert(set.clone(), vec![i]);},
                    }
                    
                    let lower = set.to_lowercase().replace(' ', "_");
                    card_page.push_str(
                        &format!("<p><a href=\"../sets/{lower}.html\">{set}</a></p>\n")
                        )
                     
                }
            }

            card_page.push_str(INDEX_END);
            fs::write(format!("./cards/{i}.html"), card_page).unwrap();

            //<p><a href="thing">thing</a></p>
 
            for (name, cards) in &sets {
                let lower = name.to_lowercase().replace(' ', "_");        
                let mut html = String::from(format!("<p><h1>{name} Set<h1><p>"));
                for e in cards {
                    html.push_str(&format!("<p><a href=\"../cards/{e}.html\">{}</a></p>\n", list[*e].name)); 
                }
                fs::write(format!("./sets/{lower}.html"), html).unwrap();
            }

            if !card.hidden {
                index.push_str(&format!("<p><a href=\"./cards/{i}.html\">{}</a></p>\n", card.name));
            }

        }
        

    } else { unreachable!() }

    index.push_str(INDEX_END);
    fs::write("./index.html", index).unwrap();

}

//this actually is useful
#[derive(Debug, Clone)]
struct CardEntry {
    pub name: String,
    pub img: String,
    pub ctype: String,
    pub subtype: Option<String>,
    pub power: Option<String>,
    pub atk: Option<String>,
    pub def: Option<String>,
    pub class: Option<String>,
    pub author: Option<String>,
    pub families: Vec<String>,
    pub hidden: bool,
}

impl TryFrom<&Map<String, Value>> for CardEntry {
    type Error = ();

    fn try_from(value: &Map<String, Value>) -> std::result::Result<Self, Self::Error> {
        let name = value.get_str("name").ok_or(())?;
        let img = format!("\"../files/{}\"", value.get_str("img").ok_or(())?);
        let ctype = value.get_str("type").ok_or(())?; 
        let subtype = value.get_str("subtype"); 
        let power = value.get_str("power"); 
        let atk = value.get_str("atk"); 
        let def = value.get_str("def"); 
        let class = value.get_str("class"); 
        let author = value.get_str("author"); 
        let hidden = match value.get("hidden").unwrap_or(&Value::Boolean(false)) {
            Value::Boolean(b) => *b,
            _ => false,
        };


        let families = match value.get("family") {
            Some(Value::Array(v)) => v.into_iter()
                .filter_map(|e| if let Value::String(s) = e {Some(s.clone())} else {None}) 
                .collect(),
            Some(Value::String(s)) => vec![s.clone()], 
            Some(_) => vec![],
            None => vec![],
        };

        Ok(Self { name, img, ctype, subtype, power, atk, def, class, author, families, hidden })
    }
}

trait CardMethod {
    fn get_str(&self, val: &str) -> Option<String>;
}

impl CardMethod for Map<String, Value> {
    fn get_str(&self, val: &str) -> Option<String> {
        self.get(val).map(|e| if let Value::String(v) = e {v} else {unreachable!()}).cloned()
    }
}
