// tests9.rs
//
// Rust 非常擅长与 C/C++ 以及其他静态编译语言共享 FFI 接口，并且它甚至可以在代码内部进行链接！这通过 extern 块来实现，就像下面的代码一样。
//
// `extern` 关键字后面的短字符串表示外部导入的函数将遵循哪个 ABI。在这个练习中，使用了 "Rust"，而其他变体如 "C" 表示标准 C ABI，"stdcall" 表示 Windows ABI。
//
// 外部导入的函数在 extern 块中声明，并用分号而不是花括号标记签名的结束。可以为这些函数声明添加一些属性以修改链接行为，例如 #[link_name = ".."] 可以修改实际的符号名称。
//
// 如果你想将符号导出到链接环境中，`extern` 关键字也可以标记在一个具有相同 ABI 字符串说明的函数定义之前。Rust 函数的默认 ABI 实际上就是 "Rust"，所以如果你想链接纯 Rust 函数，整个 extern 术语可以省略。
//
// Rust 默认会对符号进行重命名，就像 C++ 一样。为了抑制这种行为并使这些函数可以通过名称寻址，可以应用属性 #[no_mangle]。
//
// 在这个练习中，你的任务是让测试用例能够调用模块 Foo 中的 `my_demo_function`。`my_demo_function_alias` 是 `my_demo_function` 的别名，因此测试用例中的两行代码应该调用同一个函数。
//
// 你不应修改任何现有代码，除了添加两行属性。

// ~I AM NOT DONE

extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // Apply the no_mangle attribute to prevent Rust from mangling the symbol name.
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }

    // Apply no_mangle and specify that my_demo_function_alias refers to my_demo_function.
    #[no_mangle]
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32 {
        a
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
