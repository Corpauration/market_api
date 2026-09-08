
syn::custom_keyword!(from);
syn::custom_keyword!(with);

#[derive(Debug)] #[derive(Clone, Copy)] #[derive(PartialEq, Eq)] #[derive(Hash)]
pub enum CsvProperty {
    Header(bool),
    Delimiter(u8),
}

impl syn::parse::Parse for CsvProperty {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        if ident == "header" {
            let value: syn::LitBool = input.parse()?;
            Ok(CsvProperty::Header(value.value))
        } else if ident == "delimiter" {
            let value: syn::LitChar = input.parse()?;
            if !value.value().is_ascii() {
                return Err(syn::Error::new(
                    value.span(),
                    "Delimiter must be an ASCII character",
                ));
            }
            Ok(CsvProperty::Delimiter(value.value() as u8))
        } else {
            Err(syn::Error::new(
                ident.span(),
                "Unexpected csv property",
            ))
        }
    }
}

#[derive(Clone)]
pub struct CsvConfigDeclaration {
    pub properties: syn::punctuated::Punctuated<CsvProperty, syn::Token![,]>,
}

impl syn::parse::Parse for CsvConfigDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let properties = syn::punctuated::Punctuated::parse_terminated(input)?;
        Ok(CsvConfigDeclaration { properties })
    }
}

#[derive(Debug)] #[derive(Clone, Copy)] #[derive(PartialEq, Eq)] #[derive(Hash)]
pub struct CsvConfigSpecification {
    pub header: Option<bool>,
    pub delimiter: Option<u8>,
}

impl Default for CsvConfigSpecification {
    fn default() -> Self {
        CsvConfigSpecification {
            header: None,
            delimiter: Some(b','),
        }
    }
}

impl From<CsvConfigDeclaration> for CsvConfigSpecification {
    fn from(declaration: CsvConfigDeclaration) -> Self {
        let mut specification = CsvConfigSpecification::default();
        for property in declaration.properties {
            match property {
                CsvProperty::Header(value) => specification.header = Some(value),
                CsvProperty::Delimiter(value) => specification.delimiter = Some(value),
            }
        }
        specification
    }
}

impl CsvConfigSpecification {
    pub fn override_with(self, other: CsvConfigSpecification) -> CsvConfigSpecification {
        CsvConfigSpecification {
            header: other.header.or(self.header),
            delimiter: other.delimiter.or(self.delimiter),
        }
    }
}

#[derive(Debug)] #[derive(Clone, Copy)] #[derive(PartialEq, Eq)] #[derive(Hash)]
pub struct CsvConfig {
    pub header: bool,
    pub delimiter: u8,
}

#[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, Eq)] #[derive(Hash)]
#[derive(thiserror::Error)] #[error("Missing property: {property_name}")]
pub struct CsvConfigMissingPropertyError {
    pub property_name: String,
}

impl TryFrom<CsvConfigSpecification> for CsvConfig {
    type Error = CsvConfigMissingPropertyError;

    fn try_from(specification: CsvConfigSpecification) -> Result<Self, Self::Error> {
        let header = specification.header.ok_or(CsvConfigMissingPropertyError {
            property_name: "header".into(),
        })?;
        let delimiter = specification.delimiter.ok_or(CsvConfigMissingPropertyError {
            property_name: "delimiter".into(),
        })?;

        Ok(CsvConfig { header, delimiter })
    }
}

impl syn::parse::Parse for CsvConfig {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let declaration = input.parse::<CsvConfigDeclaration>()?;
        let specification = CsvConfigSpecification::default().override_with(declaration.into());
        specification.try_into().map_err(|err: CsvConfigMissingPropertyError| {
            syn::Error::new(input.span(), format!("Missing property: {}", err.property_name))
        })
    }
}

#[derive(Clone)]
pub struct EnumFromCsv {
    pub visibility: syn::Visibility,
    pub name: syn::Ident,
    pub source_path: syn::LitStr,
    pub source_config: CsvConfig,
}

impl syn::parse::Parse for EnumFromCsv {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let visibility = input.parse()?;
        input.parse::<syn::Token![enum]>()?;
        let name = input.parse()?;
        input.parse::<from>()?;
        let source_path = input.parse()?;
        input.parse::<with>()?;
        let source_config = input.parse()?;
        Ok(EnumFromCsv {
            visibility,
            name,
            source_path,
            source_config,
        })
    }
}

#[derive(Clone)]
pub struct Variant {
    pub identifier: syn::Ident,
}

impl Variant {
    pub fn from_name(name: &str) -> Result<Self, syn::Error> {
        syn::parse_str::<syn::Ident>(name).map(|identifier| Self { identifier }).map_err(|err| {
            syn::Error::new(proc_macro2::Span::call_site(), format!("Invalid variant name '{}': {}", name, err))
        })
    }
}

impl quote::ToTokens for Variant {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let identifier = &self.identifier;
        tokens.extend(quote::quote! {
            #identifier,
        });
    }
}

#[derive(Clone)]
pub struct Enum {
    pub visibility: syn::Visibility,
    pub name: syn::Ident,
    pub variants: Vec<Variant>,
}

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum EnumFromCsvError {
    #[error("Failed to read CSV file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse CSV: {0}")]
    Csv(#[from] csv::Error),
    #[error("Failed to parse variant name: {0}")]
    VariantParse(#[from] syn::Error),
    #[error("Failed to resolve path: {0}")]
    PathResolve(#[from] std::env::VarError),
}

pub fn resolve_path_relative_to_cargo_manifest(path: &str) -> Result<std::path::PathBuf, std::env::VarError> {
    let path = std::path::Path::new(path);
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").ok_or(std::env::VarError::NotPresent)?;
        Ok(std::path::PathBuf::from(manifest_dir).join(path))
    }
}

impl TryFrom<EnumFromCsv> for Enum {
    type Error = EnumFromCsvError;

    fn try_from(r#enum: EnumFromCsv) -> Result<Self, Self::Error> {
        let source_path = resolve_path_relative_to_cargo_manifest(&r#enum.source_path.value())?;
        let csv_content = std::fs::read_to_string(source_path)?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(r#enum.source_config.header)
            .delimiter(r#enum.source_config.delimiter)
            .from_reader(csv_content.as_bytes());
        let mut variants = Vec::new();
        for result in reader.records() {
            let record = result?;
            if let Some(title) = record.get(0) {
                variants.push(Variant::from_name(title)?);
            }
        }
        Ok(Enum {
            visibility: r#enum.visibility,
            name: r#enum.name,
            variants,
        })
    }
}

impl quote::ToTokens for Enum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let visibility = &self.visibility;
        let name = &self.name;
        let variants = &self.variants;
        tokens.extend(quote::quote! {
            #visibility enum #name {
                #(#variants)*
            }
        });
    }
}

pub fn transform(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let enum_from_csv = syn::parse_macro_input!(input as EnumFromCsv);
    let r#enum = match Enum::try_from(enum_from_csv) {
        Ok(r#enum) => r#enum,
        Err(err) => {
            return syn::Error::new(proc_macro2::Span::call_site(), format!("Failed to read CSV file: {}", err))
                .to_compile_error()
                .into();
        }
    };
    let tokens = quote::quote! {
        #r#enum
    };
    tokens.into()
}
