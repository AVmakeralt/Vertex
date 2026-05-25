use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static ERROR_EXPLAIN: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(
        "E0001",
        "\x1b[1;31m[E0001] Unknown macro used.\x1b[0m\n\n\
         The compiler encountered a call to a macro that has not been defined.\n\n\
         \x1b[1mExample:\x1b[0m\n\
         notExistingMacro!(); // error: macro doesn't exist\n\n\
         \x1b[1;32mFix:\x1b[0m Ensure that the macro name is spelled correctly and that it is available in the current context.",
    );

    m.insert(
        "E0002",
        "\x1b[1;31m[E0002] Cannot infer type for variable.\x1b[0m\n\n\
         Vertex was unable to determine the type of a variable because it has no initial value and no explicit type annotation.\n\n\
         \x1b[1mExample:\x1b[0m\n\
         var x; // error: no type or value\n\n\
         \x1b[1;32mFix:\x1b[0m Specify the type explicitly (e.g., \x1b[36mvar x: int;\x1b[0m) or assign an initial value (e.g., \x1b[36mvar x = 5;\x1b[0m).",
    );

    m.insert(
        "E0003",
        "\x1b[1;31m[E0003] Undefined type used.\x1b[0m\n\n\
         The specified type is not recognized by the compiler.\n\n\
         \x1b[1mExample:\x1b[0m\n\
         var x: MyCustomType; // error: MyCustomType not declared\n\n\
         \x1b[1;32mFix:\x1b[0m Ensure you are using built-in types (\x1b[36mint, string, bool, flt, void\x1b[0m) or that your custom type is properly defined.",
    );

    m.insert(
        "E0004",
        "\x1b[1;31m[E0004] Type mismatch.\x1b[0m\n\n\
         A value was used in a context that expected a different type.\n\n\
         \x1b[1mExample:\x1b[0m\n\
         var foo: bool = 123; // error: cannot assign int to bool\n\n\
         \x1b[1;32mFix:\x1b[0m Ensure that the value matches the expected type of the variable, function argument, or operation.",
    );

    m.insert(
        "E0005",
        "\x1b[1;31m[E0005] Invalid binary operation.\x1b[0m\n\n\
         A binary operator was used with incompatible operand types.\n\n\
         \x1b[1mExample:\x1b[0m\n\
         true + 5; // error: '+' cannot be used between bool and int\n\n\
         \x1b[1;32mFix:\x1b[0m Check the types of both operands. Most operations require operands to be of the same or compatible numeric types.",
    );

    m.insert(
        "E0006",
        "\x1b[1;31m[E0006] Undefined variable.\x1b[0m\n\n\
         The compiler found a reference to a variable that has not been declared in the current scope.\n\n\
         \x1b[1mExample:\x1b[0m\n\
         writeLn!(my_var); // error: my_var not declared\n\n\
         \x1b[1;32mFix:\x1b[0m Declare the variable using \x1b[36mvar\x1b[0m or \x1b[36mconst\x1b[0m before accessing it, and ensure it is spelled correctly.",
    );

    m.insert(
        "E0007",
        "\x1b[1;31m[E0007] Variable already exists.\x1b[0m\n\n\
         A variable or constant with the same name has already been declared in the current scope.\n\n\
         \x1b[1mExample:\x1b[0m\n\
         var x = 5;\n\
         var x = 10; // error: x redefined\n\n\
         \x1b[1;32mFix:\x1b[0m Use a unique name for the new declaration, or remove the redundant one.",
    );

    m.insert(
        "E0008",
        "\x1b[1;31m[E0008] Constant without value.\x1b[0m\n\n\
         A constant declaration must include an initial value.\n\n\
         \x1b[1mExample:\x1b[0m\n\
         const PI: flt; // error: constants must have a value\n\n\
         \x1b[1;32mFix:\x1b[0m Provide an initial value at the time of declaration: \x1b[36mconst PI: flt = 3.14;\x1b[0m",
    );

    m.insert(
        "E0009",
        "\x1b[1;31m[E0009] Cannot reassign constant.\x1b[0m\n\n\
         Constants are immutable and cannot be reassigned after their initial declaration.\n\n\
         \x1b[1mExample:\x1b[0m\n\
         const X = 5;\n\
         X = 6; // error: cannot reassign const\n\n\
         \x1b[1;32mFix:\x1b[0m If you need to change the value, declare the identifier using \x1b[36mvar\x1b[0m instead of \x1b[36mconst\x1b[0m.",
    );

    m.insert(
        "E0010",
        "\x1b[1;31m[E0010] Wrong macro argument count.\x1b[0m\n\n\
         The macro was called with an incorrect number of arguments.\n\n\
         \x1b[1mExample:\x1b[0m\n\
         my_macro!(1, 2, 3); // error: expected 2 args, got 3\n\n\
         \x1b[1;32mFix:\x1b[0m Call the macro with the correct number of arguments as specified in its definition.",
    );

    m.insert(
        "E0011",
        "\x1b[1;31m[E0011] Expected printable type.\x1b[0m\n\n\
         The write! or writeLn! macro was called with a type that cannot be printed (e.g., bool).\n\n\
         \x1b[1mExample:\x1b[0m\n\
         var b = true;\n\
         writeLn!(b); // error: bool is not printable\n\n\
         \x1b[1;32mFix:\x1b[0m Convert the value to a string or use a printable type (int, flt, string).",
    );

    m.insert(
        "E0012",
        "\x1b[1;31m[E0012] Function already defined.\x1b[0m\n\n\
         A function with the same name has already been declared.\n\n\
         \x1b[1mExample:\x1b[0m\n\
         fnc foo() { ... }\n\
         fnc foo() { ... } // error: foo redefined\n\n\
         \x1b[1;32mFix:\x1b[0m Use a unique name for the function.",
    );

    m.insert(
        "E0013",
        "\x1b[1;31m[E0013] Unknown function.\x1b[0m\n\n\
         The compiler found a call to a function that has not been defined.\n\n\
         \x1b[1mExample:\x1b[0m\n\
         my_func(); // error: my_func not defined\n\n\
         \x1b[1;32mFix:\x1b[0m Ensure the function is defined and the name is spelled correctly.",
    );

    m.insert(
        "E0014",
        "\x1b[1;31m[E0014] Unexpected number of function arguments.\x1b[0m\n\n\
         A function was called with more or fewer arguments than its definition requires.\n\n\
         \x1b[1mExample:\x1b[0m\n\
         fnc add(a: int, b: int): int { ... }\n\
         add(1); // error: expected 2 args, got 1\n\n\
         \x1b[1;32mFix:\x1b[0m Pass the correct number of arguments matching the function's signature.",
    );

    m.insert(
        "E0015",
        "\x1b[1;31m[E0015] Type already exists.\x1b[0m\n\n\
         A type with the same name has already been defined.\n\n\
         \x1b[1;32mFix:\x1b[0m Use a unique name for the new type.",
    );

    m.insert(
        "E0016",
        "\x1b[1;31m[E0016] Missing return type.\x1b[0m\n\n\
         The function definition is missing a required return type annotation.\n\n\
         \x1b[1mExample:\x1b[0m\n\
         fnc foo() { return 5; } // error: return type not specified\n\n\
         \x1b[1;32mFix:\x1b[0m Add the return type after the function arguments: \x1b[36mfnc foo(): int { ... }\x1b[0m",
    );

    m.insert(
        "E0017",
        "\x1b[1;31m[E0017] Cannot return outside of a function.\x1b[0m\n\n\
         A return statement was used at the top level of a script or outside of any function body.\n\n\
         \x1b[1mExample:\x1b[0m\n\
         var x = 5;\n\
         return x; // error: return outside of function\n\n\
         \x1b[1;32mFix:\x1b[0m Remove the return statement or wrap it inside a function.",
    );

    m
});
