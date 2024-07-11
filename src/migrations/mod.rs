use std::{ fs::File, io::Read, path::PathBuf, process::exit
};
#[derive(Debug)]
pub struct Migarion(pub Vec<String>);


impl  From<PathBuf> for Migarion{
    fn from(path: PathBuf) -> Self {
        let mut file=File::open(path).unwrap_or_else(|err|{
            log::error!("Open file with migration Error: {:?}", err);
            exit(1);
        });
        let mut buff=String::new();
        file.read_to_string(&mut buff).unwrap_or_else(|err|{
            log::error!("Readig file with migration Error: {:?}", err);
            exit(1);

        });
        let mut migrations=Vec::new();
        buff.split('#').for_each(|migraion|{
            if !migraion.is_empty(){
                    migrations.push(migraion.to_owned())
            }
        });
    Self(migrations)
    }
}