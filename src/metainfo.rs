struct Metainfo {
    info: BObj,
    announce: String,
    announce_list: Option<Vec<Vec<String>>>,
    creation_date: Option<int>,
    comment: Option<String>,
    created_by: Option<(String, String)>,
    encoding: Option<String>,
}

struct CommonFileInfo {
    piece_length: uint,
    pieces: Vec<u8>,
    private: bool,
}

struct SingleFileInfo {
    name: String,
    length: uint,
    md5sum: String,
}

struct MultiFileInfo {
    name: String,
    files: Vec<MFIIndividualFile>,
}

struct MFIIndividualFile {
    length: uint,
    md5sum: String,
    path: Vec<String>,
}
