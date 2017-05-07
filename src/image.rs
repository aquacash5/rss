// This file is part of rss.
//
// Copyright © 2015-2017 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use error::Error;

use fromxml::FromXml;
use quick_xml::{Element, Event, XmlReader, XmlWriter};
use quick_xml::error::Error as XmlError;
use toxml::{ToXml, XmlWriterExt};

/// A representation of the `<image>` element.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Image
{
    /// The URL of the channel image.
    pub url: String,
    /// A description of the image. This is used in the HTML `alt` attribute.
    pub title: String,
    /// The URL that the image links to.
    pub link: String,
    /// The width of the image.
    pub width: Option<String>,
    /// The height of the image.
    pub height: Option<String>,
    /// The text for the HTML `title` attribute.
    pub description: Option<String>,
}

impl FromXml for Image
{
    fn from_xml<R: ::std::io::BufRead>(mut reader: XmlReader<R>,
                                       _: Element)
        -> Result<(Self, XmlReader<R>), Error>
    {
        let mut url = None;
        let mut title = None;
        let mut link = None;
        let mut width = None;
        let mut height = None;
        let mut description = None;

        while let Some(e) = reader.next() {
            match e {
                Ok(Event::Start(element)) => {
                    match element.name() {
                        b"url" => url = element_text!(reader),
                        b"title" => title = element_text!(reader),
                        b"link" => link = element_text!(reader),
                        b"width" => width = element_text!(reader),
                        b"height" => height = element_text!(reader),
                        b"description" => description = element_text!(reader),
                        _ => skip_element!(reader),
                    }
                },
                Ok(Event::End(_)) => {
                    let url = url.unwrap_or_default();
                    let title = title.unwrap_or_default();
                    let link = link.unwrap_or_default();

                    return Ok((Image { url: url,
                                       title: title,
                                       link: link,
                                       width: width,
                                       height: height,
                                       description: description, },
                               reader));
                },
                Err(err) => return Err(err.into()),
                _ => {},
            }
        }

        Err(Error::EOF)
    }
}

impl ToXml for Image
{
    fn to_xml<W: ::std::io::Write>(&self,
                                   writer: &mut XmlWriter<W>)
        -> Result<(), XmlError>
    {
        let element = Element::new(b"image");

        writer.write(Event::Start(element.clone()))?;

        writer.write_text_element(b"url",
                                  &self.url)?;
        writer.write_text_element(b"title",
                                  &self.title)?;
        writer.write_text_element(b"link",
                                  &self.link)?;

        if let Some(width) = self.width.as_ref() {
            writer.write_text_element(b"width",
                                      width)?;
        }

        if let Some(height) = self.height.as_ref() {
            writer.write_text_element(b"height",
                                      height)?;
        }

        if let Some(description) = self.description.as_ref() {
            writer.write_text_element(b"description",
                                      description)?;
        }

        writer.write(Event::End(element))
    }
}
