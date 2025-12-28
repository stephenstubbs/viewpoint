//! Basic usage examples for the js! macro.
//!
//! Run with: cargo run -p viewpoint-js --example basic_usage

use viewpoint_js::js;

fn main() {
    // Example 1: Simple expressions (returns &'static str)
    println!("=== Simple Expressions ===");

    let simple = js! { 1 + 2 };
    println!("1 + 2: {}", simple);

    let arrow = js! { () => window.innerWidth };
    println!("Arrow function: {}", arrow);

    // Note: For JavaScript string literals, use interpolation with Rust strings
    // because Rust's tokenizer doesn't handle JS single-quoted strings.
    let selector = "li";
    let multiline = js! {
        (() => {
            const items = document.querySelectorAll(#{selector});
            return items.length;
        })()
    };
    println!("IIFE: {}", multiline);

    // Example 2: Interpolation with different types (returns String)
    println!("\n=== Interpolation ===");

    let selector = ".my-class";
    let with_string = js! { document.querySelector(#{selector}) };
    println!("With string interpolation: {}", with_string);

    let count = 42;
    let with_number = js! { Array(#{count}).fill(0) };
    println!("With number interpolation: {}", with_number);

    let is_visible = true;
    let with_bool = js! { element.hidden = !#{is_visible} };
    println!("With bool interpolation: {}", with_bool);

    // Example 3: Option handling
    println!("\n=== Option Handling ===");

    let some_value: Option<i32> = Some(100);
    let with_some = js! { console.log(#{some_value}) };
    println!("With Some value: {}", with_some);

    let none_value: Option<i32> = None;
    let with_none = js! { console.log(#{none_value}) };
    println!("With None value: {}", with_none);

    // Example 4: Multiple interpolations
    println!("\n=== Multiple Interpolations ===");

    let x = 10;
    let y = 20;
    let multiple = js! { ({ x: #{x}, y: #{y} }) };
    println!("Multiple values: {}", multiple);

    // Example 5: String escaping
    println!("\n=== String Escaping ===");

    let message = "Hello \"World\"!\nNew line here.";
    let escaped = js! { alert(#{message}) };
    println!("Escaped string: {}", escaped);

    // Example 6: Complex expressions
    println!("\n=== Complex Expressions ===");

    let len = 5;
    let complex = js! {
        (() => {
            const result = [];
            for (let i = 0; i < #{len}; i++) {
                result.push(i);
            }
            return result;
        })()
    };
    println!("Complex expression: {}", complex);

    // Example 7: Type demonstration
    println!("\n=== Output Types ===");

    // Without interpolation: &'static str
    let static_str: &'static str = js! { window.location.href };
    println!("Static str type: {}", static_str);

    // With interpolation: String
    let val = 1;
    let dynamic_string: String = js! { console.log(#{val}) };
    println!("Dynamic String type: {}", dynamic_string);

    println!("\n=== All examples completed! ===");
}
