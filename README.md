# simpletemplate

Template engine written entirely in rust.

## usage  

```rust
// main.rs
use simpletemplate::render;
use serde_json::{json};

fn main() {
    let data = json!({
        "name": ["Bob Belcher"],
    });
    let content = "{{ name }}";
    let res = render(content, data);
    println!("{}", res); //returns Bob Belcher
}
```

## template usage

For variables, use `{{ variable_name }}`.

For array indexing, use ``{{ array_name[index] }}``.

To iterate over an array:

```rust
{{ for loop_variable in loop_iterable }} 
    loop_body 
{{ endfor }}
```

To access the index:

```rust
{{ for loop_variable in loop_iterable }} 
    {{ index }}
{{ endfor }}
```

If statements: `if_body` is rendered if `condition` is not `null`, `false`, or `"false"`.

```rust
{{ if condition }} 
    if_body 
{{ else }} 
    else_body 
{{ endif }} 
```

View `src/main.rs` to see how to render from an HTML file.
