//! Workspace watcher is part the greater workspace system that i'll be building
//! to backup my stuff on gitlab in Private repos so that i get 10GB of space for each folder
//! ]since each folder will be a git repo.
//! what else should i add
pub fn workspace_watcher() {
    println!("Hello World!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        workspace_watcher();
        assert_eq!("usairim", "usairim");
        assert_eq!(2 + 2, 4);
        ////
    }
}
