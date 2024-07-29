use nickel_lang_core::{error::ExportErrorData, repl::query_print, serialize::{self, ExportFormat}, term::{record::Field, RichTerm, Term}};
use serde::ser::{Serialize, Serializer, SerializeStruct};

use crate::{
    cli::GlobalOptions,
    customize::{Customize, ExtractFieldOnly},
    error::{CliResult, Error, ResultErrorExt, Warning},
    input::{InputOptions, Prepare},
};

#[derive(clap::Parser, Debug)]
pub struct QueryCommand {
    #[arg(long)]
    pub doc: bool,

    #[arg(long)]
    pub contract: bool,

    #[arg(long = "type")]
    pub typ: bool,

    #[arg(long)]
    pub default: bool,

    #[arg(long)]
    pub value: bool,

    #[arg(long, short, value_enum)]
    pub format: Option<ExportFormat>,

    #[command(flatten)]
    pub inputs: InputOptions<ExtractFieldOnly>,
}

#[derive(Clone, Debug)]
struct QueryResult(pub Field);


impl Serialize for QueryResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut state = serializer.serialize_struct("QueryResult", 2)?;
        state.serialize_field("value", &self.0.value)?;
        state.serialize_field("metadata", &self.0.metadata)?;
        state.end()
    }
}

impl QueryCommand {
    fn attributes_specified(&self) -> bool {
        self.doc || self.contract || self.typ || self.default || self.value
    }

    fn query_attributes(&self) -> query_print::Attributes {
        // Use a default selection of attributes if no option is specified
        if !self.attributes_specified() {
            query_print::Attributes::default()
        } else {
            query_print::Attributes {
                doc: self.doc,
                contract: self.contract,
                typ: self.typ,
                default: self.default,
                value: self.value,
            }
        }
    }

    pub fn run(self, global: GlobalOptions) -> CliResult<()> {
        let mut program = self.inputs.prepare(&global)?;

        if self.inputs.customize_mode.field().is_none() {
            program.report(Warning::EmptyQueryPath, global.error_format);
        }

        match self.format {
            None => {
                let found = program
                    .query()
                    .map(|field| {
                        query_print::write_query_result(
                            &mut std::io::stdout(),
                            &field,
                            self.query_attributes(),
                        )
                        .unwrap()
                    })
                    .report_with_program(program)?;
        
                if !found {
                    eprintln!("No metadata found for this field.")
                }
            },
            Some(format) => {
                println!("format: {}", &format);
                let found = program
                    .query()
                    .map(|field| {
                        QueryResult(field)
                    });
                match found {
                    Ok(res) => {
                        println!("some field...");
                        serde_json::to_writer_pretty(std::io::stdout(), &res).map_err(|err| ExportErrorData::Other(err.to_string()));
                        // serialize::to_writer(std::io::stdout(), format, &rt)?;
                    },
                    _ => {eprintln!("some error...");}
                }
            },
        }

        Ok(())
    }
}
