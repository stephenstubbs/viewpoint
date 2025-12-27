use viewpoint_js::js;

fn main() {
    // Invalid JavaScript: reserved word as variable name
    let _code = js! { let class = 1 };
}
