use super::*;
use crate::derive::{Data, DataEnum, DataStruct, DataUnion, DeriveInput};
use crate::punctuated::Punctuated;
use crate::partial_borrows::PartialBorrows;
use proc_macro2::TokenStream;

#[cfg(feature = "extra-traits")]
use crate::tt::TokenStreamHelper;
#[cfg(feature = "extra-traits")]
use std::hash::{Hash, Hasher};

ast_enum_of_structs! {
    /// Things that can appear directly inside of a module or scope.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    //
    // TODO: change syntax-tree-enum link to an intra rustdoc link, currently
    // blocked on https://github.com/rust-lang/rust/issues/62833
    pub enum Item #manual_extra_traits {
        /// A constant item: `const MAX: u16 = 65535`.
        Const(ItemConst),

        /// An enum definition: `enum Foo<A, B> { A(A), B(B) }`.
        Enum(ItemEnum),

        /// An `extern crate` item: `extern crate serde`.
        ExternCrate(ItemExternCrate),

        /// A free-standing function: `fn process(n: usize) -> Result<()> { ...
        /// }`.
        Fn(ItemFn),

        /// A block of foreign items: `extern "C" { ... }`.
        ForeignMod(ItemForeignMod),

        /// An impl block providing trait or associated items: `impl<A> Trait
        /// for Data<A> { ... }`.
        Impl(ItemImpl),

        /// A macro invocation, which includes `macro_rules!` definitions.
        Macro(ItemMacro),

        /// A 2.0-style declarative macro introduced by the `macro` keyword.
        Macro2(ItemMacro2),

        /// A module or module declaration: `mod m` or `mod m { ... }`.
        Mod(ItemMod),

        /// A static item: `static BIKE: Shed = Shed(42)`.
        Static(ItemStatic),

        /// A struct definition: `struct Foo<A> { x: A }`.
        Struct(ItemStruct),

        /// A trait definition: `pub trait Iterator { ... }`.
        Trait(ItemTrait),

        /// A trait alias: `pub trait SharableIterator = Iterator + Sync`.
        TraitAlias(ItemTraitAlias),

        /// A type alias: `type Result<T> = std::result::Result<T, MyError>`.
        Type(ItemType),

        /// A union definition: `union Foo<A, B> { x: A, y: B }`.
        Union(ItemUnion),

        /// A use declaration: `use std::collections::HashMap`.
        Use(ItemUse),

        /// Tokens forming an item not interpreted by Syn.
        Verbatim(TokenStream),

        #[doc(hidden)]
        __Nonexhaustive,
    }
}

ast_struct! {
    /// A constant item: `const MAX: u16 = 65535`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemConst {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub const_token: Token![const],
        pub ident: Ident,
        pub colon_token: Token![:],
        pub ty: Box<Type>,
        pub eq_token: Token![=],
        pub expr: Box<Expr>,
        pub semi_token: Token![;],
    }
}

ast_struct! {
    /// An enum definition: `enum Foo<A, B> { A(A), B(B) }`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemEnum {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub enum_token: Token![enum],
        pub ident: Ident,
        pub generics: Generics,
        pub brace_token: token::Brace,
        pub variants: Punctuated<Variant, Token![,]>,
    }
}

ast_struct! {
    /// An `extern crate` item: `extern crate serde`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemExternCrate {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub extern_token: Token![extern],
        pub crate_token: Token![crate],
        pub ident: Ident,
        pub rename: Option<(Token![as], Ident)>,
        pub semi_token: Token![;],
    }
}

ast_struct! {
    /// A free-standing function: `fn process(n: usize) -> Result<()> { ...
    /// }`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemFn {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub sig: Signature,
        pub block: Box<Block>,
    }
}

ast_struct! {
    /// A block of foreign items: `extern "C" { ... }`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemForeignMod {
        pub attrs: Vec<Attribute>,
        pub abi: Abi,
        pub brace_token: token::Brace,
        pub items: Vec<ForeignItem>,
    }
}

ast_struct! {
    /// An impl block providing trait or associated items: `impl<A> Trait
    /// for Data<A> { ... }`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemImpl {
        pub attrs: Vec<Attribute>,
        pub defaultness: Option<Token![default]>,
        pub unsafety: Option<Token![unsafe]>,
        pub impl_token: Token![impl],
        pub generics: Generics,
        /// Trait this impl implements.
        pub trait_: Option<(Option<Token![!]>, Path, Token![for])>,
        /// The Self type of the impl.
        pub self_ty: Box<Type>,
        pub brace_token: token::Brace,
        pub items: Vec<ImplItem>,
    }
}

ast_struct! {
    /// A macro invocation, which includes `macro_rules!` definitions.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemMacro {
        pub attrs: Vec<Attribute>,
        /// The `example` in `macro_rules! example { ... }`.
        pub ident: Option<Ident>,
        pub mac: Macro,
        pub semi_token: Option<Token![;]>,
    }
}

ast_struct! {
    /// A 2.0-style declarative macro introduced by the `macro` keyword.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemMacro2 #manual_extra_traits {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub macro_token: Token![macro],
        pub ident: Ident,
        pub rules: TokenStream,
    }
}

ast_struct! {
    /// A module or module declaration: `mod m` or `mod m { ... }`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemMod {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub mod_token: Token![mod],
        pub ident: Ident,
        pub content: Option<(token::Brace, Vec<Item>)>,
        pub semi: Option<Token![;]>,
    }
}

ast_struct! {
    /// A static item: `static BIKE: Shed = Shed(42)`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemStatic {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub static_token: Token![static],
        pub mutability: Option<Token![mut]>,
        pub ident: Ident,
        pub colon_token: Token![:],
        pub ty: Box<Type>,
        pub eq_token: Token![=],
        pub expr: Box<Expr>,
        pub semi_token: Token![;],
    }
}

ast_struct! {
    /// A struct definition: `struct Foo<A> { x: A }`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemStruct {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub struct_token: Token![struct],
        pub ident: Ident,
        pub generics: Generics,
        pub fields: Fields,
        pub semi_token: Option<Token![;]>,
    }
}

ast_struct! {
    /// A trait definition: `pub trait Iterator { ... }`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemTrait {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub unsafety: Option<Token![unsafe]>,
        pub auto_token: Option<Token![auto]>,
        pub trait_token: Token![trait],
        pub ident: Ident,
        pub generics: Generics,
        pub colon_token: Option<Token![:]>,
        pub supertraits: Punctuated<TypeParamBound, Token![+]>,
        pub brace_token: token::Brace,
        pub items: Vec<TraitItem>,
    }
}

ast_struct! {
    /// A trait alias: `pub trait SharableIterator = Iterator + Sync`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemTraitAlias {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub trait_token: Token![trait],
        pub ident: Ident,
        pub generics: Generics,
        pub eq_token: Token![=],
        pub bounds: Punctuated<TypeParamBound, Token![+]>,
        pub semi_token: Token![;],
    }
}

ast_struct! {
    /// A type alias: `type Result<T> = std::result::Result<T, MyError>`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemType {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub type_token: Token![type],
        pub ident: Ident,
        pub generics: Generics,
        pub eq_token: Token![=],
        pub ty: Box<Type>,
        pub semi_token: Token![;],
    }
}

ast_struct! {
    /// A union definition: `union Foo<A, B> { x: A, y: B }`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemUnion {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub union_token: Token![union],
        pub ident: Ident,
        pub generics: Generics,
        pub fields: FieldsNamed,
    }
}

ast_struct! {
    /// A use declaration: `use std::collections::HashMap`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ItemUse {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub use_token: Token![use],
        pub leading_colon: Option<Token![::]>,
        pub tree: UseTree,
        pub semi_token: Token![;],
    }
}

#[cfg(feature = "extra-traits")]
impl Eq for Item {}

#[cfg(feature = "extra-traits")]
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Item::Const(this), Item::Const(other)) => this == other,
            (Item::Enum(this), Item::Enum(other)) => this == other,
            (Item::ExternCrate(this), Item::ExternCrate(other)) => this == other,
            (Item::Fn(this), Item::Fn(other)) => this == other,
            (Item::ForeignMod(this), Item::ForeignMod(other)) => this == other,
            (Item::Impl(this), Item::Impl(other)) => this == other,
            (Item::Macro(this), Item::Macro(other)) => this == other,
            (Item::Macro2(this), Item::Macro2(other)) => this == other,
            (Item::Mod(this), Item::Mod(other)) => this == other,
            (Item::Static(this), Item::Static(other)) => this == other,
            (Item::Struct(this), Item::Struct(other)) => this == other,
            (Item::Trait(this), Item::Trait(other)) => this == other,
            (Item::TraitAlias(this), Item::TraitAlias(other)) => this == other,
            (Item::Type(this), Item::Type(other)) => this == other,
            (Item::Union(this), Item::Union(other)) => this == other,
            (Item::Use(this), Item::Use(other)) => this == other,
            (Item::Verbatim(this), Item::Verbatim(other)) => {
                TokenStreamHelper(this) == TokenStreamHelper(other)
            }
            _ => false,
        }
    }
}

#[cfg(feature = "extra-traits")]
impl Hash for Item {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self {
            Item::Const(item) => {
                state.write_u8(0);
                item.hash(state);
            }
            Item::Enum(item) => {
                state.write_u8(1);
                item.hash(state);
            }
            Item::ExternCrate(item) => {
                state.write_u8(2);
                item.hash(state);
            }
            Item::Fn(item) => {
                state.write_u8(3);
                item.hash(state);
            }
            Item::ForeignMod(item) => {
                state.write_u8(4);
                item.hash(state);
            }
            Item::Impl(item) => {
                state.write_u8(5);
                item.hash(state);
            }
            Item::Macro(item) => {
                state.write_u8(6);
                item.hash(state);
            }
            Item::Macro2(item) => {
                state.write_u8(7);
                item.hash(state);
            }
            Item::Mod(item) => {
                state.write_u8(8);
                item.hash(state);
            }
            Item::Static(item) => {
                state.write_u8(9);
                item.hash(state);
            }
            Item::Struct(item) => {
                state.write_u8(10);
                item.hash(state);
            }
            Item::Trait(item) => {
                state.write_u8(11);
                item.hash(state);
            }
            Item::TraitAlias(item) => {
                state.write_u8(12);
                item.hash(state);
            }
            Item::Type(item) => {
                state.write_u8(13);
                item.hash(state);
            }
            Item::Union(item) => {
                state.write_u8(14);
                item.hash(state);
            }
            Item::Use(item) => {
                state.write_u8(15);
                item.hash(state);
            }
            Item::Verbatim(item) => {
                state.write_u8(16);
                TokenStreamHelper(item).hash(state);
            }
            Item::__Nonexhaustive => unreachable!(),
        }
    }
}

#[cfg(feature = "extra-traits")]
impl Eq for ItemMacro2 {}

#[cfg(feature = "extra-traits")]
impl PartialEq for ItemMacro2 {
    fn eq(&self, other: &Self) -> bool {
        self.attrs == other.attrs
            && self.vis == other.vis
            && self.macro_token == other.macro_token
            && self.ident == other.ident
            && TokenStreamHelper(&self.rules) == TokenStreamHelper(&other.rules)
    }
}

#[cfg(feature = "extra-traits")]
impl Hash for ItemMacro2 {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.attrs.hash(state);
        self.vis.hash(state);
        self.macro_token.hash(state);
        self.ident.hash(state);
        TokenStreamHelper(&self.rules).hash(state);
    }
}

impl From<DeriveInput> for Item {
    fn from(input: DeriveInput) -> Item {
        match input.data {
            Data::Struct(data) => Item::Struct(ItemStruct {
                attrs: input.attrs,
                vis: input.vis,
                struct_token: data.struct_token,
                ident: input.ident,
                generics: input.generics,
                fields: data.fields,
                semi_token: data.semi_token,
            }),
            Data::Enum(data) => Item::Enum(ItemEnum {
                attrs: input.attrs,
                vis: input.vis,
                enum_token: data.enum_token,
                ident: input.ident,
                generics: input.generics,
                brace_token: data.brace_token,
                variants: data.variants,
            }),
            Data::Union(data) => Item::Union(ItemUnion {
                attrs: input.attrs,
                vis: input.vis,
                union_token: data.union_token,
                ident: input.ident,
                generics: input.generics,
                fields: data.fields,
            }),
        }
    }
}

impl From<ItemStruct> for DeriveInput {
    fn from(input: ItemStruct) -> DeriveInput {
        DeriveInput {
            attrs: input.attrs,
            vis: input.vis,
            ident: input.ident,
            generics: input.generics,
            data: Data::Struct(DataStruct {
                struct_token: input.struct_token,
                fields: input.fields,
                semi_token: input.semi_token,
            }),
        }
    }
}

impl From<ItemEnum> for DeriveInput {
    fn from(input: ItemEnum) -> DeriveInput {
        DeriveInput {
            attrs: input.attrs,
            vis: input.vis,
            ident: input.ident,
            generics: input.generics,
            data: Data::Enum(DataEnum {
                enum_token: input.enum_token,
                brace_token: input.brace_token,
                variants: input.variants,
            }),
        }
    }
}

impl From<ItemUnion> for DeriveInput {
    fn from(input: ItemUnion) -> DeriveInput {
        DeriveInput {
            attrs: input.attrs,
            vis: input.vis,
            ident: input.ident,
            generics: input.generics,
            data: Data::Union(DataUnion {
                union_token: input.union_token,
                fields: input.fields,
            }),
        }
    }
}

ast_enum_of_structs! {
    /// A suffix of an import tree in a `use` item: `Type as Renamed` or `*`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    //
    // TODO: change syntax-tree-enum link to an intra rustdoc link, currently
    // blocked on https://github.com/rust-lang/rust/issues/62833
    pub enum UseTree {
        /// A path prefix of imports in a `use` item: `std::...`.
        Path(UsePath),

        /// An identifier imported by a `use` item: `HashMap`.
        Name(UseName),

        /// An renamed identifier imported by a `use` item: `HashMap as Map`.
        Rename(UseRename),

        /// A glob import in a `use` item: `*`.
        Glob(UseGlob),

        /// A braced group of imports in a `use` item: `{A, B, C}`.
        Group(UseGroup),
    }
}

ast_struct! {
    /// A path prefix of imports in a `use` item: `std::...`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct UsePath {
        pub ident: Ident,
        pub colon2_token: Token![::],
        pub tree: Box<UseTree>,
    }
}

ast_struct! {
    /// An identifier imported by a `use` item: `HashMap`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct UseName {
        pub ident: Ident,
    }
}

ast_struct! {
    /// An renamed identifier imported by a `use` item: `HashMap as Map`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct UseRename {
        pub ident: Ident,
        pub as_token: Token![as],
        pub rename: Ident,
    }
}

ast_struct! {
    /// A glob import in a `use` item: `*`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct UseGlob {
        pub star_token: Token![*],
    }
}

ast_struct! {
    /// A braced group of imports in a `use` item: `{A, B, C}`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct UseGroup {
        pub brace_token: token::Brace,
        pub items: Punctuated<UseTree, Token![,]>,
    }
}

ast_enum_of_structs! {
    /// An item within an `extern` block.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    //
    // TODO: change syntax-tree-enum link to an intra rustdoc link, currently
    // blocked on https://github.com/rust-lang/rust/issues/62833
    pub enum ForeignItem #manual_extra_traits {
        /// A foreign function in an `extern` block.
        Fn(ForeignItemFn),

        /// A foreign static item in an `extern` block: `static ext: u8`.
        Static(ForeignItemStatic),

        /// A foreign type in an `extern` block: `type void`.
        Type(ForeignItemType),

        /// A macro invocation within an extern block.
        Macro(ForeignItemMacro),

        /// Tokens in an `extern` block not interpreted by Syn.
        Verbatim(TokenStream),

        #[doc(hidden)]
        __Nonexhaustive,
    }
}

ast_struct! {
    /// A foreign function in an `extern` block.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ForeignItemFn {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub sig: Signature,
        pub semi_token: Token![;],
    }
}

ast_struct! {
    /// A foreign static item in an `extern` block: `static ext: u8`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ForeignItemStatic {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub static_token: Token![static],
        pub mutability: Option<Token![mut]>,
        pub ident: Ident,
        pub colon_token: Token![:],
        pub ty: Box<Type>,
        pub semi_token: Token![;],
    }
}

ast_struct! {
    /// A foreign type in an `extern` block: `type void`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ForeignItemType {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub type_token: Token![type],
        pub ident: Ident,
        pub semi_token: Token![;],
    }
}

ast_struct! {
    /// A macro invocation within an extern block.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ForeignItemMacro {
        pub attrs: Vec<Attribute>,
        pub mac: Macro,
        pub semi_token: Option<Token![;]>,
    }
}

#[cfg(feature = "extra-traits")]
impl Eq for ForeignItem {}

#[cfg(feature = "extra-traits")]
impl PartialEq for ForeignItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ForeignItem::Fn(this), ForeignItem::Fn(other)) => this == other,
            (ForeignItem::Static(this), ForeignItem::Static(other)) => this == other,
            (ForeignItem::Type(this), ForeignItem::Type(other)) => this == other,
            (ForeignItem::Macro(this), ForeignItem::Macro(other)) => this == other,
            (ForeignItem::Verbatim(this), ForeignItem::Verbatim(other)) => {
                TokenStreamHelper(this) == TokenStreamHelper(other)
            }
            _ => false,
        }
    }
}

#[cfg(feature = "extra-traits")]
impl Hash for ForeignItem {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self {
            ForeignItem::Fn(item) => {
                state.write_u8(0);
                item.hash(state);
            }
            ForeignItem::Static(item) => {
                state.write_u8(1);
                item.hash(state);
            }
            ForeignItem::Type(item) => {
                state.write_u8(2);
                item.hash(state);
            }
            ForeignItem::Macro(item) => {
                state.write_u8(3);
                item.hash(state);
            }
            ForeignItem::Verbatim(item) => {
                state.write_u8(4);
                TokenStreamHelper(item).hash(state);
            }
            ForeignItem::__Nonexhaustive => unreachable!(),
        }
    }
}

ast_enum_of_structs! {
    /// An item declaration within the definition of a trait.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    //
    // TODO: change syntax-tree-enum link to an intra rustdoc link, currently
    // blocked on https://github.com/rust-lang/rust/issues/62833
    pub enum TraitItem #manual_extra_traits {
        /// An associated constant within the definition of a trait.
        Const(TraitItemConst),

        /// A trait method within the definition of a trait.
        Method(TraitItemMethod),

        /// An associated type within the definition of a trait.
        Type(TraitItemType),

        /// A macro invocation within the definition of a trait.
        Macro(TraitItemMacro),

        /// Tokens within the definition of a trait not interpreted by Syn.
        Verbatim(TokenStream),

        #[doc(hidden)]
        __Nonexhaustive,
    }
}

ast_struct! {
    /// An associated constant within the definition of a trait.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct TraitItemConst {
        pub attrs: Vec<Attribute>,
        pub const_token: Token![const],
        pub ident: Ident,
        pub colon_token: Token![:],
        pub ty: Type,
        pub default: Option<(Token![=], Expr)>,
        pub semi_token: Token![;],
    }
}

ast_struct! {
    /// A trait method within the definition of a trait.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct TraitItemMethod {
        pub attrs: Vec<Attribute>,
        pub sig: Signature,
        pub default: Option<Block>,
        pub semi_token: Option<Token![;]>,
    }
}

ast_struct! {
    /// An associated type within the definition of a trait.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct TraitItemType {
        pub attrs: Vec<Attribute>,
        pub type_token: Token![type],
        pub ident: Ident,
        pub generics: Generics,
        pub colon_token: Option<Token![:]>,
        pub bounds: Punctuated<TypeParamBound, Token![+]>,
        pub default: Option<(Token![=], Type)>,
        pub semi_token: Token![;],
    }
}

ast_struct! {
    /// A macro invocation within the definition of a trait.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct TraitItemMacro {
        pub attrs: Vec<Attribute>,
        pub mac: Macro,
        pub semi_token: Option<Token![;]>,
    }
}

#[cfg(feature = "extra-traits")]
impl Eq for TraitItem {}

#[cfg(feature = "extra-traits")]
impl PartialEq for TraitItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TraitItem::Const(this), TraitItem::Const(other)) => this == other,
            (TraitItem::Method(this), TraitItem::Method(other)) => this == other,
            (TraitItem::Type(this), TraitItem::Type(other)) => this == other,
            (TraitItem::Macro(this), TraitItem::Macro(other)) => this == other,
            (TraitItem::Verbatim(this), TraitItem::Verbatim(other)) => {
                TokenStreamHelper(this) == TokenStreamHelper(other)
            }
            _ => false,
        }
    }
}

#[cfg(feature = "extra-traits")]
impl Hash for TraitItem {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self {
            TraitItem::Const(item) => {
                state.write_u8(0);
                item.hash(state);
            }
            TraitItem::Method(item) => {
                state.write_u8(1);
                item.hash(state);
            }
            TraitItem::Type(item) => {
                state.write_u8(2);
                item.hash(state);
            }
            TraitItem::Macro(item) => {
                state.write_u8(3);
                item.hash(state);
            }
            TraitItem::Verbatim(item) => {
                state.write_u8(4);
                TokenStreamHelper(item).hash(state);
            }
            TraitItem::__Nonexhaustive => unreachable!(),
        }
    }
}

ast_enum_of_structs! {
    /// An item within an impl block.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    //
    // TODO: change syntax-tree-enum link to an intra rustdoc link, currently
    // blocked on https://github.com/rust-lang/rust/issues/62833
    pub enum ImplItem #manual_extra_traits {
        /// An associated constant within an impl block.
        Const(ImplItemConst),

        /// A method within an impl block.
        Method(ImplItemMethod),

        /// An associated type within an impl block.
        Type(ImplItemType),

        /// A macro invocation within an impl block.
        Macro(ImplItemMacro),

        /// Tokens within an impl block not interpreted by Syn.
        Verbatim(TokenStream),

        #[doc(hidden)]
        __Nonexhaustive,
    }
}

ast_struct! {
    /// An associated constant within an impl block.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ImplItemConst {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub defaultness: Option<Token![default]>,
        pub const_token: Token![const],
        pub ident: Ident,
        pub colon_token: Token![:],
        pub ty: Type,
        pub eq_token: Token![=],
        pub expr: Expr,
        pub semi_token: Token![;],
    }
}

ast_struct! {
    /// A method within an impl block.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ImplItemMethod {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub defaultness: Option<Token![default]>,
        pub sig: Signature,
        pub block: Block,
    }
}

ast_struct! {
    /// An associated type within an impl block.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ImplItemType {
        pub attrs: Vec<Attribute>,
        pub vis: Visibility,
        pub defaultness: Option<Token![default]>,
        pub type_token: Token![type],
        pub ident: Ident,
        pub generics: Generics,
        pub eq_token: Token![=],
        pub ty: Type,
        pub semi_token: Token![;],
    }
}

ast_struct! {
    /// A macro invocation within an impl block.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct ImplItemMacro {
        pub attrs: Vec<Attribute>,
        pub mac: Macro,
        pub semi_token: Option<Token![;]>,
    }
}

#[cfg(feature = "extra-traits")]
impl Eq for ImplItem {}

#[cfg(feature = "extra-traits")]
impl PartialEq for ImplItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ImplItem::Const(this), ImplItem::Const(other)) => this == other,
            (ImplItem::Method(this), ImplItem::Method(other)) => this == other,
            (ImplItem::Type(this), ImplItem::Type(other)) => this == other,
            (ImplItem::Macro(this), ImplItem::Macro(other)) => this == other,
            (ImplItem::Verbatim(this), ImplItem::Verbatim(other)) => {
                TokenStreamHelper(this) == TokenStreamHelper(other)
            }
            _ => false,
        }
    }
}

#[cfg(feature = "extra-traits")]
impl Hash for ImplItem {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self {
            ImplItem::Const(item) => {
                state.write_u8(0);
                item.hash(state);
            }
            ImplItem::Method(item) => {
                state.write_u8(1);
                item.hash(state);
            }
            ImplItem::Type(item) => {
                state.write_u8(2);
                item.hash(state);
            }
            ImplItem::Macro(item) => {
                state.write_u8(3);
                item.hash(state);
            }
            ImplItem::Verbatim(item) => {
                state.write_u8(4);
                TokenStreamHelper(item).hash(state);
            }
            ImplItem::__Nonexhaustive => unreachable!(),
        }
    }
}

ast_struct! {
    /// A function signature in a trait or implementation: `unsafe fn
    /// initialize(&self)`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct Signature {
        pub constness: Option<Token![const]>,
        pub asyncness: Option<Token![async]>,
        pub unsafety: Option<Token![unsafe]>,
        pub abi: Option<Abi>,
        pub fn_token: Token![fn],
        pub ident: Ident,
        pub generics: Generics,
        pub paren_token: token::Paren,
        pub inputs: Punctuated<FnArg, Token![,]>,
        pub variadic: Option<Variadic>,
        pub output: ReturnType,
    }
}

impl Signature {
    /// A method's `self` receiver, such as `&self` or `self: Box<Self>`.
    pub fn receiver(&self) -> Option<&FnArg> {
        let arg = self.inputs.first()?;
        match arg {
            FnArg::Receiver(_) => Some(arg),
            FnArg::Typed(PatType { pat, .. }) => {
                if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                    if ident == "self" {
                        return Some(arg);
                    }
                }
                None
            }
        }
    }
}

ast_enum_of_structs! {
    /// An argument in a function signature: the `n: usize` in `fn f(n: usize)`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub enum FnArg {
        /// The `self` argument of an associated method, whether taken by value
        /// or by reference.
        ///
        /// Note that `self` receivers with a specified type, such as `self:
        /// Box<Self>`, are parsed as a `FnArg::Typed`.
        Receiver(Receiver),

        /// A function argument accepted by pattern and type.
        Typed(PatType),
    }
}

ast_enum! {
    pub enum Reference {
        None(Option<Token![mut]>),
        Partial(Token![.], PartialBorrows),
        Full(Token![&], Option<Lifetime>, Option<Token![mut]>),
    }
}

ast_struct! {
    /// The `self` argument of an associated method, whether taken by value
    /// or by reference.
    ///
    /// Note that `self` receivers with a specified type, such as `self:
    /// Box<Self>`, are parsed as a `FnArg::Typed`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct Receiver {
        pub attrs: Vec<Attribute>,
        pub reference: Reference,
        pub self_token: Token![self],
    }
}

#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;

    use crate::ext::IdentExt;
    use crate::parse::discouraged::Speculative;
    use crate::parse::{Parse, ParseStream, Result};
    use proc_macro2::{Delimiter, Group, Punct, Spacing, TokenTree};
    use std::iter::{self, FromIterator};

    crate::custom_keyword!(existential);

    impl Parse for Item {
        fn parse(input: ParseStream) -> Result<Self> {
            let mut attrs = input.call(Attribute::parse_outer)?;
            let ahead = input.fork();
            let vis: Visibility = ahead.parse()?;

            let lookahead = ahead.lookahead1();
            let mut item = if lookahead.peek(Token![extern]) {
                ahead.parse::<Token![extern]>()?;
                let lookahead = ahead.lookahead1();
                if lookahead.peek(Token![crate]) {
                    input.parse().map(Item::ExternCrate)
                } else if lookahead.peek(Token![fn]) {
                    input.parse().map(Item::Fn)
                } else if lookahead.peek(token::Brace) {
                    input.parse().map(Item::ForeignMod)
                } else if lookahead.peek(LitStr) {
                    ahead.parse::<LitStr>()?;
                    let lookahead = ahead.lookahead1();
                    if lookahead.peek(token::Brace) {
                        input.parse().map(Item::ForeignMod)
                    } else if lookahead.peek(Token![fn]) {
                        input.parse().map(Item::Fn)
                    } else {
                        Err(lookahead.error())
                    }
                } else {
                    Err(lookahead.error())
                }
            } else if lookahead.peek(Token![use]) {
                input.parse().map(Item::Use)
            } else if lookahead.peek(Token![static]) {
                input.parse().map(Item::Static)
            } else if lookahead.peek(Token![const]) {
                ahead.parse::<Token![const]>()?;
                let lookahead = ahead.lookahead1();
                if lookahead.peek(Ident) || lookahead.peek(Token![_]) {
                    input.parse().map(Item::Const)
                } else if lookahead.peek(Token![unsafe])
                    || lookahead.peek(Token![async])
                    || lookahead.peek(Token![extern])
                    || lookahead.peek(Token![fn])
                {
                    input.parse().map(Item::Fn)
                } else {
                    Err(lookahead.error())
                }
            } else if lookahead.peek(Token![unsafe]) {
                ahead.parse::<Token![unsafe]>()?;
                let lookahead = ahead.lookahead1();
                if lookahead.peek(Token![trait])
                    || lookahead.peek(Token![auto]) && ahead.peek2(Token![trait])
                {
                    input.parse().map(Item::Trait)
                } else if lookahead.peek(Token![impl]) {
                    input.parse().map(Item::Impl)
                } else if lookahead.peek(Token![async])
                    || lookahead.peek(Token![extern])
                    || lookahead.peek(Token![fn])
                {
                    input.parse().map(Item::Fn)
                } else {
                    Err(lookahead.error())
                }
            } else if lookahead.peek(Token![async]) || lookahead.peek(Token![fn]) {
                input.parse().map(Item::Fn)
            } else if lookahead.peek(Token![mod]) {
                input.parse().map(Item::Mod)
            } else if lookahead.peek(Token![type]) {
                input.parse().map(Item::Type)
            } else if lookahead.peek(existential) {
                input.call(item_existential).map(Item::Verbatim)
            } else if lookahead.peek(Token![struct]) {
                input.parse().map(Item::Struct)
            } else if lookahead.peek(Token![enum]) {
                input.parse().map(Item::Enum)
            } else if lookahead.peek(Token![union]) && ahead.peek2(Ident) {
                input.parse().map(Item::Union)
            } else if lookahead.peek(Token![trait]) {
                input.call(parse_trait_or_trait_alias)
            } else if lookahead.peek(Token![auto]) && ahead.peek2(Token![trait]) {
                input.parse().map(Item::Trait)
            } else if lookahead.peek(Token![impl])
                || lookahead.peek(Token![default]) && !ahead.peek2(Token![!])
            {
                input.parse().map(Item::Impl)
            } else if lookahead.peek(Token![macro]) {
                input.parse().map(Item::Macro2)
            } else if vis.is_inherited()
                && (lookahead.peek(Ident)
                    || lookahead.peek(Token![self])
                    || lookahead.peek(Token![super])
                    || lookahead.peek(Token![extern])
                    || lookahead.peek(Token![crate])
                    || lookahead.peek(Token![::]))
            {
                input.parse().map(Item::Macro)
            } else {
                Err(lookahead.error())
            }?;

            {
                let item_attrs = match &mut item {
                    Item::ExternCrate(item) => &mut item.attrs,
                    Item::Use(item) => &mut item.attrs,
                    Item::Static(item) => &mut item.attrs,
                    Item::Const(item) => &mut item.attrs,
                    Item::Fn(item) => &mut item.attrs,
                    Item::Mod(item) => &mut item.attrs,
                    Item::ForeignMod(item) => &mut item.attrs,
                    Item::Type(item) => &mut item.attrs,
                    Item::Struct(item) => &mut item.attrs,
                    Item::Enum(item) => &mut item.attrs,
                    Item::Union(item) => &mut item.attrs,
                    Item::Trait(item) => &mut item.attrs,
                    Item::TraitAlias(item) => &mut item.attrs,
                    Item::Impl(item) => &mut item.attrs,
                    Item::Macro(item) => &mut item.attrs,
                    Item::Macro2(item) => &mut item.attrs,
                    Item::Verbatim(_) => return Ok(item),
                    Item::__Nonexhaustive => unreachable!(),
                };
                attrs.extend(item_attrs.drain(..));
                *item_attrs = attrs;
            }

            Ok(item)
        }
    }

    impl Parse for ItemMacro {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let path = input.call(Path::parse_mod_style)?;
            let bang_token: Token![!] = input.parse()?;
            let ident: Option<Ident> = input.parse()?;
            let (delimiter, tokens) = input.call(mac::parse_delimiter)?;
            let semi_token: Option<Token![;]> = if !delimiter.is_brace() {
                Some(input.parse()?)
            } else {
                None
            };
            Ok(ItemMacro {
                attrs,
                ident,
                mac: Macro {
                    path,
                    bang_token,
                    delimiter,
                    tokens,
                },
                semi_token,
            })
        }
    }

    impl Parse for ItemMacro2 {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let vis: Visibility = input.parse()?;
            let macro_token: Token![macro] = input.parse()?;
            let ident: Ident = input.parse()?;
            let mut rules = TokenStream::new();

            let mut lookahead = input.lookahead1();
            if lookahead.peek(token::Paren) {
                let paren_content;
                let paren_token = parenthesized!(paren_content in input);
                let args: TokenStream = paren_content.parse()?;
                let mut args = Group::new(Delimiter::Parenthesis, args);
                args.set_span(paren_token.span);
                rules.extend(iter::once(TokenTree::Group(args)));
                lookahead = input.lookahead1();
            }

            if lookahead.peek(token::Brace) {
                let brace_content;
                let brace_token = braced!(brace_content in input);
                let body: TokenStream = brace_content.parse()?;
                let mut body = Group::new(Delimiter::Brace, body);
                body.set_span(brace_token.span);
                rules.extend(iter::once(TokenTree::Group(body)));
            } else {
                return Err(lookahead.error());
            }

            Ok(ItemMacro2 {
                attrs,
                vis,
                macro_token,
                ident,
                rules,
            })
        }
    }

    impl Parse for ItemExternCrate {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ItemExternCrate {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                extern_token: input.parse()?,
                crate_token: input.parse()?,
                ident: {
                    if input.peek(Token![self]) {
                        input.call(Ident::parse_any)?
                    } else {
                        input.parse()?
                    }
                },
                rename: {
                    if input.peek(Token![as]) {
                        let as_token: Token![as] = input.parse()?;
                        let rename: Ident = if input.peek(Token![_]) {
                            Ident::from(input.parse::<Token![_]>()?)
                        } else {
                            input.parse()?
                        };
                        Some((as_token, rename))
                    } else {
                        None
                    }
                },
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ItemUse {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ItemUse {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                use_token: input.parse()?,
                leading_colon: input.parse()?,
                tree: input.parse()?,
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for UseTree {
        fn parse(input: ParseStream) -> Result<UseTree> {
            let lookahead = input.lookahead1();
            if lookahead.peek(Ident)
                || lookahead.peek(Token![self])
                || lookahead.peek(Token![super])
                || lookahead.peek(Token![crate])
                || lookahead.peek(Token![extern])
            {
                let ident = input.call(Ident::parse_any)?;
                if input.peek(Token![::]) {
                    Ok(UseTree::Path(UsePath {
                        ident,
                        colon2_token: input.parse()?,
                        tree: Box::new(input.parse()?),
                    }))
                } else if input.peek(Token![as]) {
                    Ok(UseTree::Rename(UseRename {
                        ident,
                        as_token: input.parse()?,
                        rename: {
                            if input.peek(Ident) {
                                input.parse()?
                            } else if input.peek(Token![_]) {
                                Ident::from(input.parse::<Token![_]>()?)
                            } else {
                                return Err(input.error("expected identifier or underscore"));
                            }
                        },
                    }))
                } else {
                    Ok(UseTree::Name(UseName { ident }))
                }
            } else if lookahead.peek(Token![*]) {
                Ok(UseTree::Glob(UseGlob {
                    star_token: input.parse()?,
                }))
            } else if lookahead.peek(token::Brace) {
                let content;
                Ok(UseTree::Group(UseGroup {
                    brace_token: braced!(content in input),
                    items: content.parse_terminated(UseTree::parse)?,
                }))
            } else {
                Err(lookahead.error())
            }
        }
    }

    impl Parse for ItemStatic {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ItemStatic {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                static_token: input.parse()?,
                mutability: input.parse()?,
                ident: input.parse()?,
                colon_token: input.parse()?,
                ty: input.parse()?,
                eq_token: input.parse()?,
                expr: input.parse()?,
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ItemConst {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ItemConst {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                const_token: input.parse()?,
                ident: {
                    let lookahead = input.lookahead1();
                    if lookahead.peek(Ident) || lookahead.peek(Token![_]) {
                        input.call(Ident::parse_any)?
                    } else {
                        return Err(lookahead.error());
                    }
                },
                colon_token: input.parse()?,
                ty: input.parse()?,
                eq_token: input.parse()?,
                expr: input.parse()?,
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ItemFn {
        fn parse(input: ParseStream) -> Result<Self> {
            let outer_attrs = input.call(Attribute::parse_outer)?;
            let vis: Visibility = input.parse()?;
            let constness: Option<Token![const]> = input.parse()?;
            let asyncness: Option<Token![async]> = input.parse()?;
            let unsafety: Option<Token![unsafe]> = input.parse()?;
            let abi: Option<Abi> = input.parse()?;
            let fn_token: Token![fn] = input.parse()?;
            let ident: Ident = input.parse()?;
            let generics: Generics = input.parse()?;

            let content;
            let paren_token = parenthesized!(content in input);
            let inputs = content.parse_terminated(FnArg::parse)?;
            let variadic = inputs.last().as_ref().and_then(get_variadic);

            fn get_variadic(input: &&FnArg) -> Option<Variadic> {
                if let FnArg::Typed(PatType { ty, .. }) = input {
                    if let Type::Verbatim(tokens) = &**ty {
                        if let Ok(dots) = parse2(tokens.clone()) {
                            return Some(Variadic {
                                attrs: Vec::new(),
                                dots,
                            });
                        }
                    }
                }
                None
            }

            let output: ReturnType = input.parse()?;
            let where_clause: Option<WhereClause> = input.parse()?;

            let content;
            let brace_token = braced!(content in input);
            let inner_attrs = content.call(Attribute::parse_inner)?;
            let stmts = content.call(Block::parse_within)?;

            Ok(ItemFn {
                attrs: private::attrs(outer_attrs, inner_attrs),
                vis,
                sig: Signature {
                    constness,
                    asyncness,
                    unsafety,
                    abi,
                    fn_token,
                    ident,
                    paren_token,
                    inputs,
                    output,
                    variadic,
                    generics: Generics {
                        where_clause,
                        ..generics
                    },
                },
                block: Box::new(Block { brace_token, stmts }),
            })
        }
    }

    impl Parse for FnArg {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;

            let ahead = input.fork();
            if let Ok(mut receiver) = ahead.parse::<Receiver>() {
                if !ahead.peek(Token![:]) {
                    input.advance_to(&ahead);
                    receiver.attrs = attrs;
                    return Ok(FnArg::Receiver(receiver));
                }
            }

            let mut typed = input.call(fn_arg_typed)?;
            typed.attrs = attrs;
            Ok(FnArg::Typed(typed))
        }
    }

    impl Parse for Receiver {
        fn parse(input: ParseStream) -> Result<Self> {
            let reference;
            let self_token;
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![mut]) {
                reference = Reference::None(input.parse()?);
                self_token = input.parse()?;
            } else if lookahead.peek(Token![&]) {
                reference = Reference::Full(
                    input.parse()?,
                    input.parse()?,
                    input.parse()?,
                );
                self_token = input.parse()?;
            } else if lookahead.peek(Token![self]) {
                self_token = input.parse()?;
                reference = if input.peek(Token![.]) {
                    Reference::Partial(
                        input.parse()?,
                        input.parse()?,
                    )
                } else {
                    Reference::None(None)
                };
            } else {
                return Err(lookahead.error());
            }
            Ok(Receiver { attrs: Vec::new(), reference, self_token })
        }
    }

    fn fn_arg_typed(input: ParseStream) -> Result<PatType> {
        // Hack to parse pre-2018 syntax in
        // test/ui/rfc-2565-param-attrs/param-attrs-pretty.rs
        // because the rest of the test case is valuable.
        if input.peek(Ident) && input.peek2(Token![<]) {
            let span = input.fork().parse::<Ident>()?.span();
            return Ok(PatType {
                attrs: Vec::new(),
                pat: Box::new(Pat::Wild(PatWild {
                    attrs: Vec::new(),
                    underscore_token: Token![_](span),
                })),
                colon_token: Token![:](span),
                ty: input.parse()?,
            });
        }

        Ok(PatType {
            attrs: Vec::new(),
            pat: input.parse()?,
            colon_token: input.parse()?,
            ty: Box::new(match input.parse::<Option<Token![...]>>()? {
                Some(dot3) => {
                    let args = vec![
                        TokenTree::Punct(Punct::new('.', Spacing::Joint)),
                        TokenTree::Punct(Punct::new('.', Spacing::Joint)),
                        TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                    ];
                    let tokens = TokenStream::from_iter(args.into_iter().zip(&dot3.spans).map(
                        |(mut arg, span)| {
                            arg.set_span(*span);
                            arg
                        },
                    ));
                    Type::Verbatim(tokens)
                }
                None => input.parse()?,
            }),
        })
    }

    impl Parse for ItemMod {
        fn parse(input: ParseStream) -> Result<Self> {
            let outer_attrs = input.call(Attribute::parse_outer)?;
            let vis: Visibility = input.parse()?;
            let mod_token: Token![mod] = input.parse()?;
            let ident: Ident = input.parse()?;

            let lookahead = input.lookahead1();
            if lookahead.peek(Token![;]) {
                Ok(ItemMod {
                    attrs: outer_attrs,
                    vis,
                    mod_token,
                    ident,
                    content: None,
                    semi: Some(input.parse()?),
                })
            } else if lookahead.peek(token::Brace) {
                let content;
                let brace_token = braced!(content in input);
                let inner_attrs = content.call(Attribute::parse_inner)?;

                let mut items = Vec::new();
                while !content.is_empty() {
                    items.push(content.parse()?);
                }

                Ok(ItemMod {
                    attrs: private::attrs(outer_attrs, inner_attrs),
                    vis,
                    mod_token,
                    ident,
                    content: Some((brace_token, items)),
                    semi: None,
                })
            } else {
                Err(lookahead.error())
            }
        }
    }

    impl Parse for ItemForeignMod {
        fn parse(input: ParseStream) -> Result<Self> {
            let outer_attrs = input.call(Attribute::parse_outer)?;
            let abi: Abi = input.parse()?;

            let content;
            let brace_token = braced!(content in input);
            let inner_attrs = content.call(Attribute::parse_inner)?;
            let mut items = Vec::new();
            while !content.is_empty() {
                items.push(content.parse()?);
            }

            Ok(ItemForeignMod {
                attrs: private::attrs(outer_attrs, inner_attrs),
                abi,
                brace_token,
                items,
            })
        }
    }

    impl Parse for ForeignItem {
        fn parse(input: ParseStream) -> Result<Self> {
            let mut attrs = input.call(Attribute::parse_outer)?;
            let ahead = input.fork();
            let vis: Visibility = ahead.parse()?;

            let lookahead = ahead.lookahead1();
            let mut item = if lookahead.peek(Token![fn]) {
                input.parse().map(ForeignItem::Fn)
            } else if lookahead.peek(Token![static]) {
                input.parse().map(ForeignItem::Static)
            } else if lookahead.peek(Token![type]) {
                input.parse().map(ForeignItem::Type)
            } else if vis.is_inherited()
                && (lookahead.peek(Ident)
                    || lookahead.peek(Token![self])
                    || lookahead.peek(Token![super])
                    || lookahead.peek(Token![extern])
                    || lookahead.peek(Token![crate])
                    || lookahead.peek(Token![::]))
            {
                input.parse().map(ForeignItem::Macro)
            } else {
                Err(lookahead.error())
            }?;

            {
                let item_attrs = match &mut item {
                    ForeignItem::Fn(item) => &mut item.attrs,
                    ForeignItem::Static(item) => &mut item.attrs,
                    ForeignItem::Type(item) => &mut item.attrs,
                    ForeignItem::Macro(item) => &mut item.attrs,
                    ForeignItem::Verbatim(_) | ForeignItem::__Nonexhaustive => unreachable!(),
                };
                attrs.extend(item_attrs.drain(..));
                *item_attrs = attrs;
            }

            Ok(item)
        }
    }

    impl Parse for ForeignItemFn {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let vis: Visibility = input.parse()?;
            let fn_token: Token![fn] = input.parse()?;
            let ident: Ident = input.parse()?;
            let generics: Generics = input.parse()?;

            let content;
            let paren_token = parenthesized!(content in input);
            let mut inputs = Punctuated::new();
            let mut variadic = None;
            while !content.is_empty() {
                let attrs = content.call(Attribute::parse_outer)?;

                if let Some(dots) = content.parse()? {
                    variadic = Some(Variadic { attrs, dots });
                    break;
                }

                let mut arg = content.call(fn_arg_typed)?;
                arg.attrs = attrs;
                inputs.push_value(FnArg::Typed(arg));
                if content.is_empty() {
                    break;
                }

                inputs.push_punct(content.parse()?);
            }

            let output: ReturnType = input.parse()?;
            let where_clause: Option<WhereClause> = input.parse()?;
            let semi_token: Token![;] = input.parse()?;

            Ok(ForeignItemFn {
                attrs,
                vis,
                sig: Signature {
                    constness: None,
                    asyncness: None,
                    unsafety: None,
                    abi: None,
                    fn_token,
                    ident,
                    paren_token,
                    inputs,
                    output,
                    variadic,
                    generics: Generics {
                        where_clause,
                        ..generics
                    },
                },
                semi_token,
            })
        }
    }

    impl Parse for ForeignItemStatic {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ForeignItemStatic {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                static_token: input.parse()?,
                mutability: input.parse()?,
                ident: input.parse()?,
                colon_token: input.parse()?,
                ty: input.parse()?,
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ForeignItemType {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ForeignItemType {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                type_token: input.parse()?,
                ident: input.parse()?,
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ForeignItemMacro {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let mac: Macro = input.parse()?;
            let semi_token: Option<Token![;]> = if mac.delimiter.is_brace() {
                None
            } else {
                Some(input.parse()?)
            };
            Ok(ForeignItemMacro {
                attrs,
                mac,
                semi_token,
            })
        }
    }

    impl Parse for ItemType {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ItemType {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                type_token: input.parse()?,
                ident: input.parse()?,
                generics: {
                    let mut generics: Generics = input.parse()?;
                    generics.where_clause = input.parse()?;
                    generics
                },
                eq_token: input.parse()?,
                ty: input.parse()?,
                semi_token: input.parse()?,
            })
        }
    }

    #[cfg(not(feature = "printing"))]
    fn item_existential(input: ParseStream) -> Result<TokenStream> {
        Err(input.error("existential type is not supported"))
    }

    #[cfg(feature = "printing")]
    fn item_existential(input: ParseStream) -> Result<TokenStream> {
        use crate::attr::FilterAttrs;
        use quote::{ToTokens, TokenStreamExt};

        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let existential_token: existential = input.parse()?;
        let type_token: Token![type] = input.parse()?;
        let ident: Ident = input.parse()?;

        let mut generics: Generics = input.parse()?;
        generics.where_clause = input.parse()?;

        let colon_token: Token![:] = input.parse()?;

        let mut bounds = Punctuated::new();
        while !input.peek(Token![;]) {
            if !bounds.is_empty() {
                bounds.push_punct(input.parse::<Token![+]>()?);
            }
            bounds.push_value(input.parse::<TypeParamBound>()?);
        }

        let semi_token: Token![;] = input.parse()?;

        let mut tokens = TokenStream::new();
        tokens.append_all(attrs.outer());
        vis.to_tokens(&mut tokens);
        existential_token.to_tokens(&mut tokens);
        type_token.to_tokens(&mut tokens);
        ident.to_tokens(&mut tokens);
        generics.to_tokens(&mut tokens);
        generics.where_clause.to_tokens(&mut tokens);
        if !bounds.is_empty() {
            colon_token.to_tokens(&mut tokens);
            bounds.to_tokens(&mut tokens);
        }
        semi_token.to_tokens(&mut tokens);
        Ok(tokens)
    }

    impl Parse for ItemStruct {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let vis = input.parse::<Visibility>()?;
            let struct_token = input.parse::<Token![struct]>()?;
            let ident = input.parse::<Ident>()?;
            let generics = input.parse::<Generics>()?;
            let (where_clause, fields, semi_token) = derive::parsing::data_struct(input)?;
            Ok(ItemStruct {
                attrs,
                vis,
                struct_token,
                ident,
                generics: Generics {
                    where_clause,
                    ..generics
                },
                fields,
                semi_token,
            })
        }
    }

    impl Parse for ItemEnum {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let vis = input.parse::<Visibility>()?;
            let enum_token = input.parse::<Token![enum]>()?;
            let ident = input.parse::<Ident>()?;
            let generics = input.parse::<Generics>()?;
            let (where_clause, brace_token, variants) = derive::parsing::data_enum(input)?;
            Ok(ItemEnum {
                attrs,
                vis,
                enum_token,
                ident,
                generics: Generics {
                    where_clause,
                    ..generics
                },
                brace_token,
                variants,
            })
        }
    }

    impl Parse for ItemUnion {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let vis = input.parse::<Visibility>()?;
            let union_token = input.parse::<Token![union]>()?;
            let ident = input.parse::<Ident>()?;
            let generics = input.parse::<Generics>()?;
            let (where_clause, fields) = derive::parsing::data_union(input)?;
            Ok(ItemUnion {
                attrs,
                vis,
                union_token,
                ident,
                generics: Generics {
                    where_clause,
                    ..generics
                },
                fields,
            })
        }
    }

    fn parse_trait_or_trait_alias(input: ParseStream) -> Result<Item> {
        let (attrs, vis, trait_token, ident, generics) = parse_start_of_trait_alias(input)?;
        let lookahead = input.lookahead1();
        if lookahead.peek(token::Brace)
            || lookahead.peek(Token![:])
            || lookahead.peek(Token![where])
        {
            let unsafety = None;
            let auto_token = None;
            parse_rest_of_trait(
                input,
                attrs,
                vis,
                unsafety,
                auto_token,
                trait_token,
                ident,
                generics,
            )
            .map(Item::Trait)
        } else if lookahead.peek(Token![=]) {
            parse_rest_of_trait_alias(input, attrs, vis, trait_token, ident, generics)
                .map(Item::TraitAlias)
        } else {
            Err(lookahead.error())
        }
    }

    impl Parse for ItemTrait {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let vis: Visibility = input.parse()?;
            let unsafety: Option<Token![unsafe]> = input.parse()?;
            let auto_token: Option<Token![auto]> = input.parse()?;
            let trait_token: Token![trait] = input.parse()?;
            let ident: Ident = input.parse()?;
            let generics: Generics = input.parse()?;
            parse_rest_of_trait(
                input,
                attrs,
                vis,
                unsafety,
                auto_token,
                trait_token,
                ident,
                generics,
            )
        }
    }

    fn parse_rest_of_trait(
        input: ParseStream,
        attrs: Vec<Attribute>,
        vis: Visibility,
        unsafety: Option<Token![unsafe]>,
        auto_token: Option<Token![auto]>,
        trait_token: Token![trait],
        ident: Ident,
        mut generics: Generics,
    ) -> Result<ItemTrait> {
        let colon_token: Option<Token![:]> = input.parse()?;

        let mut supertraits = Punctuated::new();
        if colon_token.is_some() {
            loop {
                supertraits.push_value(input.parse()?);
                if input.peek(Token![where]) || input.peek(token::Brace) {
                    break;
                }
                supertraits.push_punct(input.parse()?);
                if input.peek(Token![where]) || input.peek(token::Brace) {
                    break;
                }
            }
        }

        generics.where_clause = input.parse()?;

        let content;
        let brace_token = braced!(content in input);
        let mut items = Vec::new();
        while !content.is_empty() {
            items.push(content.parse()?);
        }

        Ok(ItemTrait {
            attrs,
            vis,
            unsafety,
            auto_token,
            trait_token,
            ident,
            generics,
            colon_token,
            supertraits,
            brace_token,
            items,
        })
    }

    impl Parse for ItemTraitAlias {
        fn parse(input: ParseStream) -> Result<Self> {
            let (attrs, vis, trait_token, ident, generics) = parse_start_of_trait_alias(input)?;
            parse_rest_of_trait_alias(input, attrs, vis, trait_token, ident, generics)
        }
    }

    fn parse_start_of_trait_alias(
        input: ParseStream,
    ) -> Result<(Vec<Attribute>, Visibility, Token![trait], Ident, Generics)> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let trait_token: Token![trait] = input.parse()?;
        let ident: Ident = input.parse()?;
        let generics: Generics = input.parse()?;
        Ok((attrs, vis, trait_token, ident, generics))
    }

    fn parse_rest_of_trait_alias(
        input: ParseStream,
        attrs: Vec<Attribute>,
        vis: Visibility,
        trait_token: Token![trait],
        ident: Ident,
        mut generics: Generics,
    ) -> Result<ItemTraitAlias> {
        let eq_token: Token![=] = input.parse()?;

        let mut bounds = Punctuated::new();
        loop {
            if input.peek(Token![where]) || input.peek(Token![;]) {
                break;
            }
            bounds.push_value(input.parse()?);
            if input.peek(Token![where]) || input.peek(Token![;]) {
                break;
            }
            bounds.push_punct(input.parse()?);
        }

        generics.where_clause = input.parse()?;
        let semi_token: Token![;] = input.parse()?;

        Ok(ItemTraitAlias {
            attrs,
            vis,
            trait_token,
            ident,
            generics,
            eq_token,
            bounds,
            semi_token,
        })
    }

    impl Parse for TraitItem {
        fn parse(input: ParseStream) -> Result<Self> {
            let mut attrs = input.call(Attribute::parse_outer)?;
            let ahead = input.fork();

            let lookahead = ahead.lookahead1();
            let mut item = if lookahead.peek(Token![const]) {
                ahead.parse::<Token![const]>()?;
                let lookahead = ahead.lookahead1();
                if lookahead.peek(Ident) {
                    input.parse().map(TraitItem::Const)
                } else if lookahead.peek(Token![async])
                    || lookahead.peek(Token![unsafe])
                    || lookahead.peek(Token![extern])
                    || lookahead.peek(Token![fn])
                {
                    input.parse().map(TraitItem::Method)
                } else {
                    Err(lookahead.error())
                }
            } else if lookahead.peek(Token![async])
                || lookahead.peek(Token![unsafe])
                || lookahead.peek(Token![extern])
                || lookahead.peek(Token![fn])
            {
                input.parse().map(TraitItem::Method)
            } else if lookahead.peek(Token![type]) {
                input.parse().map(TraitItem::Type)
            } else if lookahead.peek(Ident)
                || lookahead.peek(Token![self])
                || lookahead.peek(Token![super])
                || lookahead.peek(Token![extern])
                || lookahead.peek(Token![crate])
                || lookahead.peek(Token![::])
            {
                input.parse().map(TraitItem::Macro)
            } else {
                Err(lookahead.error())
            }?;

            {
                let item_attrs = match &mut item {
                    TraitItem::Const(item) => &mut item.attrs,
                    TraitItem::Method(item) => &mut item.attrs,
                    TraitItem::Type(item) => &mut item.attrs,
                    TraitItem::Macro(item) => &mut item.attrs,
                    TraitItem::Verbatim(_) | TraitItem::__Nonexhaustive => unreachable!(),
                };
                attrs.extend(item_attrs.drain(..));
                *item_attrs = attrs;
            }

            Ok(item)
        }
    }

    impl Parse for TraitItemConst {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(TraitItemConst {
                attrs: input.call(Attribute::parse_outer)?,
                const_token: input.parse()?,
                ident: input.parse()?,
                colon_token: input.parse()?,
                ty: input.parse()?,
                default: {
                    if input.peek(Token![=]) {
                        let eq_token: Token![=] = input.parse()?;
                        let default: Expr = input.parse()?;
                        Some((eq_token, default))
                    } else {
                        None
                    }
                },
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for TraitItemMethod {
        fn parse(input: ParseStream) -> Result<Self> {
            let outer_attrs = input.call(Attribute::parse_outer)?;
            let constness: Option<Token![const]> = input.parse()?;
            let asyncness: Option<Token![async]> = input.parse()?;
            let unsafety: Option<Token![unsafe]> = input.parse()?;
            let abi: Option<Abi> = input.parse()?;
            let fn_token: Token![fn] = input.parse()?;
            let ident: Ident = input.parse()?;
            let generics: Generics = input.parse()?;

            let content;
            let paren_token = parenthesized!(content in input);
            let inputs = content.parse_terminated(FnArg::parse)?;

            let output: ReturnType = input.parse()?;
            let where_clause: Option<WhereClause> = input.parse()?;

            let lookahead = input.lookahead1();
            let (brace_token, inner_attrs, stmts, semi_token) = if lookahead.peek(token::Brace) {
                let content;
                let brace_token = braced!(content in input);
                let inner_attrs = content.call(Attribute::parse_inner)?;
                let stmts = content.call(Block::parse_within)?;
                (Some(brace_token), inner_attrs, stmts, None)
            } else if lookahead.peek(Token![;]) {
                let semi_token: Token![;] = input.parse()?;
                (None, Vec::new(), Vec::new(), Some(semi_token))
            } else {
                return Err(lookahead.error());
            };

            Ok(TraitItemMethod {
                attrs: private::attrs(outer_attrs, inner_attrs),
                sig: Signature {
                    constness,
                    asyncness,
                    unsafety,
                    abi,
                    fn_token,
                    ident,
                    paren_token,
                    inputs,
                    output,
                    variadic: None,
                    generics: Generics {
                        where_clause,
                        ..generics
                    },
                },
                default: brace_token.map(|brace_token| Block { brace_token, stmts }),
                semi_token,
            })
        }
    }

    impl Parse for TraitItemType {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let type_token: Token![type] = input.parse()?;
            let ident: Ident = input.parse()?;
            let mut generics: Generics = input.parse()?;
            let colon_token: Option<Token![:]> = input.parse()?;

            let mut bounds = Punctuated::new();
            if colon_token.is_some() {
                while !input.peek(Token![where]) && !input.peek(Token![=]) && !input.peek(Token![;])
                {
                    if !bounds.is_empty() {
                        bounds.push_punct(input.parse()?);
                    }
                    bounds.push_value(input.parse()?);
                }
            }

            generics.where_clause = input.parse()?;
            let default = if input.peek(Token![=]) {
                let eq_token: Token![=] = input.parse()?;
                let default: Type = input.parse()?;
                Some((eq_token, default))
            } else {
                None
            };
            let semi_token: Token![;] = input.parse()?;

            Ok(TraitItemType {
                attrs,
                type_token,
                ident,
                generics,
                colon_token,
                bounds,
                default,
                semi_token,
            })
        }
    }

    impl Parse for TraitItemMacro {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let mac: Macro = input.parse()?;
            let semi_token: Option<Token![;]> = if mac.delimiter.is_brace() {
                None
            } else {
                Some(input.parse()?)
            };
            Ok(TraitItemMacro {
                attrs,
                mac,
                semi_token,
            })
        }
    }

    impl Parse for ItemImpl {
        fn parse(input: ParseStream) -> Result<Self> {
            let outer_attrs = input.call(Attribute::parse_outer)?;
            let defaultness: Option<Token![default]> = input.parse()?;
            let unsafety: Option<Token![unsafe]> = input.parse()?;
            let impl_token: Token![impl] = input.parse()?;

            let has_generics = input.peek(Token![<])
                && (input.peek2(Token![>])
                    || input.peek2(Token![#])
                    || (input.peek2(Ident) || input.peek2(Lifetime))
                        && (input.peek3(Token![:])
                            || input.peek3(Token![,])
                            || input.peek3(Token![>])));
            let generics: Generics = if has_generics {
                input.parse()?
            } else {
                Generics::default()
            };

            let trait_ = {
                // TODO: optimize using advance_to
                let ahead = input.fork();
                if ahead.parse::<Option<Token![!]>>().is_ok()
                    && ahead.parse::<Path>().is_ok()
                    && ahead.parse::<Token![for]>().is_ok()
                {
                    let polarity: Option<Token![!]> = input.parse()?;
                    let path: Path = input.parse()?;
                    let for_token: Token![for] = input.parse()?;
                    Some((polarity, path, for_token))
                } else {
                    None
                }
            };
            let self_ty: Type = input.parse()?;
            let where_clause: Option<WhereClause> = input.parse()?;

            let content;
            let brace_token = braced!(content in input);
            let inner_attrs = content.call(Attribute::parse_inner)?;

            let mut items = Vec::new();
            while !content.is_empty() {
                items.push(content.parse()?);
            }

            Ok(ItemImpl {
                attrs: private::attrs(outer_attrs, inner_attrs),
                defaultness,
                unsafety,
                impl_token,
                generics: Generics {
                    where_clause,
                    ..generics
                },
                trait_,
                self_ty: Box::new(self_ty),
                brace_token,
                items,
            })
        }
    }

    impl Parse for ImplItem {
        fn parse(input: ParseStream) -> Result<Self> {
            let mut attrs = input.call(Attribute::parse_outer)?;
            let ahead = input.fork();
            let vis: Visibility = ahead.parse()?;

            let mut lookahead = ahead.lookahead1();
            let defaultness = if lookahead.peek(Token![default]) && !ahead.peek2(Token![!]) {
                let defaultness: Token![default] = ahead.parse()?;
                lookahead = ahead.lookahead1();
                Some(defaultness)
            } else {
                None
            };

            let mut item = if lookahead.peek(Token![const]) {
                ahead.parse::<Token![const]>()?;
                let lookahead = ahead.lookahead1();
                if lookahead.peek(Ident) {
                    input.parse().map(ImplItem::Const)
                } else if lookahead.peek(Token![unsafe])
                    || lookahead.peek(Token![async])
                    || lookahead.peek(Token![extern])
                    || lookahead.peek(Token![fn])
                {
                    input.parse().map(ImplItem::Method)
                } else {
                    Err(lookahead.error())
                }
            } else if lookahead.peek(Token![unsafe])
                || lookahead.peek(Token![async])
                || lookahead.peek(Token![extern])
                || lookahead.peek(Token![fn])
            {
                input.parse().map(ImplItem::Method)
            } else if lookahead.peek(Token![type]) {
                input.parse().map(ImplItem::Type)
            } else if vis.is_inherited() && defaultness.is_none() && lookahead.peek(existential) {
                input.call(item_existential).map(ImplItem::Verbatim)
            } else if vis.is_inherited()
                && defaultness.is_none()
                && (lookahead.peek(Ident)
                    || lookahead.peek(Token![self])
                    || lookahead.peek(Token![super])
                    || lookahead.peek(Token![extern])
                    || lookahead.peek(Token![crate])
                    || lookahead.peek(Token![::]))
            {
                input.parse().map(ImplItem::Macro)
            } else {
                Err(lookahead.error())
            }?;

            {
                let item_attrs = match &mut item {
                    ImplItem::Const(item) => &mut item.attrs,
                    ImplItem::Method(item) => &mut item.attrs,
                    ImplItem::Type(item) => &mut item.attrs,
                    ImplItem::Macro(item) => &mut item.attrs,
                    ImplItem::Verbatim(_) => return Ok(item),
                    ImplItem::__Nonexhaustive => unreachable!(),
                };
                attrs.extend(item_attrs.drain(..));
                *item_attrs = attrs;
            }

            Ok(item)
        }
    }

    impl Parse for ImplItemConst {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ImplItemConst {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                defaultness: input.parse()?,
                const_token: input.parse()?,
                ident: input.parse()?,
                colon_token: input.parse()?,
                ty: input.parse()?,
                eq_token: input.parse()?,
                expr: input.parse()?,
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ImplItemMethod {
        fn parse(input: ParseStream) -> Result<Self> {
            let outer_attrs = input.call(Attribute::parse_outer)?;
            let vis: Visibility = input.parse()?;
            let defaultness: Option<Token![default]> = input.parse()?;
            let constness: Option<Token![const]> = input.parse()?;
            let asyncness: Option<Token![async]> = input.parse()?;
            let unsafety: Option<Token![unsafe]> = input.parse()?;
            let abi: Option<Abi> = input.parse()?;
            let fn_token: Token![fn] = input.parse()?;
            let ident: Ident = input.parse()?;
            let generics: Generics = input.parse()?;

            let content;
            let paren_token = parenthesized!(content in input);
            let inputs = content.parse_terminated(FnArg::parse)?;

            let output: ReturnType = input.parse()?;
            let where_clause: Option<WhereClause> = input.parse()?;

            let content;
            let brace_token = braced!(content in input);
            let inner_attrs = content.call(Attribute::parse_inner)?;
            let stmts = content.call(Block::parse_within)?;

            Ok(ImplItemMethod {
                attrs: private::attrs(outer_attrs, inner_attrs),
                vis,
                defaultness,
                sig: Signature {
                    constness,
                    asyncness,
                    unsafety,
                    abi,
                    fn_token,
                    ident,
                    paren_token,
                    inputs,
                    output,
                    variadic: None,
                    generics: Generics {
                        where_clause,
                        ..generics
                    },
                },
                block: Block { brace_token, stmts },
            })
        }
    }

    impl Parse for ImplItemType {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ImplItemType {
                attrs: input.call(Attribute::parse_outer)?,
                vis: input.parse()?,
                defaultness: input.parse()?,
                type_token: input.parse()?,
                ident: input.parse()?,
                generics: {
                    let mut generics: Generics = input.parse()?;
                    generics.where_clause = input.parse()?;
                    generics
                },
                eq_token: input.parse()?,
                ty: input.parse()?,
                semi_token: input.parse()?,
            })
        }
    }

    impl Parse for ImplItemMacro {
        fn parse(input: ParseStream) -> Result<Self> {
            let attrs = input.call(Attribute::parse_outer)?;
            let mac: Macro = input.parse()?;
            let semi_token: Option<Token![;]> = if mac.delimiter.is_brace() {
                None
            } else {
                Some(input.parse()?)
            };
            Ok(ImplItemMacro {
                attrs,
                mac,
                semi_token,
            })
        }
    }

    impl Visibility {
        fn is_inherited(&self) -> bool {
            match *self {
                Visibility::Inherited => true,
                _ => false,
            }
        }
    }

    impl MacroDelimiter {
        fn is_brace(&self) -> bool {
            match *self {
                MacroDelimiter::Brace(_) => true,
                MacroDelimiter::Paren(_) | MacroDelimiter::Bracket(_) => false,
            }
        }
    }
}

#[cfg(feature = "printing")]
mod printing {
    use super::*;

    use proc_macro2::TokenStream;
    use quote::{ToTokens, TokenStreamExt};

    use crate::attr::FilterAttrs;
    use crate::print::TokensOrDefault;

    impl ToTokens for ItemExternCrate {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.extern_token.to_tokens(tokens);
            self.crate_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            if let Some((as_token, rename)) = &self.rename {
                as_token.to_tokens(tokens);
                rename.to_tokens(tokens);
            }
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemUse {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.use_token.to_tokens(tokens);
            self.leading_colon.to_tokens(tokens);
            self.tree.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemStatic {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.static_token.to_tokens(tokens);
            self.mutability.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.colon_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
            self.eq_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemConst {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.const_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.colon_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
            self.eq_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemFn {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.sig.to_tokens(tokens);
            self.block.brace_token.surround(tokens, |tokens| {
                tokens.append_all(self.attrs.inner());
                tokens.append_all(&self.block.stmts);
            });
        }
    }

    impl ToTokens for ItemMod {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.mod_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            if let Some((brace, items)) = &self.content {
                brace.surround(tokens, |tokens| {
                    tokens.append_all(self.attrs.inner());
                    tokens.append_all(items);
                });
            } else {
                TokensOrDefault(&self.semi).to_tokens(tokens);
            }
        }
    }

    impl ToTokens for ItemForeignMod {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.abi.to_tokens(tokens);
            self.brace_token.surround(tokens, |tokens| {
                tokens.append_all(self.attrs.inner());
                tokens.append_all(&self.items);
            });
        }
    }

    impl ToTokens for ItemType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.type_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
            self.eq_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemEnum {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.enum_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
            self.brace_token.surround(tokens, |tokens| {
                self.variants.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for ItemStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.struct_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            match &self.fields {
                Fields::Named(fields) => {
                    self.generics.where_clause.to_tokens(tokens);
                    fields.to_tokens(tokens);
                }
                Fields::Unnamed(fields) => {
                    fields.to_tokens(tokens);
                    self.generics.where_clause.to_tokens(tokens);
                    TokensOrDefault(&self.semi_token).to_tokens(tokens);
                }
                Fields::Unit => {
                    self.generics.where_clause.to_tokens(tokens);
                    TokensOrDefault(&self.semi_token).to_tokens(tokens);
                }
            }
        }
    }

    impl ToTokens for ItemUnion {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.union_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
            self.fields.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemTrait {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.unsafety.to_tokens(tokens);
            self.auto_token.to_tokens(tokens);
            self.trait_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            if !self.supertraits.is_empty() {
                TokensOrDefault(&self.colon_token).to_tokens(tokens);
                self.supertraits.to_tokens(tokens);
            }
            self.generics.where_clause.to_tokens(tokens);
            self.brace_token.surround(tokens, |tokens| {
                tokens.append_all(&self.items);
            });
        }
    }

    impl ToTokens for ItemTraitAlias {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.trait_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            self.eq_token.to_tokens(tokens);
            self.bounds.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemImpl {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.defaultness.to_tokens(tokens);
            self.unsafety.to_tokens(tokens);
            self.impl_token.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            if let Some((polarity, path, for_token)) = &self.trait_ {
                polarity.to_tokens(tokens);
                path.to_tokens(tokens);
                for_token.to_tokens(tokens);
            }
            self.self_ty.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
            self.brace_token.surround(tokens, |tokens| {
                tokens.append_all(self.attrs.inner());
                tokens.append_all(&self.items);
            });
        }
    }

    impl ToTokens for ItemMacro {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.mac.path.to_tokens(tokens);
            self.mac.bang_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            match &self.mac.delimiter {
                MacroDelimiter::Paren(paren) => {
                    paren.surround(tokens, |tokens| self.mac.tokens.to_tokens(tokens));
                }
                MacroDelimiter::Brace(brace) => {
                    brace.surround(tokens, |tokens| self.mac.tokens.to_tokens(tokens));
                }
                MacroDelimiter::Bracket(bracket) => {
                    bracket.surround(tokens, |tokens| self.mac.tokens.to_tokens(tokens));
                }
            }
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ItemMacro2 {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.macro_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.rules.to_tokens(tokens);
        }
    }

    impl ToTokens for UsePath {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.ident.to_tokens(tokens);
            self.colon2_token.to_tokens(tokens);
            self.tree.to_tokens(tokens);
        }
    }

    impl ToTokens for UseName {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.ident.to_tokens(tokens);
        }
    }

    impl ToTokens for UseRename {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.ident.to_tokens(tokens);
            self.as_token.to_tokens(tokens);
            self.rename.to_tokens(tokens);
        }
    }

    impl ToTokens for UseGlob {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.star_token.to_tokens(tokens);
        }
    }

    impl ToTokens for UseGroup {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.brace_token.surround(tokens, |tokens| {
                self.items.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for TraitItemConst {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.const_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.colon_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
            if let Some((eq_token, default)) = &self.default {
                eq_token.to_tokens(tokens);
                default.to_tokens(tokens);
            }
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for TraitItemMethod {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.sig.to_tokens(tokens);
            match &self.default {
                Some(block) => {
                    block.brace_token.surround(tokens, |tokens| {
                        tokens.append_all(self.attrs.inner());
                        tokens.append_all(&block.stmts);
                    });
                }
                None => {
                    TokensOrDefault(&self.semi_token).to_tokens(tokens);
                }
            }
        }
    }

    impl ToTokens for TraitItemType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.type_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            if !self.bounds.is_empty() {
                TokensOrDefault(&self.colon_token).to_tokens(tokens);
                self.bounds.to_tokens(tokens);
            }
            self.generics.where_clause.to_tokens(tokens);
            if let Some((eq_token, default)) = &self.default {
                eq_token.to_tokens(tokens);
                default.to_tokens(tokens);
            }
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for TraitItemMacro {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.mac.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ImplItemConst {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.defaultness.to_tokens(tokens);
            self.const_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.colon_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
            self.eq_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ImplItemMethod {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.defaultness.to_tokens(tokens);
            self.sig.to_tokens(tokens);
            self.block.brace_token.surround(tokens, |tokens| {
                tokens.append_all(self.attrs.inner());
                tokens.append_all(&self.block.stmts);
            });
        }
    }

    impl ToTokens for ImplItemType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.defaultness.to_tokens(tokens);
            self.type_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
            self.eq_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ImplItemMacro {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.mac.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ForeignItemFn {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.sig.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ForeignItemStatic {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.static_token.to_tokens(tokens);
            self.mutability.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.colon_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ForeignItemType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.vis.to_tokens(tokens);
            self.type_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    impl ToTokens for ForeignItemMacro {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.mac.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
        }
    }

    fn has_variadic(inputs: &Punctuated<FnArg, Token![,]>) -> bool {
        let last = match inputs.last() {
            Some(last) => last,
            None => return false,
        };

        let pat = match last {
            FnArg::Typed(pat) => pat,
            FnArg::Receiver(_) => return false,
        };

        let tokens = match pat.ty.as_ref() {
            Type::Verbatim(tokens) => tokens,
            _ => return false,
        };

        tokens.to_string() == "..."
    }

    impl ToTokens for Signature {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.constness.to_tokens(tokens);
            self.asyncness.to_tokens(tokens);
            self.unsafety.to_tokens(tokens);
            self.abi.to_tokens(tokens);
            self.fn_token.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            self.generics.to_tokens(tokens);
            self.paren_token.surround(tokens, |tokens| {
                self.inputs.to_tokens(tokens);
                if self.variadic.is_some() && !has_variadic(&self.inputs) {
                    if !self.inputs.empty_or_trailing() {
                        <Token![,]>::default().to_tokens(tokens);
                    }
                    self.variadic.to_tokens(tokens);
                }
            });
            self.output.to_tokens(tokens);
            self.generics.where_clause.to_tokens(tokens);
        }
    }

    impl ToTokens for Receiver {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            match &self.reference {
                Reference::None(mutability) => {
                    mutability.to_tokens(tokens);
                    self.self_token.to_tokens(tokens);
                },
                Reference::Partial(dot, partial_borrows) => {
                    self.self_token.to_tokens(tokens);
                    dot.to_tokens(tokens);
                    partial_borrows.to_tokens(tokens);
                },
                Reference::Full(ampersand, lifetime, mutability) => {
                    ampersand.to_tokens(tokens);
                    lifetime.to_tokens(tokens);
                    mutability.to_tokens(tokens);
                    self.self_token.to_tokens(tokens);
                }
            }
        }
    }
}
