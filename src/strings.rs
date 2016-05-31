pub const MAIN: &'static str =
r#"#include <iostream>

int main( void ) {
	std::cout << "Hello, world!" << std::endl;

	return EXIT_SUCCESS;
}"#;

pub fn make_header(proj_name: &str) -> String {
	format!(r#"#pragma once

#ifndef __{proj_name}_HEADER_COMMON_HPP
#define __{proj_name}_HEADER_COMMON_HPP

// text goes here

#endif
"#, proj_name=proj_name.to_uppercase())

}

pub const GITIGNORE: &'static str = 
r#"# Compiled Object files
*.slo
*.lo
*.o
*.obj

# Precompiled Headers
*.gch
*.pch

# Compiled Dynamic libraries
*.so
*.dylib
*.dll

# Fortran module files
*.mod

# Compiled Static libraries
*.lai
*.la
*.a
*.lib

# Executables
*.exe
*.out
*.app

target/
"#;