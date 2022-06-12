use std::fs;

// Scan - the 
pub trait Scan {
    fn scan(&mut self);
    fn read(&mut self, path: String);
}

// Scanner - contains errors and other information
// that can be bubbled up to the interpreter entrypoint
#[derive(Debug)]
pub struct Scanner {
    pub error: bool,
    pub source: String
} 

// Scan - the concrete implementation of the trait Scan
impl Scan for Scanner {

    // read - takes in a path to a file and reads the contents of the file
    fn read(&mut self, path: String) {
        self.source = fs::read_to_string(path)
            .expect("Something went wrong reading the file");
    }

    // scan - the beginning of a walking tree scanner
    // this will walk through source code line-by-line and then
    // 
    fn scan(&mut self) {        
        let lines = self.source.split("\n");
        // todo - figure out if clone is used properly here
        let len = lines.clone().count();

        // todo - need to figure out if I can
        // test if lines is empty
        if len <= 0 {
            println!("No source code found");
            self.error = true;
            return;
        } 

        for line in lines {    
            let tokens = line.split(" ");
            let len = tokens.clone().count();

            // todo - same as lines above
            if len <= 0 {
                println!("No source code found");
                self.error = true;
                return;
            }

            // for now just print out the tokens
            for token in tokens {
                println!("{}", token)
            }
        }
    }
}
