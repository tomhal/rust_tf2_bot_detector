use crate::rcon::RConArgs;
use std::process::{Child, Command};

/*

"C:\Program Files (x86)\Steam\steamapps\common\Team Fortress 2\hl2.exe" -steam -game tf  -usercon -high +developer 1 +alias developer +contimes 0 +alias contimes +ip 0.0.0.0 +alias ip +sv_rcon_whitelist_address 127.0.0.1 +alias sv_rcon_whitelist_address +sv_quota_stringcmdspersecond 1000000 +alias sv_quota_stringcmdspersecond +rcon_password rconpwd +alias rcon_password +hostport 40434 +alias hostport +alias cl_reload_localization_files +net_start +con_timestamp 1 +alias con_timestamp -condebug -conclearlog -novid -nojoy -nosteamcontroller -nohltv -particles 1 -console -full -w 2560 -h 1440 +cl_cmdrate 66 +cl_updaterate 25000 +cl_interp 0 +cl_interp_ratio 1 +m_rawinput 1 +zoom_sensitivity_ratio 0.7  +fps_max 300

*/

#[derive(Debug)]
pub struct Tf2ProcessArgs {
    pub exe_path: String,
}

impl Tf2ProcessArgs {
    pub fn new() -> Self {
        Tf2ProcessArgs {
            exe_path: r"C:\Program Files (x86)\Steam\steamapps\common\Team Fortress 2".to_string(),
            // other_args: r"-novid -nojoy -nosteamcontroller -nohltv -particles 1 -console -full -w 2560 -h 1440".to_string()
        }
    }
}

#[derive(Debug)]
pub struct Tf2Process {
    process: Child,
}

impl Tf2Process {
    pub fn start(control_args: &Tf2ProcessArgs, rcon_args: &RConArgs) -> Tf2Process {
        let complete_exe_path = format!("{}\\hl2.exe", control_args.exe_path);
        println!("Starting the TF2 process in the background...");

        // https://developer.valvesoftware.com/wiki/List_of_TF2_console_commands_and_variables

        let child = Command::new(complete_exe_path)
            .arg("-steam")
            .arg("-game")
            .arg("tf")
            .arg("-novid")
            .arg("-console")
            .arg("-usercon")
            .arg("+rcon_password")
            .arg(rcon_args.password.clone())
            .arg("+hostport")
            .arg(format!("{}", rcon_args.port))
            .arg("+ip")
            .arg("0.0.0.0")
            .arg("+sv_rcon_whitelist_address")
            .arg("127.0.0.1")
            .arg("+fpsmax")
            .arg("300")
            //
            // Remove commands via alias
            //
            .arg("+alias")
            .arg("rcon_password")
            .arg("+alias")
            .arg("hostport")
            .arg("+alias")
            .arg("ip")
            .arg("+alias")
            .arg("sv_rcon_whitelist_address")
            //
            // Start the TF2 process
            //
            .spawn()
            // Panic the process if something went wrong
            .expect("Failed to start the TF2 process");

        Tf2Process { process: child }
    }

    pub fn wait(mut self) {
        println!("Waiting for the TF2 process to exit...");
        self.process
            .wait()
            .expect("Unexpected TF2 process problem.");
        println!("The TF2 process has exited.");
    }
}
