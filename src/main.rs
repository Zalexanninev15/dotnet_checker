use execute::Execute;
use std::process;
use std::process::Command;
use try_catch::catch;

mod frameworks;
mod windows;

fn main() {
    winconsole::console::set_title("Universal .NET components checker").unwrap();
    println!("Universal .NET components checker v1.0 by Zalexanninev15\n");
    if windows::is_app_elevated() {
        println!("Default version of .NET:");
        dotnet_check("--version");
        println!();

        println!(".NET Core 3.1 Runtimes and later (simple .NET):");
        dotnet_check("--list-runtimes");
        println!();

        println!(".NET Core 3.1 SDKs and later (simple .NET):");
        dotnet_check("--list-sdks");
        println!();

        println!("All components of .NET Core 3.1 and later (simple .NET):");
        dotnet_check("--info");
        println!();

        println!(".NET Framework 4.5 or later:");
        catch! {
            try {
        frameworks::get_45_plus_from_registry();
            }
            catch _error {
                println!(".NET Framework 4.5 or later is not detected!")
            }
        }
        println!();

        println!(".NET Framework 1.0-4.0:");
        catch! {
            try {
        frameworks::get_version_from_registry();
            }
            catch _error {
                println!(".NET Framework 1.0-4.0 is not detected!")
            }
        }
        println!();

        press_btn_continue::wait("Press any key to exit...").unwrap();
        process::exit(0);
    } else {
        press_btn_continue::wait("Administrator rights are required to run!").unwrap();
        process::exit(1);
    }
}

// Gets the version of .NET and .NET Core
fn dotnet_check(argument: &str) {
    catch! {
        try {
        let mut command = Command::new("dotnet");
        command.arg(argument);
        let _output = command.execute_output();
        }
        catch _error {
            println!(".NET or .NET Core is not detected.")
        }
    }
}
