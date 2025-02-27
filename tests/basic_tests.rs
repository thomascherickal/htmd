use htmd::{
    options::{BrStyle, Options},
    Element, HtmlToMarkdown,
};

#[test]
fn links() {
    let html = r#"
        <a href="https://example.com">Link 1</a>
        <a href="https://example.com" title="Hello">Link 2</a>
        "#;
    let md = HtmlToMarkdown::new().convert(html).unwrap();
    assert_eq!(
        "[Link 1](https://example.com)[Link 2](https://example.com \"Hello\")",
        &md
    )
}

#[test]
fn images() {
    let html = r#"
        <img src="https://example.com" />
        <img src="https://example.com" alt="Image 1" />
        <img src="https://example.com" alt="Image 2" title="Hello" />
        "#;
    let md = HtmlToMarkdown::new().convert(html).unwrap();
    assert_eq!(
        "![](https://example.com)![Image 1](https://example.com)\
            ![Image 2](https://example.com \"Hello\")",
        &md
    )
}

#[test]
fn unordered_lists() {
    let html = r#"
        <ul>
            <li>Item 1</li>
            <li>Item 2</li>
            <li>Item 3</li>
        </ul>
        "#;
    let md = HtmlToMarkdown::new().convert(html).unwrap();
    assert_eq!("*   Item 1\n*   Item 2\n*   Item 3", &md)
}

#[test]
fn headings() {
    let html = r#"
        <h1>Heading 1</h1>
        <h2>Heading 2</h2>
        <h3>Heading 3</h3>
        <h4>Heading 4</h4>
        <h5>Heading 5</h5>
        <h6>Heading 6</h6>
        "#;
    let md = HtmlToMarkdown::new().convert(html).unwrap();
    assert_eq!(
        "# Heading 1\n\n## Heading 2\n\n### Heading 3\n\n\
             #### Heading 4\n\n##### Heading 5\n\n###### Heading 6",
        &md
    )
}

#[test]
fn code_blocks() {
    let html = r#"
        <pre><code>println!("Hello");</code></pre>
        "#;
    let md = HtmlToMarkdown::new().convert(html).unwrap();
    assert_eq!("```\nprintln!(\"Hello\");\n```", &md);
}

#[test]
fn paragraphs() {
    let html = r#"
        <p>The first.</p>
        <p>The <span>second.</span></p>
        "#;
    let md = HtmlToMarkdown::new().convert(html).unwrap();
    assert_eq!("The first.\n\nThe second.", &md);
}

#[test]
fn quotes() {
    let html = r#"
        <blockquote>Once upon a time</blockquote>
        "#;
    let md = HtmlToMarkdown::new().convert(html).unwrap();
    assert_eq!("> Once upon a time", &md);
}

#[test]
fn br() {
    let html = r#"
        Hi<br>there<br><br>!
        "#;
    let md = HtmlToMarkdown::new().convert(html).unwrap();
    assert_eq!("Hi  \nthere  \n  \n!", &md);

    let md = HtmlToMarkdown::builder()
        .options(Options {
            br_style: BrStyle::Backslash,
            ..Default::default()
        })
        .build()
        .convert(html)
        .unwrap();
    assert_eq!("Hi\\\nthere\\\n\\\n!", &md);
}

#[test]
fn hr() {
    let html = r#"Hi <hr/> there"#;
    let md = HtmlToMarkdown::new().convert(html).unwrap();
    assert_eq!("Hi\n\n* * *\n\nthere", &md);
}

#[test]
fn strong_italic() {
    let html = r#"<i>Italic</i> <em>Also italic</em> <strong>Strong</strong>"#;
    let md = HtmlToMarkdown::new().convert(html).unwrap();
    assert_eq!("_Italic__Also italic_**Strong**", &md);
}

#[test]
fn raw_text() {
    let html = r#"Hello world!"#;
    let md = HtmlToMarkdown::new().convert(html).unwrap();
    assert_eq!("Hello world!", &md);
}

#[test]
fn nested_divs() {
    let html = r#"
    <div>
        <div>
            <div>Hi</div>
        </div>
        <div></div>
        <div>there</div>
    </div>
    "#;
    let md = HtmlToMarkdown::new().convert(html).unwrap();
    assert_eq!("Hi\n\nthere", &md);
}

#[test]
fn with_head() {
    let html = r#"
    <html>
        <head>
            <title>Demo</title>
            <script>console.log('Hello');</script>
            <style>body {}</style>
        </head>
        <body>
            Content
        </body>
    </html> 
    "#;
    let md = HtmlToMarkdown::new().convert(html).unwrap();
    assert_eq!("Demo\n\nconsole.log('Hello');\n\nbody {}\n\nContent", &md);
}

#[test]
fn with_custom_rules() {
    // Remove element
    let html = r#"<img src="https://example.com"/>"#;
    let md = HtmlToMarkdown::builder()
        .add_handler(vec!["img"], |_: Element| None)
        .build()
        .convert(html)
        .unwrap();
    assert_eq!("", &md);
}
