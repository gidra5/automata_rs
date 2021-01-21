extern crate proc_macro;
extern crate syn;
extern crate quote;
use proc_macro::TokenStream;
use syn::*;
use parse::*;
use punctuated::*;
use quote::*;
use std::*;

struct AnonymousEnum(Punctuated<Variant, Token![,]>);

impl Parse for AnonymousEnum {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner;
        
        braced!(inner in input);

        let items = Punctuated::<Variant, Token![,]>::parse_terminated(&inner)?;
        
        Ok(Self(items))
    }
}

struct AnonymousStruct(Punctuated<FieldValue, Token![,]>);

impl Parse for AnonymousStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner;

        braced!(inner in input);

        let items = Punctuated::<FieldValue, Token![,]>::parse_terminated(&inner)?;
        
        Ok(AnonymousStruct(items))
    }
}

enum AnonymousTypeOrTypeName {
    Enum(AnonymousEnum),
    Struct(AnonymousStruct),
    Type(Ident)
}

impl Parse for AnonymousTypeOrTypeName {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token!(enum)) {
            input.parse::<Token!(enum)>()?;
            Ok(AnonymousTypeOrTypeName::Enum(input.parse::<AnonymousEnum>().unwrap()))
        } else if lookahead.peek(Token!(struct)) {
            input.parse::<Token!(struct)>()?;
            Ok(AnonymousTypeOrTypeName::Struct(input.parse::<AnonymousStruct>().unwrap()))
        } else { //if lookahead.peek(Ident)
            Ok(AnonymousTypeOrTypeName::Type(input.parse::<Ident>().unwrap()))
        } 

        // Ok(match input.parse::<Ident>().unwrap().to_string().as_str() {
        //     "enm"      => AnonymousTypeOrTypeName::Enum(    input.parse::<AnonymousEnum>()  .unwrap()),
        //     "strct"    => AnonymousTypeOrTypeName::Struct(  input.parse::<AnonymousStruct>().unwrap()),
        //     _           => AnonymousTypeOrTypeName::Type(    input.parse::<Ident>()          .unwrap())
        // })
    }
}

struct Declaration {
    name: Ident,
    state: AnonymousTypeOrTypeName,
    input_type: AnonymousTypeOrTypeName,
    output_type: AnonymousTypeOrTypeName,
    output_table: ExprBlock,
    translation_table: ExprBlock,
}

impl Parse for Declaration {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner;
        let name = input.parse::<Ident>()?;
        
        braced!(inner in input); 

        let mut state = None;
        let mut input_type = None;
        let mut output_type = None;
        let mut output_table = None;
        let mut translation_table = None;

        while let Ok(parsed) = inner.parse::<Ident>() {
            inner.parse::<Token!(:)>()?;
            match parsed.to_string().as_str() {
                "state"             => state = Some(inner.parse::<AnonymousTypeOrTypeName>()?),
                "input"             => input_type = Some(inner.parse::<AnonymousTypeOrTypeName>()?),
                "output"            => output_type = Some(inner.parse::<AnonymousTypeOrTypeName>()?),
                "output_table"      => output_table = Some(inner.parse::<ExprBlock>()?),
                "transition_table"  => translation_table = Some(inner.parse::<ExprBlock>()?),
                _ => ()
            }
        }

        Ok(Declaration {
            name, 
            state: state.unwrap(), 
            input_type: input_type.unwrap(), 
            output_type: output_type.unwrap(), 
            output_table: output_table.unwrap(), 
            translation_table: translation_table.unwrap()
        })
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
    } = parse_macro_input!{ declaration as Declaration };

    let (state_decl, state_type) = {
        match state {
            AnonymousTypeOrTypeName::Enum(AnonymousEnum(a)) => (quote!{ enum #name { #a }}, &name),
            AnonymousTypeOrTypeName::Struct(AnonymousStruct(a)) => (quote!{ struct #name { #a }}, &name),
            AnonymousTypeOrTypeName::Type(ref a) => (quote!{}, a),
        }
    };
    let (input_decl, input_type) = {
        let name = Ident::new(&format!("{}Input", name), name.span());

        match input_type {
            AnonymousTypeOrTypeName::Enum(AnonymousEnum(a)) => (quote!{ enum #name { #a }}, name),
            AnonymousTypeOrTypeName::Struct(AnonymousStruct(a)) => (quote!{ struct #name { #a }}, name),
            AnonymousTypeOrTypeName::Type(a) => (quote!{}, a),
        }
    };
    let (output_decl, output_type) = {
        let name = Ident::new(&format!("{}Output", name), name.span());

        match output_type {
            AnonymousTypeOrTypeName::Enum(AnonymousEnum(a)) => (quote!{ enum #name { #a }}, name),
            AnonymousTypeOrTypeName::Struct(AnonymousStruct(a)) => (quote!{ struct #name { #a }}, name),
            AnonymousTypeOrTypeName::Type(a) => (quote!{}, a),
        }
    };

    let code = quote!{
        #state_decl
        #input_decl
        #output_decl

        impl Automata<'_> for #state_type {
            type Input = #input_type;
            type Output = #output_type;

            fn output_table(state: &Self, input: &Self::Input) -> Self::Output #output_table

            fn transition_table(state: &Self, input: &Self::Input) -> Self #translation_table
        }
    };

    println!("{}", code);
    code.into()
}