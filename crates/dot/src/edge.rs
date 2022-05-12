use crate::{
    arrow::{Arrow},
    style::{Style},
    utils::{quote_string},
    render::{RenderOption}
};

pub trait EdgeTrait {
    fn to_dot_string(&self, edge_symbol: &str, options: &[RenderOption]) -> String;
}


pub struct Edge {
    pub from: String,
    pub to: String,
    pub label: &'static str,
    pub style: Style,
    pub start_arrow: Arrow,
    pub end_arrow: Arrow,
    pub color: Option<&'static str>,
}

impl EdgeTrait for Edge {
    fn to_dot_string(&self, edge_symbol: &str, options: &[RenderOption]) -> String {
        let colorstring: String;
        let escaped_label: &String = &quote_string(self.label.into());
        let start_arrow_s: String = self.start_arrow.to_dot_string();
        let end_arrow_s: String = self.end_arrow.to_dot_string();

        let mut text = vec![self.from.as_str(), " ",
                            edge_symbol, " ",
                            self.to.as_str()];

        if !options.contains(&RenderOption::NoEdgeLabels) {
            text.push("[label=");
            text.push(escaped_label.as_str());
            text.push("]");
        }

        if !options.contains(&RenderOption::NoEdgeStyles) && self.style != Style::None {
            text.push("[style=\"");
            text.push(self.style.as_slice());
            text.push("\"]");
        }

        let color: Option<String> = match self.color {
            Some(l) => {
                Some((*l).into())
            },
            None => None,
        };
        if !options.contains(&RenderOption::NoEdgeColors) {
            if let Some(c) = color {
                colorstring = quote_string(c);
                text.push("[color=");
                text.push(&colorstring);
                text.push("]");
            }
        }

        let mut arrow_text: Vec<String> = vec![];
        let mut arrow_str: String = String::new();
        if !options.contains(&RenderOption::NoArrows) &&
            (!self.start_arrow.is_default() || !self.end_arrow.is_default()) {
                
                if !self.end_arrow.is_default() {
                    arrow_text.push(vec!["arrowhead=\"", &end_arrow_s, "\""].into_iter().collect());
                }
                if !self.start_arrow.is_default() {
                    arrow_text.push(vec!["arrowtail=\"", &start_arrow_s, "\""].into_iter().collect());
                }
                if !self.start_arrow.is_default() && !self.end_arrow.is_default() {
                    arrow_text.push(String::from("dir=\"both\""));
                }
            }
        if arrow_text.len() > 0 {
            arrow_str.push_str(&arrow_text.join(" "));
            arrow_str.insert(0, '[');
            arrow_str.push_str("]");
            text.push(arrow_str.as_str());
        }      
        

            // text.push("[");
            // if !self.end_arrow.is_default() {
            //     text.push("arrowhead=\"");
            //     text.push(&end_arrow_s);
            //     text.push("\"");
            // }
            // if !self.start_arrow.is_default() {
            //     text.push("dir=\"both\" arrowtail=\"");
            //     text.push(&start_arrow_s);
            //     text.push("\"");
            // }

            // text.push("]");
        

        text.push(";");
        return text.into_iter().collect();
    }
}

pub fn edge(from: &str, to: &str, label: &'static str, style: Style, color: Option<&'static str>) -> Edge {
    Edge {
        from: String::from(from),
        to: String::from(to),
        label: label,
        style: style,
        start_arrow: Arrow::default(),
        end_arrow: Arrow::default(),
        color: color,

    }
}

pub fn edge_with_arrows(from: &str, to: &str, label: &'static str, style:Style,
    start_arrow: Arrow, end_arrow: Arrow, color: Option<&'static str>) -> Edge {
    Edge {
        from: String::from(from),
        to: String::from(to),
        label: label,
        style: style,
        start_arrow: start_arrow,
        end_arrow: end_arrow,
        color: color,
    }
}
