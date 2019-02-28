/*
This code is part of the WhiteboxTools geospatial analysis library.
Authors: Dr. John Lindsay
Created: June 21, 2017
Last Modified: 07/11/2018
License: MIT
*/

/*!
WhiteboxTools is an advanced geospatial data analysis platform developed at
the University of Guelph's Geomorphometry and Hydrogeomatics Research Group (GHRG).

WhiteboxTools is a command-line program and can be run either by calling it,
with appropriate commands and arguments, from a terminal application, or, more
conveniently, by calling it from a script. The following commands are recognized
by the WhiteboxTools library:

| Command           | Description                                                                                       |
| ----------------- | ------------------------------------------------------------------------------------------------- |
| --cd, --wd        | Changes the working directory; used in conjunction with --run flag.                               |
| -h, --help        | Prints help information.                                                                          |
| -l, --license     | Prints the whitebox-tools license.                                                                |
| --listtools       | Lists all available tools, with tool descriptions. Keywords may also be used, --listtools slope.  |
| -r, --run         | Runs a tool; used in conjunction with --cd flag; -r="LidarInfo".                                  |
| --toolbox         | Prints the toolbox associated with a tool; --toolbox=Slope.                                       |
| --toolhelp        | Prints the help associated with a tool; --toolhelp="LidarInfo".                                   |
| --toolparameters  | Prints the parameters (in json form) for a specific tool; --toolparameters=\"LidarInfo\".         |
| -v                | Verbose mode. Without this flag, tool outputs will not be printed.                                |
| --viewcode        | Opens the source code of a tool in a web browser; --viewcode=\"LidarInfo\".                       |
| --version         | Prints the version information.                                                                   |

*/


pub mod algorithms;
pub mod lidar;
pub mod raster;
pub mod rendering;
pub mod spatial_ref_system;
pub mod structures;
pub mod tools;
pub mod utils;
pub mod vector;

use crate::tools::ToolManager;
use nalgebra as na;
use std::env;
use std::io::Error;
use std::path;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate clap;

use clap::{App, Arg};
/// WhiteboxTools is an advanced geospatial data analysis engine.
///
/// # Examples
///
/// From the command line prompt, *WhiteboxTools* can be called to run a tool as follows:
///
/// ```
/// >>./whitebox_tools --wd='/Users/johnlindsay/Documents/data/' --run=DevFromMeanElev --input='DEM clipped.dep' --output='DEV raster.dep' -v
/// ```

fn main() {
    match run() {
        Ok(()) => {}
        Err(err) => panic!("{}", err),
    }
}

// // This is just used for testing new features.
// fn main() {
//     // let file_name = "/Users/johnlindsay/Documents/data/whitebox_cities.shp";
//     let file_name = "/Users/johnlindsay/Documents/data/world_map.shp";
//     // let file_name = "/Users/johnlindsay/Documents/data/Minnesota/HUC07inside08_buff25m.shp";

//     let sf = vector::Shapefile::new(file_name, "r").unwrap();
//     println!("{}", sf.header);
//     for i in 0..sf.num_records {
//         println!("id={}, {:?}", i, sf.get_record(i).points);
//     }
// }

fn run() -> Result<(), Error> {
    let sep: &str = &path::MAIN_SEPARATOR.to_string();
    let mut working_dir = String::new();
    let mut tool_name = String::new();
    let mut run_tool = false;
    let mut tool_help = false;
    let mut tool_parameters = false;
    let mut toolbox = false;
    let mut list_tools = false;
    let mut keywords: Vec<String> = vec![];
    let mut view_code = false;
    let mut tool_args_vec: Vec<String> = vec![];
    let mut verbose = false;
    let mut finding_working_dir = false;

    let _matches = App::new("WhiteboxTools")
        .version(crate_version!())
        .author("Dr. John Lindsay")
        .about("WhiteboxTools is an advanced geospatial data analysis platform developed at the University of Guelph's Geomorphometry and Hydrogeomatics Research Group (GHRG)")
        .arg(Arg::with_name("cd")
            .visible_alias("wd")
        ).arg(Arg::with_name("license")
            .short("l")
        ).arg(Arg::with_name("listtools")
            .visible_alias("list_tools")
        ).arg(Arg::with_name("run")
            .short("r")
            .takes_value(true)
            .multiple(true)
        ).arg(Arg::with_name("toolbox")
            .takes_value(true)
        ).arg(Arg::with_name("toolhelp")
            .takes_value(true)
        ).arg(Arg::with_name("toolparameters")
            .takes_value(true)
        ).arg(Arg::with_name("verbose")
            .short("v")
        ).arg(Arg::with_name("viewcode")
        ).arg(Arg::with_name("version")
            .short("V")
        )
        .get_matches();


    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        version();
        // print help
        help();
        // list tools
        let tm = ToolManager::new(&working_dir, &verbose)?;
        tm.list_tools();

        return Ok(());
    }
    for arg in args {
        let flag_val = arg.to_lowercase().replace("--", "-");
        if flag_val == "-h" || flag_val == "-help" {
            help();
            return Ok(());
        } else if flag_val.starts_with("-cd") || flag_val.starts_with("-wd") {
            let mut v = arg
                .replace("--cd", "")
                .replace("--wd", "")
                .replace("-cd", "")
                .replace("-wd", "")
                .replace("\"", "")
                .replace("\'", "");
            if v.starts_with("=") {
                v = v[1..v.len()].to_string();
            }
            if v.trim().is_empty() {
                finding_working_dir = true;
            }
            if !v.ends_with(sep) {
                v.push_str(sep);
            }
            working_dir = v.to_string();
        } else if arg.starts_with("-run") || arg.starts_with("--run") || arg.starts_with("-r") {
            let mut v = arg
                .replace("--run", "")
                .replace("-run", "")
                .replace("-r", "")
                .replace("\"", "")
                .replace("\'", "");
            if v.starts_with("=") {
                v = v[1..v.len()].to_string();
            }
            tool_name = v;
            run_tool = true;
        } else if arg.starts_with("-toolhelp") || arg.starts_with("--toolhelp") {
            let mut v = arg
                .replace("--toolhelp", "")
                .replace("-toolhelp", "")
                .replace("\"", "")
                .replace("\'", "");
            if v.starts_with("=") {
                v = v[1..v.len()].to_string();
            }
            tool_name = v;
            tool_help = true;
        } else if arg.starts_with("-toolparameters") || arg.starts_with("--toolparameters") {
            let mut v = arg
                .replace("--toolparameters", "")
                .replace("-toolparameters", "")
                .replace("\"", "")
                .replace("\'", "");
            if v.starts_with("=") {
                v = v[1..v.len()].to_string();
            }
            tool_name = v;
            tool_parameters = true;
        } else if arg.starts_with("-toolbox") || arg.starts_with("--toolbox") {
            let mut v = arg
                .replace("--toolbox", "")
                .replace("-toolhelp", "")
                .replace("\"", "")
                .replace("\'", "");
            if v.starts_with("=") {
                v = v[1..v.len()].to_string();
            }
            tool_name = v;
            toolbox = true;
        } else if arg.starts_with("-listtools")
            || arg.starts_with("--listtools")
            || arg.starts_with("-list_tools")
            || arg.starts_with("--list_tools")
        {
            list_tools = true;
        } else if arg.starts_with("-viewcode") || arg.starts_with("--viewcode") {
            let mut v = arg
                .replace("--viewcode", "")
                .replace("-viewcode", "")
                .replace("\"", "")
                .replace("\'", "");
            if v.starts_with("=") {
                v = v[1..v.len()].to_string();
            }
            tool_name = v;
            view_code = true;
        } else if arg.starts_with("-license")
            || arg.starts_with("-licence")
            || arg.starts_with("--license")
            || arg.starts_with("--licence")
            || arg.starts_with("-l")
        {
            license();
            return Ok(());
        } else if arg.starts_with("-version") || arg.starts_with("--version") {
            version();
            return Ok(());
        } else if arg.trim() == "-v" {
            verbose = true;
        } else if arg.starts_with("-") {
            // it's an arg to be fed to the tool
            if !arg.contains("-17976931348623157") {
                // The QGIS plugin doesn't seem to handle numerical arguments that don't supply default values very well.
                // When this is the case, it will use an extremely large negative value, starting with the sequence above,
                // as the default. So if this number occurs in the argument, it means that the value was unspecified. If
                // it's an optional parameter, the tool will be able to handle this situation. If not, an error will likely
                // be thrown by the absence of the parameter.
                tool_args_vec.push(arg.trim().to_string().clone());
            }
        } else if !arg.contains("whitebox_tools") {
            // add it to the keywords list
            keywords.push(
                arg.trim()
                    .replace("\"", "")
                    .replace("\'", "")
                    .to_string()
                    .clone(),
            );
            if finding_working_dir {
                working_dir = arg.trim().to_string().clone();
                finding_working_dir = false;
            } else if tool_args_vec.len() > 0 {
                tool_args_vec.push(arg.trim().to_string().clone());
            }
        }
    }

    let sep = path::MAIN_SEPARATOR;
    if !working_dir.ends_with(sep) {
        working_dir.push_str(&(sep.to_string()));
    }
    let tm = ToolManager::new(&working_dir, &verbose)?;
    if run_tool {
        if tool_name.is_empty() && keywords.len() > 0 {
            tool_name = keywords[0].clone();
        }
        return tm.run_tool(tool_name, tool_args_vec);
    } else if tool_help {
        if tool_name.is_empty() && keywords.len() > 0 {
            tool_name = keywords[0].clone();
        }
        return tm.tool_help(tool_name);
    } else if tool_parameters {
        if tool_name.is_empty() && keywords.len() > 0 {
            tool_name = keywords[0].clone();
        }
        return tm.tool_parameters(tool_name);
    } else if toolbox {
        if tool_name.is_empty() && keywords.len() > 0 {
            tool_name = keywords[0].clone();
        }
        if tool_name.is_empty() {
            tool_name = String::new();
        }
        return tm.toolbox(tool_name);
    } else if list_tools {
        if keywords.len() == 0 {
            tm.list_tools();
        } else {
            tm.list_tools_with_keywords(keywords);
        }
    } else if view_code {
        if tool_name.is_empty() && keywords.len() > 0 {
            tool_name = keywords[0].clone();
        }
        return tm.get_tool_source_code(tool_name);
    }

    Ok(())
}

fn help() {
    let mut ext = "";
    if cfg!(target_os = "windows") {
        ext = ".exe";
    }

    let exe_name = &format!("whitebox-tools{}", ext);
    let sep: String = path::MAIN_SEPARATOR.to_string();
    let s = "WhiteboxTools Help

The following commands are recognized:
--cd, --wd       Changes the working directory; used in conjunction with --run flag.
-h, --help       Prints help information.
-l, --license    Prints the whitebox-tools license.
--listtools      Lists all available tools. Keywords may also be used, --listtools slope.
-r, --run        Runs a tool; used in conjuction with --wd flag; -r=\"LidarInfo\".
--toolbox        Prints the toolbox associated with a tool; --toolbox=Slope.
--toolhelp       Prints the help associated with a tool; --toolhelp=\"LidarInfo\".
--toolparameters Prints the parameters (in json form) for a specific tool; --toolparameters=\"LidarInfo\".
-v               Verbose mode. Without this flag, tool outputs will not be printed.
--viewcode       Opens the source code of a tool in a web browser; --viewcode=\"LidarInfo\".
--version        Prints the version information.

Example Usage:
>> .*EXE_NAME -r=lidar_info --cd=\"*path*to*data*\" -i=input.las --vlr --geokeys
"
            .replace("*", &sep)
            .replace("EXE_NAME", exe_name);
    println!("{}", s);
}

fn license() {
    let license_text = "WhiteboxTools License
Copyright 2017-2019 John Lindsay

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
associated documentation files (the \"Software\"), to deal in the Software without restriction,
including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial
portions of the Software.

THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES
OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.";
    println!("{}", license_text);
}

fn version() {
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    println!(
        "WhiteboxTools v{} by Dr. John B. Lindsay (c) 2017-2019

WhiteboxTools an advanced geospatial data analysis platform developed at
the University of Guelph's Geomorphometry and Hydrogeomatics Research 
Group (GHRG). See https://www.uoguelph.ca/~hydrogeo/WhiteboxTools/index.html 
for more details.",
        VERSION.unwrap_or("unknown")
    );
}
