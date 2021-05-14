# generated url to bind method mapping

**GENERAL RULE: for fixed path with some fixed name 
children or method, the name of path will be the function name**

Example

```
/
 +-/fixed_path_0
 |  `-/fixed_child
 |     `-get: gets fixed_path_0/fixed_child
 `-/fixed_path_1
    `-get: gets fixed_path_1
```

```rust
fn test() {
    client
        .fixed_path_0()
        .fixed_child()
        .get(..params);
    client
        .fixed_path_1()
        .get(..params);
}
```

**GENERAL RULE: for named path, the name of parameter will be the function name.**

Example

```
/
 +-/fixed_path_0
 |  `-/{variable_name}
 |     `-get: gets fixed_path_0/variable_name
 `-/{variable_name_1}
    `-get: gets variable_name_1
```

```rust
fn test() {
    client
        .fixed_path_0()
        .variable_name("variable_name_here")
        .get(..params);
    client
        .variable_name_1("variable_name_here")
        .get(..params);
}
```

**EXCEPTION RULE: if parent of named paths is a fixed name,
the fixed path name in singular form will be the method name**

```
/
 +-/variables
 |  `-/{variable_name}
 |     `-get: gets variables/variable_name
 `-/multi_variables
    `-/{variable_name_1}
       `-/{variable_name_2}
          `-get: gets multi_variables/variable_name_1/variable_name_2
```

```rust
fn test() {
    client
        .variable("variable_name")
        .get(..params);
    client
        .multi_variable("variable_name_1", "variable_name_2")
        .get(..params);
}
```
