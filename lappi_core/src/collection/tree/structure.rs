#[derive(Clone)]
pub struct ArtistFoldersDescription {
    pub folder_description: FolderDescriptor,
}

#[derive(Clone)]
pub struct TagFoldersDescription {
    pub tag: String,
    pub folder_description: FolderDescriptor,
}

#[derive(Clone)]
pub enum FolderContentDescriptor {
    Items,
    ArtistsFolders(ArtistFoldersDescription),
    TagFolders(TagFoldersDescription),
}

#[derive(Clone)]
pub struct FolderDescriptor {
    pub content: Box<FolderContentDescriptor>,
    //other_folders: Vec<FolderDescriptor>,
}

impl FolderDescriptor {

}

impl Default for FolderDescriptor {
    fn default() -> Self {
        let album_folder = FolderDescriptor {
            content: Box::new(FolderContentDescriptor::Items),
            //other_folders: vec![],
        };

        let author_folder = FolderDescriptor {
            content: Box::new(FolderContentDescriptor::TagFolders(TagFoldersDescription {
                tag: "album".to_string(),
                folder_description: album_folder,
            })),
        };

        let root = FolderDescriptor {
            content: Box::new(FolderContentDescriptor::ArtistsFolders(ArtistFoldersDescription {
                folder_description: author_folder,
            })),
        };

        return root;
    }
}
