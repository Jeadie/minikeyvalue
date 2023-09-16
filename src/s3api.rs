use serde::Deserialize;
use std::io::Read;
use quick_xml::de::from_reader;

#[derive(Debug, Deserialize)]
struct CompleteMultipartUpload {
    #[serde(rename = "Part")]
    parts: Vec<Part>,
}

#[derive(Debug, Deserialize)]
struct Part {
    #[serde(rename = "PartNumber")]
    part_number: i32,
    #[serde(rename = "ETag")]
    e_tag: String,
}

#[derive(Debug, Deserialize)]
struct Delete {
    #[serde(rename = "Object")]
    objects: Vec<Object>,
}

#[derive(Debug, Deserialize)]
struct Object {
    #[serde(rename = "Key")]
    key: String,
}

fn parse_xml<T: DeserializeOwned, R: Read>(reader: R) -> Result<T, quick_xml::DeError> {
    from_reader(reader)
}

fn parse_complete_multipart_upload<R: Read>(reader: R) -> Result<CompleteMultipartUpload, quick_xml::DeError> {
    parse_xml(reader)
}

fn parse_delete<R: Read>(reader: R) -> Result<Delete, quick_xml::DeError> {
    parse_xml(reader)
}