use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use syn::{Attribute, ItemFn, ItemStruct, visit};
use syn::__private::ToTokens;
use syn::visit::Visit;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        println!("Usage: bridge_gen <path to source code> <path to output file>");
        return;
    }

    let source_code_path = Path::new(&args[1]);

    let mut rust_paths = Vec::new();
    get_all_rust_files_in_dir(source_code_path, &mut rust_paths);

    let parsed_files = rust_paths.iter().map(|path| parse_file(path)).collect::<Vec<_>>();

    let mut output = r#"/* WARNING: This file is auto-generated. Do not modify. */

import { NativeModules, Platform } from 'react-native';
import { Result } from './index';

const LINKING_ERROR =
    `The package 'react-native-bubble-rust' doesn't seem to be linked. Make sure: \n\n${Platform.select(
        { ios: "- You have run 'pod install'\n", default: '' }
    )}- You rebuilt the app after installing the package\n` +
    `- You are not using Expo Go\n`;

const RustInterop = NativeModules.Bubble
    ? NativeModules.Bubble
    : new Proxy(
          {},
          {
              get() {
                  throw new Error(LINKING_ERROR);
              },
          }
      );

/* ---------------- STRUCT DEFINITIONS ------------------- */

"#.to_string();

    for parsed_file in &parsed_files {
        for bridge_struct in &parsed_file.bridge_structs {
            output.push_str(&convert_struct_to_ts(bridge_struct));
        }
    }

    output.push_str("\n/* ---------------- FUNCTION DEFINITIONS ------------------- */\n\n");

    for parsed_file in &parsed_files {
        for bridge_function in &parsed_file.bridge_functions {
            output.push_str(&convert_function_to_ts(bridge_function));
        }
    }

    fs::write(&args[2], output).unwrap();
}

fn get_all_rust_files_in_dir(dir: &Path, rust_paths: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            get_all_rust_files_in_dir(&path, rust_paths);
        } else if path.extension().unwrap() == "rs" {
            rust_paths.push(path);
        }
    }
}

struct ParsedFile {
    bridge_functions: Vec<ItemFn>,
    bridge_structs: Vec<ItemStruct>,
}

fn has_bridge_attribute(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("bridge") {
            return true;
        }
    }
    false
}

impl<'ast> Visit<'ast> for ParsedFile {
    fn visit_item_struct(&mut self, item_struct: &'ast ItemStruct) {
        if has_bridge_attribute(&item_struct.attrs) {
            self.bridge_structs.push(item_struct.clone());
        }
        visit::visit_item_struct(self, item_struct);
    }

    fn visit_item_fn(&mut self, item_fn: &'ast ItemFn) {
        if has_bridge_attribute(&item_fn.attrs) {
            self.bridge_functions.push(item_fn.clone());
        }
        visit::visit_item_fn(self, item_fn);
    }
}

fn parse_file(path: &Path) -> ParsedFile {
    let source_code = std::fs::read_to_string(path).unwrap();
    let syntax_tree = syn::parse_str(&source_code).expect("Failed to parse source code");

    let mut parsed_file = ParsedFile {
        bridge_functions: Vec::new(),
        bridge_structs: Vec::new(),
    };

    visit::visit_file(&mut parsed_file, &syntax_tree);

    parsed_file
}

fn convert_struct_to_ts(in_struct: &ItemStruct) -> String {
    let struct_name = in_struct.ident.to_string();
    let mut out_struct = format!("export interface {} {{\n", struct_name);

    for field in &in_struct.fields {
        let field_name = field.ident.as_ref().unwrap().to_string();

        out_struct.push_str(&format!("    {}: {},\n", field_name, convert_type_to_ts(&field.ty.to_token_stream().to_string())));
    }

    out_struct.push_str("}\n");

    out_struct
}

fn convert_function_to_ts(int_func: &ItemFn) -> String {
    let func_name = int_func.sig.ident.to_string();
    let mut out_func = format!("export function {}(", func_name);

    let mut input_names = Vec::new();
    input_names.reserve(int_func.sig.inputs.len());

    let mut ts_inputs = Vec::new();
    for input in &int_func.sig.inputs {
        let input = match input {
            syn::FnArg::Typed(input) => input,
            _ => panic!("Unexpected function input type")
        };
        let input_name = input.pat.to_token_stream().to_string();
        input_names.push(input_name.clone());
        let input_type = convert_type_to_ts(&input.ty.to_token_stream().to_string());

        ts_inputs.push(format!("{}: {} ", input_name, input_type));
    }
    out_func.push_str(&ts_inputs.join(", "));
    out_func.push_str("): Promise<");

    // export function multiply(a: number, b: number): Promise<Result<number, void>> {
    //     return RustInterop.call(JSON.stringify({
    //         method: 'multiply',
    //         args: { a, b },
    //     })).then((res: string) => JSON.parse(res));
    // }

    if let syn::ReturnType::Type(_, ty) = &int_func.sig.output {
        out_func.push_str(&convert_type_to_ts(&ty.to_token_stream().to_string()));
    } else {
        out_func.push_str("Result<void, void>");
    }

    out_func.push_str(r#"> {
    return RustInterop.call(JSON.stringify({
        method: '"#);
    out_func.push_str(&func_name);
    out_func.push_str(r#"',
        args: {"#);
    out_func.push_str(&input_names.join(", "));
    out_func.push_str(r#"},
    })).then((res: string) => JSON.parse(res));
}

    "#);


    out_func
}

fn convert_type_to_ts(in_type: &str) -> String {
    let mut out_type = String::new();

    let vec_regex = Regex::new(r"^Vec<(.*)>$").unwrap();

    if let Some(captures) = vec_regex.captures(in_type) {
        let inner_type = captures.get(1).unwrap().as_str();
        out_type.push_str(&format!("{}[]", convert_type_to_ts(inner_type)));
        return out_type;
    }

    let result_regex = Regex::new(r"^Result<(.*), (.*)>$").unwrap();

    if let Some(captures) = result_regex.captures(in_type) {
        let ok_type = captures.get(1).unwrap().as_str();
        let err_type = captures.get(2).unwrap().as_str();
        out_type.push_str(&format!("Result<{}, {}>", convert_type_to_ts(ok_type), convert_type_to_ts(err_type)));
        return out_type;
    }

    match in_type {
        "i8" => out_type.push_str("number"),
        "i16" => out_type.push_str("number"),
        "i32" => out_type.push_str("number"),
        "i64" => out_type.push_str("number"),
        "u8" => out_type.push_str("number"),
        "u16" => out_type.push_str("number"),
        "u32" => out_type.push_str("number"),
        "u64" => out_type.push_str("number"),
        "f32" => out_type.push_str("number"),
        "f64" => out_type.push_str("number"),
        "bool" => out_type.push_str("boolean"),
        "String" => out_type.push_str("string"),
        _ => out_type.push_str(in_type)
    }

    out_type
}