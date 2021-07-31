use std::rc::Rc;
use exercise_domapi::{browser::Browser, html};

const HTML: &str = r#"<body>
    <p>hello</p>
    <p class="inline">world</p>
    <p class="inline">:)</p>
    <div class="none"><p>this should not be shown</p></div>
    <style>
        .none { 
            display: none;
        }
        .inline {
            display: inline;
        }
    </style>

    <div id="result">
        <p>not loaded</p>
    </div>
    <script>
        document.getElementById("result").innerHTML = `\x3cp\x3eloaded\x3c/p\x3e`
    </script>    
</body>"#;

fn main() {
    let mut siv = cursive::default();

    let node = html::parse(HTML);

    let mut container = Browser::new(Rc::new(siv.cb_sink().clone()), node);
    container.execute_inline_scripts();

    siv.add_fullscreen_layer(container);
    siv.run();
}
