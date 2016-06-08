//! Commands for `budget`

#[derive(Debug)]
pub enum Command {
    /// Setup is for generating the configuration
    Setup,
    /// Status, the default command if a working config exists, simply reports the status of the
    /// budget account to the user
    Status,
    /// OneTime creates a one-time transaction
    OneTime(i64),
    /// Recurring adds a recurring transaction
    Recurring(i64, i8),
    /// Remove destroys an exsisting recurring transaction, by ID
    Remove(i64),
    /// List lists all existing recurring transactions, including ID
    List,
}

/// Given a tuple of one, two, or three arguments, decide which Command is being asked for and
/// parse its arguments
pub fn args_to_command(args: (Option<&str>, Option<&str>, Option<&str>)) -> 
                       Result<Command, String> 
    {
        // These three Strings will be the destinations for unwrapped version of the input
        let mut arg0: String;
        let mut arg1: String;
        let mut arg2: String;

        // These variables will be used to hold arguments, if we need them
        let mut famount: f64;
        let mut amount: i64;
        let mut day: i8;
        
        // Match on the first argument, the command
        match args.0 {
            // If no command was given, we simply need to return the status of the account
            None => return Ok(Command::Status),
            // Otherwise, we have a command and need to try to parse it
            Some(string) => arg0 = String::from(string.as_ref()).to_lowercase(),
        }

        // Match arg1 against all the commands that take no arguments
        // If it matches none of these, we can proceed to the next argument
        match arg0.as_ref() {
            "status" => return Ok(Command::Status),
            "setup" => return Ok(Command::Setup),
            "list" => return Ok(Command::List),
            _ => ()
        }

        // Every command that requires argument needs an i64, an amount of cents, as its first
        // argument.
        match args.1 { 
            // If we don't have a second argument, complain
            None => return Err(String::from("Missing required argument: amount.")),
            // If we do, just unwrap it into arg1
            Some(string) => arg1 = String::from(string.as_ref()).to_lowercase(),
        }
        
        // Check if we're running a Remove. It has different parsing requirements
        if &arg0 == "remove" {
            // For Remove, we parse the second arg directly to an i64 and then return
            match arg1.parse::<i64>() {
                Ok(n) => amount = n,
                Err(r) => return Err(String::from("Argument id was not valid: ") 
                                     + r.to_string().as_ref()),
            }
            // We're done, because Remove doesn't require another argument
            return Ok(Command::Remove(amount));
        }

        // Now, we can parse it into the famount f64, because we know we are working with
        // an actual amount
        match arg1.parse::<f64>() {
            Ok(n) => famount = n,
            Err(r) => return Err(String::from("Argument amount was not valid: ") 
                                 + r.to_string().as_ref()),
        }

        // Now we need to convert the float number of dollars to an int number of cents
        // f64 as i64 rounds towards 0
        amount = (famount * 100.0) as i64;

        // One last thing to check. If we are adding a recurring transaction, we need to
        //  parse the final argument; otherwise, we're done.
        match arg0.as_ref() {
            "spend" => return Ok(Command::OneTime(-amount)),
            "tip" => return Ok(Command::OneTime(amount)),
            // we know we're not running Remove because we checked it before
            _ => ()
        }

        // At this point we've returned unless we're adding a recurring transaction, so
        //  we need to go ahead and do the parsing for that
        match args.2 { 
            // If we don't have a third argument, complain
            None => return Err(String::from("Missing required argument: day.")),
            // If we do, just unwrap it into arg2
            Some(string) => arg2 = String::from(string.as_ref()).to_lowercase(),
        }

        // Parse the third argument into an i8
        match arg2.parse::<i8>() {
            Ok(n) => day = n,
            Err(r) => return Err(String::from("Argument day was not valid: ") 
                                 + r.to_string().as_ref()),
        }

        // If we've made it to this point, we're done.
        match arg0.as_ref() {
            "income" => return Ok(Command::Recurring(amount, day)),
            "expense" => return Ok(Command::Recurring(-amount, day)),
            _ => return Err(String::from("Unknown command: ") + arg0.as_ref()),
        }

    }
    

