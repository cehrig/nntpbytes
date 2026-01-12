use crate::decoder::decoder::{Encode, ExpectedResponse};
use crate::decoder::ExpectedResponseCode;
use crate::messages::{Decode, Decoder, ResponseCodeTuples};
use crate::{Error, Result};
use bytes::{BufMut, BytesMut};
use std::io::Write;
use std::str::FromStr;

pub struct ListRequest {
    keyword: Option<String>,
    arg: Option<String>,
}

#[derive(Default)]
pub struct GroupListResponse {
    groups: Vec<Group>,
}

#[derive(Default)]
pub struct GroupTimesResponse {
    groups: Vec<GroupTimes>,
}

#[derive(Default)]
pub struct GroupNewsgroupResponse {
    groups: Vec<GroupNewsgroup>,
}

#[derive(Debug, Copy, Clone)]
pub enum GroupStatus {
    PostingPermitted,
    PostingNotPermitted,
    Moderated,
}

pub struct Group {
    name: String,
    high: usize,
    low: usize,
    status: GroupStatus,
}

pub struct GroupTimes {
    name: String,
    age: usize,
    creator: String,
}

pub struct GroupNewsgroup {
    name: String,
    description: String,
}

impl ListRequest {
    pub fn new(keyword: Option<impl ToString>, arg: Option<impl ToString>) -> Self {
        Self {
            keyword: keyword.map(|k| k.to_string()),
            arg: arg.map(|a| a.to_string()),
        }
    }
}

impl GroupListResponse {
    pub fn groups(&self) -> &Vec<Group> {
        &self.groups
    }
}

impl GroupTimesResponse {
    pub fn groups(&self) -> &Vec<GroupTimes> {
        &self.groups
    }
}

impl GroupNewsgroupResponse {
    pub fn groups(&self) -> &Vec<GroupNewsgroup> {
        &self.groups
    }
}

impl Group {
    fn new(name: String, high: usize, low: usize, status: GroupStatus) -> Self {
        Self {
            name,
            high,
            low,
            status,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn high(&self) -> usize {
        self.high
    }

    pub fn low(&self) -> usize {
        self.low
    }

    pub fn status(&self) -> GroupStatus {
        self.status
    }
}

impl GroupTimes {
    fn new(name: String, age: usize, creator: String) -> Self {
        Self { name, age, creator }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> usize {
        self.age
    }

    pub fn creator(&self) -> &str {
        &self.creator
    }
}

impl GroupNewsgroup {
    fn new(name: String, description: String) -> Self {
        Self { name, description }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

impl Encode for ListRequest {
    fn encoder(&self, bytes: &mut BytesMut) -> Result<()> {
        write!(bytes.writer(), "LIST").map_err(Error::encode)?;

        if let Some(keyword) = &self.keyword {
            write!(bytes.writer(), " {}", keyword).map_err(Error::encode)?;

            if let Some(arg) = &self.arg {
                write!(bytes.writer(), " {}", arg).map_err(Error::encode)?;
            }
        }

        Ok(())
    }
}

impl ExpectedResponse for ListRequest {
    type Response = GroupListResponse;
}

impl ExpectedResponseCode for GroupListResponse {
    const CODES: ResponseCodeTuples = &[(215, true, true)];
}

impl Decode for GroupListResponse {
    fn decoder(&mut self, bytes: &mut Decoder, _: u16) -> Result<()>
    where
        Self: Sized,
    {
        // Discard first line
        let _ = bytes.line();

        while let Some(mut line) = bytes.line() {
            let name = line.get()?;
            let high = line.get()?;
            let low = line.get()?;
            let status = line.get()?;

            self.groups.push(Group::new(name, high, low, status));
        }

        Ok(())
    }
}

impl ExpectedResponseCode for GroupTimesResponse {
    const CODES: ResponseCodeTuples = &[(215, true, true)];
}

impl Decode for GroupTimesResponse {
    fn decoder(&mut self, bytes: &mut Decoder, _: u16) -> Result<()>
    where
        Self: Sized,
    {
        // Discard first line
        let _ = bytes.line();

        while let Some(mut line) = bytes.line() {
            let name = line.get()?;
            let age = line.get()?;
            let creator = line.get()?;

            self.groups.push(GroupTimes::new(name, age, creator));
        }

        Ok(())
    }
}

impl ExpectedResponseCode for GroupNewsgroupResponse {
    const CODES: ResponseCodeTuples = &[(215, true, true)];
}

impl Decode for GroupNewsgroupResponse {
    fn decoder(&mut self, bytes: &mut Decoder, _: u16) -> Result<()>
    where
        Self: Sized,
    {
        // Discard first line
        let _ = bytes.line();

        while let Some(mut line) = bytes.line() {
            let name = line.get()?;
            let description = line.get()?;

            self.groups.push(GroupNewsgroup::new(name, description));
        }

        Ok(())
    }
}

impl FromStr for GroupStatus {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "y" => Ok(GroupStatus::PostingPermitted),
            "n" => Ok(GroupStatus::PostingNotPermitted),
            "m" => Ok(GroupStatus::Moderated),
            _ => Err(Error::DecodeFromStr),
        }
    }
}
