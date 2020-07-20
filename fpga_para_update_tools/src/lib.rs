mod assemble_mb;
mod assemble_output;
mod assemble_weight;
pub mod config_setting{

    pub struct ConfigSetting {
        //for input
        input_dir_output: String,
        input_dir_mb: String,
        input_dir_weight: String,
    
        // for output
        output_dir: String,
        output_dir_weight: String,
        output_dir_mb: String,
    
        //for user rule config
        config_rule_path: String,
    }

    impl ConfigSetting {
        pub fn default() -> Self{
            ConfigSetting {
                // for input
                input_dir_output: String::from("./input_simulate/output/"),
                input_dir_mb: String::from("./intput_simulate/mb/"),
                input_dir_weight: String::from("./input_simulate/weight/"),
                //for output
                output_dir: String::from("./out/output/"),
                output_dir_weight: String::from("./out/weight/"),
                output_dir_mb: String::from("./out/mb/"),
                // for user rule config
                config_rule_path: String::from("./config/config_rule")
            }
        }

        pub fn get_input_dir_output(&self) -> &str {
            &self.input_dir_output
        }

        pub fn get_input_dir_mb(&self) -> &str {
            &self.input_dir_mb
        }
        
        pub fn get_input_dir_weight(&self) -> &str {
            &self.input_dir_weight
        }

        pub fn get_output_dir(&self) -> &str {
            &self.output_dir
        }
        
        pub fn get_output_weight(&self) -> &str {
            &self.output_dir_weight
        }

        pub fn get_output_mb(&self) -> &str {
            &self.output_dir_mb
        }

        pub fn get_config_rule_path(&self) -> &str {
            &self.config_rule_path
        }
    }

}