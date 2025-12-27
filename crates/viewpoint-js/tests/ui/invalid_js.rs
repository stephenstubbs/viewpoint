use viewpoint_js::js;

fn main() {
    // Invalid JavaScript: double let keyword
    let _code = js! { let let x = 1 };
}
