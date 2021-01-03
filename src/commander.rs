use std::future::Future;

pub struct Command<'a, Fut>
where
  Fut: Future<Output = ()>, {
  name: String,
  args: Vec<String>,
  desc: String,
  func: Option<&'a dyn Fn(Vec<String>)>,
  asyncfunc: Option<&'a dyn Fn(Vec<String>) -> Fut>
}

impl<'a, Fut> Command<'a, Fut>
where
  Fut: Future<Output = ()>, {
  pub fn new(name: String, args: Vec<String>) -> Command<'a, Fut> {
    return Command {
      name,
      args,
      desc: "No Description".to_string(),
      func: None,
      asyncfunc: None
    };
  }

  pub fn description(&mut self, desc: &str) -> &mut Command<'a, Fut> {
    self.desc = desc.to_string();

    return self;
  }

  pub fn asyncaction(&mut self, func: &'a dyn Fn(Vec<String>) -> Fut) -> &mut Command<'a, Fut>
  where
    Fut: Future<Output = ()>, {
    self.asyncfunc = Some(func);

    return self;
  }

  pub fn action(&mut self, func: &'a dyn Fn(Vec<String>) -> ()) -> &mut Command<'a, Fut> {
    self.func = Some(func);

    return self;
  }

  fn print(&self) {
    print!("  {}", self.name);
    for arg in &self.args {
      print!(" {}", arg);
    }

    println!("             {}", self.desc);
  }
}

pub struct Commands<'a, Fut>
where
  Fut: Future<Output = ()>, {
  name: String,
  desc: String,
  command_list: Vec<Command<'a, Fut>>
}

impl<'a, Fut> Commands<'a, Fut>
where
  Fut: Future<Output = ()>, {
  pub fn new(name: &str) -> Commands<'a, Fut> {
    return Commands {
      name: name.to_string(),
      desc: "No Description".to_string(),
      command_list: vec![]
    };
  }

  pub fn command(&mut self, cmdstr: &str) -> &mut Command<'a, Fut> {
    let words: Vec<String> = cmdstr.to_string().split_whitespace().map(|s| s.to_string()).collect();
    let cmdname = words[0].to_string();
    let args: Vec<String> = words.split_at(1).1.to_vec();

    let cmd = Command::new(cmdname, args);

    self.command_list.push(cmd);

    let i = self.command_list.len() - 1;

    return &mut self.command_list[i];
  }

  pub fn description(&mut self, desc: &str) -> &mut Commands<'a, Fut> {
    self.desc = desc.to_owned();

    return self;
  }

  fn help(&self) {
    println!("{}\n", self.desc);
    println!("Usage: {} [options] [command]", self.name);
    println!();
    println!("Option:");
    println!("  -h, --help");
    println!();
    println!("Commands:");

    for cmd in &self.command_list {
      cmd.print();
    }
  }

  fn parse_args(&self, argv: Vec<String>, cmdargs: Vec<String>) -> Option<Vec<String>> {
    let argcount = argv.len();
    let mut parsed_args: Vec<String> = vec![];

    let mut i = 0;

    for arg in cmdargs {
      let is_required = arg.starts_with("<") && arg.ends_with(">");

      if is_required && i >= argcount {
        return None;
      } else if i >= argcount {
        return Some(parsed_args);
      }

      let argument = argv[i].clone();

      parsed_args.push(argument);

      i += 1;
    }

    return Some(parsed_args);
  }

  pub async fn parse(&mut self, args: Vec<String>) {
    let argc = args.len();

    if argc < 2 {
      self.help();
      return;
    }

    let _main = &args[0];
    let cmdname = &args[1];
    let arguments = &args.split_at(2).1.to_vec();

    for cmd in &self.command_list {
      if cmd.name == *cmdname {
        let mut nofunc = false;
        let mut noasync = false;
        let mut missingargs = false;

        let parsed_args = self.parse_args(arguments.clone(), cmd.args.clone());

        match parsed_args {
          Some(pargs) => {
            match cmd.func {
              Some(func) => func(pargs.clone()),
              None => nofunc = true
            }
            match cmd.asyncfunc {
              Some(afunc) => afunc(pargs.clone()).await,
              None => noasync = true
            }
          },
          None => {
            missingargs = true;
            println!("Missing arguments:");
            cmd.print();
          }
        }

        if nofunc && noasync && !missingargs {
          println!("Not implemented yet.");
        }
      } else if *cmdname == "--help" || *cmdname == "-h" {
        self.help();
      }
    }
  }
}