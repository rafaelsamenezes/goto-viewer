use std::collections::HashMap;
use std::collections::HashSet;
#[derive(Clone, Debug)]
pub struct Irept {
    pub id: String,
    pub subt: Vec<Irept>,
    pub named_subt: HashMap<String, Irept>,
    pub comments: HashMap<String, Irept>,
}

impl Irept {
    pub fn fix_expression(&mut self) {
        if self.id == "side_effect" {
            self.id = "sideeffect".to_string();
        } 

        let expressions: HashSet<String> = HashSet::from(
            [
                "member", "typecast", "notequal", "or", "mod", "not", "*", "/", "+", "-", "=", "<",
                "lshr",
            ]
            .map(|x| x.to_string()),
        );

        if expressions.contains(&self.id) {
            let mut operands = Irept::default();
            operands.subt = self.subt.clone();
            self.named_subt.insert("operands".to_string(), operands);
            self.subt.clear();
        }

        for sub in &mut self.subt {
            sub.fix_expression();
        }

        for (k, v) in &mut self.named_subt {

            if k == "components" {
                for sub in &mut v.subt {
                    sub.id = "component".to_string();
                }
            }
            v.fix_expression();
        }
    }

}

impl std::hash::Hash for Irept {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        for irep in &self.subt {
            irep.hash(state);
        }
        for (name, irep) in &self.named_subt {
            name.hash(state);
            irep.hash(state);
        }
        for (name, irep) in &self.comments {
            name.hash(state);
            irep.hash(state);
        }
    }
}

impl PartialEq for Irept {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.subt == other.subt
            && self.named_subt == other.named_subt
            && self.comments == other.comments
    }
}
impl Eq for Irept {}

impl std::fmt::Display for Irept {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Default for Irept {
    fn default() -> Self {
        Irept {
            id: String::from(""),
            subt: Vec::new(),
            named_subt: HashMap::new(),
            comments: HashMap::new(),
        }
    }
}

impl From<&String> for Irept {
    fn from(data: &String) -> Self {
        let mut res = Irept::default();
        res.id = data.clone();
        res
    }
}

impl From<String> for Irept {
    fn from(data: String) -> Self {
        let mut res = Irept::default();
        res.id = data;
        res
    }
}

impl From<&str> for Irept {
    fn from(data: &str) -> Self {
        let mut res = Irept::default();
        res.id = data.to_string();
        res
    }
}

impl Irept {
    pub fn get_nil() -> Self {
        Irept::from("nil")
    }
}
