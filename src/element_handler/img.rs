use crate::{text_util::TrimAsciiWhitespace, Element};

pub(super) fn img_handler(element: Element) -> Option<String> {
    let mut link: Option<String> = None;
    let mut alt: Option<String> = None;
    let mut title: Option<String> = None;
    for attr in element.attrs.iter() {
        let name = &attr.name.local;
        if name == "href" {
            link = Some(attr.value.to_string())
        } else if name == "src" {
            link = Some(attr.value.to_string());
        } else if name == "alt" {
            alt = Some(attr.value.to_string());
        } else if name == "title" {
            title = Some(attr.value.to_string());
        }
    }

    if link.is_none() {
        return None;
    }

    let process_alt_title = |text: String| {
        text.lines()
            .map(|line| line.trim_ascii_whitespace().replace("\"", "\\\""))
            .filter(|line| !line.is_empty())
            .collect::<Vec<String>>()
            .join("\n")
    };

    // Handle new lines in alt
    let alt = alt.map(process_alt_title);

    // Handle new lines in title
    let title = title.map(process_alt_title);

    let link = link.map(|text| text.replace("(", "\\(").replace(")", "\\)"));

    let md = format!(
        "![{}]({}{})",
        alt.unwrap_or(String::new()),
        link.unwrap_or(String::new()),
        title.map_or(String::new(), |t| format!(" \"{}\"", t))
    );
    Some(md)
}
