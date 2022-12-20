//! Render html templates.
//!
//! # Examples
//!
//! ```
//! // variables
//! let data = json!({ "foo": "bar" });
//! let template = r"{{ foo }}";
//! let expected = "bar";
//! assert_eq!(render(template, data), expected);
//! ```
//! 
//! ```
//! // logic statements
//! let data = json!({ "foo": "bar" });
//! let template = r"{{ if foo }}{{ foo }}{{ else }}foo is not set{{ endif }}";
//! let expected = "bar";
//! assert_eq!(render(template, data), expected);
//! ```
//! 
//! ```
//! let data = json!({ "foo": null });
//! let template = r"{{ if foo }}{{ foo }}{{ else }}foo is not set{{ endif }}";
//! let expected = "foo is not set";
//! assert_eq!(render(template, data), expected);
//! ```
//! 
//! ```
//! // for loop
//! let data = json!({ "items": ["a", "b", "c"] });
//! let template = r"{{ for item in items }}{{ item }}{{ endfor }}";
//! let expected = "abc";
//! assert_eq!(render(template, data), expected);
//! ```
//! 
//! ```
//! let data = json!({ "items": [] });
//! let template = r"{{ for item in items }}{{ item }}{{ endfor }}";
//! let expected = "";
//! assert_eq!(render(template, data), expected);
//! ```
//! 
//! ```
//! // accessing by index
//! let data = json!({ "items": ["a", "b", "c"] });
//! let template = r"{{ items[0] }}";
//! let expected = "a";
//! assert_eq!(render(template, data), expected);
//! ```
//! 
//! ```
//! let data = json!({ "items": ["a", "b", "c"] });
//! let template = r"{{ items[1] }}";
//! let expected = "b";
//! assert_eq!(render(template, data), expected);
//! ```
//! 
//! ```
//! let data = json!({ "items": ["a", "b", "c"] });
//! let template = r"{{ items[2] }}";
//! let expected = "c";
//! assert_eq!(render(template, data), expected);
//! ```
//! 
//! ```
//! // combining logic
//! let data = json!({
//! "foo": "bar",
//! "items": ["a", "b", "c"],
//! "show_items": true,
//! "show_foo": false,
//! });
//! let template = r"{{ if show_items }}{{ for item in items }}{{ item }}{{ endfor }}{{ endif }}{{ if show_foo }}{{ foo }}{{ endif }}";
//! let expected = "abc";
//! assert_eq!(render(template, data), expected);
//! ```
//! 
//! ```
//! let data = json!({
//! "foo": "bar",
//! "items": ["a", "b", "c"],
//! "show_items": true,
//! "show_foo": false,
//! });
//! let template = r"{{ if show_items }}{{ for item in items }}{{ item }}{{ endfor }}{{ endif }}{{ if show_foo }}{{ foo }}{{ else }}foo is not set{{ endif }}";
//! let expected = "abcfoo is not set";
//! assert_eq!(render(template, data), expected);
//! ```
//! 
//! ```
//! // blank template
//! let data = json!({});
//! let template = "";
//! let expected = "";
//! assert_eq!(render(template, data), expected);
//! ```
//! 
//! ```
//! // invalid variable
//! let data = json!({ "foo": "bar" });
//! let template = "{{ baz }}";
//! let expected = "null";
//! assert_eq!(render(template, data), expected);
//! ```
//! 
//! ```
//! // invalid index
//! let data = json!({ "items": ["a", "b", "c"] });
//! let template = "{{ items[5] }}";
//! let expected = "null";
//! assert_eq!(render(template, data), expected);
//! ```
//! 
//! ```
//! // invalid statement
//! let data = json!({});
//! let template = "{{ if foo }}{{ endif }}";
//! let expected = "";
//! assert_eq!(render(template, data), expected);
//! ```

pub use serde_json::{Value};
pub use regex::Regex;
pub use string_join::display::Join;

pub fn render(content: &str, data: Value) -> String {
    // handle for loop
    let re = Regex::new(r"\{\{ for (\w+) in (\w+) \}\}([\s\S]*?)\{\{ endfor \}\}").unwrap();
    let replaced = re.replace_all(content, |caps: &regex::Captures| {
        let loop_variable = &caps[1];
        let loop_variable_formatted = "".join(["{{ ", loop_variable, " }}"]);
        let loop_variable = loop_variable_formatted.as_str();

        let loop_iterable = &caps[2];
        let loop_body = &caps[3];

        let iterable = &data[loop_iterable];
        if !iterable.is_array() {
            return "".to_string()
        }

        let mut result = "".to_string();
        for (index, value) in iterable.as_array().unwrap().iter().enumerate() {
            let mut loop_body_replaced = loop_body.trim_start_matches('\n').to_string();
            let val = get_value_string(value);
            loop_body_replaced = loop_body_replaced.replace(loop_variable, &val);
            loop_body_replaced = loop_body_replaced.replace("{{ index }}", &index.to_string());
            result.push_str(&loop_body_replaced);
        }
        result
    }); 

    // handle if statements
    let re = Regex::new(r"\{\{ if (\w+) \}\}\s*([\s\S]*?)\s*(?:\{\{ else \}\}\s*([\s\S]*?)\s*)?\{\{ endif \}\}").unwrap();
    let replaced = re.replace_all(&replaced, |caps: &regex::Captures| {
        let matched_group_count = (1..caps.len()).filter(|i| caps.get(*i).is_some()).count();
        let condition = &caps[1];
        let if_body = &caps[2];
        let else_body = if matched_group_count > 2 {
            &caps[3]
        } else {
            ""
        };

        let value = &data[condition];
        if value.is_null() || value == "false" || value == false {
            else_body.to_string()
        } else {
            if_body.to_string()
        }
    });


    // handle variables
    let re = Regex::new(r"\{\{ (\w+) \}\}").unwrap();
    let replaced = re.replace_all(&replaced, |caps: &regex::Captures| {
        let key = &caps[1];
        let value = &data[key];
        if value.is_array() {
            // Join the array values into a single string, separated by commas
            value.as_array()
                .unwrap()
                .iter()
                .map(|v| get_value_string(v))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            get_value_string(value)
        }
    }); 

    // handle indexing
    let re = Regex::new(r"\{\{ (\w+)\[(\d+)\] \}\}").unwrap();
    let replaced = re.replace_all(&replaced, |caps: &regex::Captures| {
        let array_name = &caps[1];
        let index = caps[2].parse::<usize>().unwrap();
        let value = &data[array_name][index];
        get_value_string(value)        
    });

    replaced.to_string()
}

fn get_value_string(value: &Value) -> String {
    if value.is_string() {
        value.as_str().unwrap().to_string()
    } else {
        value.to_string()
    }
}
