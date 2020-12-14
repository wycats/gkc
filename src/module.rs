use std::{fs, path::PathBuf};

use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecma_ast::{Module, Program};
use swc_ecma_parser::{Parser, StringInput, Syntax, TsConfig, lexer::Lexer};

use crate::writer::Writer;

pub struct ModuleFile {
    filename: PathBuf,
}

impl ModuleFile {
    pub fn new(filename: impl Into<PathBuf>) -> ModuleFile {
        ModuleFile {
            filename: filename.into(),
        }
    }

    pub fn parse(self) -> ParsedModule {
        let file = fs::read(&self.filename)
            .unwrap_or_else(|_| panic!("Could't read file {}", &self.filename.display()));
        let map = Lrc::new(SourceMap::default());

        let fm = map.new_source_file(
            FileName::Real(self.filename),
            String::from_utf8_lossy(&file).into(),
        );

        let mut tsconfig = TsConfig::default();
        tsconfig.dynamic_import = true;

        let lexer = Lexer::new(
            Syntax::Typescript(tsconfig),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );
        let mut parser = Parser::new_from(lexer);
        // TODO better error message
        let module = parser.parse_module().unwrap_or_else(|m| panic!("{:?}", m));

        ParsedModule { module, map }
    }
}

pub struct ParsedModule {
    module: Module,
    map: Lrc<SourceMap>,
}

impl ParsedModule {
    pub fn emit(self) -> Vec<u8> {
        let writer = Writer::new(self.map);
        writer.emit(Program::Module(self.module))
    }
}
