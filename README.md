# simpletemplate

This is a template engine written entirely in rust.

## usage  

For variables, use `{{ variable_name }}`.

For array indexing, use ``{{ array_name[index] }}``.

To iterate over an array:

```bash
{{ for loop_variable in loop_iterable }} 
    loop_body 
{{ endfor }}
```

To access the index:

```bash
{{ for loop_variable in loop_iterable }} 
    {{ index }}
{{ endfor }}
```

If statements: `if_body` is rendered if `condition` is not `null`, `false`, or `"false"`.

```bash
{{ if condition }} 
    if_body 
{{ else }} 
    else_body 
{{ endif }} 
```

View `src/main.rs` to see how to render from an HTML file.
