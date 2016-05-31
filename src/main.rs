extern crate docopt;
extern crate rustc_serialize;
use docopt::Docopt;

mod makefile;
mod strings;

const DOC: &'static str = 
r#"cpp-new

Creates a new C++ project in the target directory.

USAGE:
		cpp-new <proj-name> [--ext=F]
        cpp-new --help

OPTIONS:
        -h --help   Show this screen.
        -e --ext=F  The extension for the source files, without the '.' [default: cxx]
"#;

#[derive(Clone,RustcEncodable,RustcDecodable,Debug,Hash,Eq,PartialEq)]
pub struct Args {
	arg_proj_name: String,
	flag_ext: String,
}

fn main() {
	use std::fs;
	use std::fs::File;
	use std::path::PathBuf;
	use std::io::Write;

	let args: Args = Docopt::new(DOC)
		.and_then(|d| d.decode())
		.unwrap_or_else(|e| e.exit());

	let mfile = makefile::make_makefile(&args);
	let header = strings::make_header(&args.arg_proj_name);

	let mut proj_path = PathBuf::from(args.arg_proj_name);

	// Scope this so all the files get closed at the end of this block
	{
		fs::create_dir_all(&*proj_path)
			.unwrap_or_else(|e| panic!("Cannot create project directory: {}", e));

		proj_path.push("libs");
		fs::create_dir_all(&*proj_path)
			.unwrap_or_else(|e| panic!("Cannot create libs directory: {}", e));

		proj_path.pop();
		proj_path.push("include");
		fs::create_dir_all(&*proj_path)
			.unwrap_or_else(|e| panic!("Cannot create include directory: {}", e));

		proj_path.push("common.hpp");

		let mut include_file = File::create(&*proj_path)
			.unwrap_or_else(|e| panic!("Could not create common.hpp: {}", e));

		include_file.write_all(header.as_bytes())
			.unwrap_or_else(|e| panic!("Could not write header file: {}", e));

		proj_path.pop();
		proj_path.pop();

		proj_path.push("src");
		fs::create_dir_all(&*proj_path)
			.unwrap_or_else(|e| panic!("Cannot create source directory: {}", e));
		
		proj_path.push(format!("main.{}", args.flag_ext));

		let mut source_file = File::create(&*proj_path)
			.unwrap_or_else(|e| panic!("Could not create main.cxx: {}", e));

		source_file.write_all(strings::MAIN.as_bytes())
			.unwrap_or_else(|e| panic!("Could not write main file: {}", e));

		proj_path.pop();
		proj_path.pop();

		proj_path.push("Makefile");

		let mut makefile = File::create(&*proj_path)
			.unwrap_or_else(|e| panic!("Could not create Makefile: {}", e));

		makefile.write_all(mfile.as_bytes())
			.unwrap_or_else(|e| panic!("Could not write Makefile: {}", e));

		proj_path.pop();

		proj_path.push(".gitignore");

		let mut gitignore = File::create(&*proj_path)
			.unwrap_or_else(|e| panic!("Could not create .gitignore: {}", e));

		gitignore.write_all(strings::GITIGNORE.as_bytes())
			.unwrap_or_else(|e| panic!("Could not write .gitignore: {}", e));
	}
}
