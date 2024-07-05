use std::fs;

pub struct Grep {
    // Accepts the user's target string, the flag and file name for searching.
    pub target: String,
    pub flag: String,
    pub file_name: String,
}

impl Grep {
    // A constructor function to add the user's input to the struct.
    pub fn set_details(args: &[String]) -> Grep {
        let target = args[1].clone();
        let flag = args[2].clone();
        let file_name = args[3].clone();

        Grep{target, flag, file_name}
    }

    pub fn grep(&self) {
        // Opens and reads the file, and establishes a variable to count the lines of the text file.
        let file_content = fs::read_to_string(&self.file_name).expect("Failed to read file.");
        let mut line_num = 1;
        
        // Matches the chosen flag to the correct function, and panics if it's invalid.
        match self.flag.as_ref() {
            // Reads through the file, and if the string is present anywhere, it confirms that to the user.
            // If not, it informs the user that it isn't present.
            "-l" => {
                let mut string_presence: bool = false;

                for line in file_content.lines() {
                    if line.contains(&self.target) {
                        string_presence = true;
                    }
                }

                if string_presence {println!("The string is present in the file.")}
                else {println!("The string is not present in the file.")}
            },

            // Reads through the file, and if a line contains the string, it prints the line and the line number to the user.
            // If not, it confirms this to the user. Case-sensitive.
            "-n" => {
                let mut string_presence: bool = false;

                for line in file_content.lines() {
                    if line.contains(&self.target) {
                        println!("Line no. {}: {}", line_num, line);
                        string_presence = true;
                    }

                    line_num += 1;
                }

                if !string_presence {println!("The string is not present in the file.")}
            },

            // Does the same thing as the previous command, but is case-insensitive.
            "-i" => {
                let mut string_presence: bool = false;

                for line in file_content.lines() {
                    if line.contains(&self.target) || line.to_uppercase().contains(&self.target.to_uppercase()) || line.to_lowercase().contains(&self.target.to_lowercase()) {
                        println!("Line no. {}: {}", line_num, line);
                        string_presence = true;
                    }

                    line_num += 1;
                }

                if !string_presence {println!("The string is not present in the file.")}
            },

            // Carries out a similar function to the '-n' command.
            // It, however, prints the line and line number to the user if it doesn't contain the string.
            "-v" => {
                let mut string_absence: bool = false;

                for line in file_content.lines() {
                    if !line.contains(&self.target) {
                        println!("Line no. {}: {}", line_num, line);
                        string_absence = true;
                    }

                    line_num += 1;
                }

                if !string_absence {println!("The string is present in all lines of the file.")}
            },

            // Panics if the user sends an invalid flag.
            _ => panic!("Unsupported flag type."),
        }
        
    }
}
