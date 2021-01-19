extern crate proc_macro;
extern crate syn;
extern crate quote;
use proc_macro::TokenStream;
use syn::*;
use parse::*;
use quote::*;
use std::*;

// struct Field {
//     name: String,
//     type_: String
// }
// struct EnumItem {
//     name: String,
//     fields: Vec<Field>
// }
struct AnonymousEnum {
    items: Vec<Variant>
}
enum State {
    Enum(AnonymousEnum),
    Struct(Vec<Field>)
}

struct Declaration {
    name: Ident,
    state: State,
    input_type: Ident,
    output_type: Ident,
    output_table: String,
    translation_table: String,
    anonymous_enums: Vec<AnonymousEnum>
}

impl Parse for Declaration {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner;

        let name: Ident = input.parse()?;
        braced!(inner in input);

        let 

        todo!()
    }
}


#[proc_macro]
pub fn automata(declaration: TokenStream) -> TokenStream {
    let Declaration {
        name,
        state,
        input_type,
        output_type,
        output_table,
        translation_table,
        anonymous_enums
    } = parse_macro_input!{ declaration as Declaration };

    println!("in macro");

    let enum_decl = quote!{
        enum #name State {

        }
    };

    let state_decl = quote!{

    };

    let code = quote!{
        #enum_decl

        #state_decl

        impl Automata<'_> for #name {
            type Input = #input_type;
            type Output = #output_type;

            fn output_table(state: &Self::State, input: &Self::Input) -> Self::Output {
                match (state, input) #output_table
            }

            fn transition_table(state: &Self::State, input: &Self::Input) -> Self::State {
                match (state, input) #translation_table
            }
        }
    };

    println!("{:?}", code);
    code.into()
    // todo!()
}