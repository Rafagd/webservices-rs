use std::collections::HashMap;

extern crate sxd_document;
use self::sxd_document::Package;
use self::sxd_document::writer::format_document;

use soap::{ Fault, Part };

pub struct Response {
    pub operation: String,
    pub responses: HashMap<String, Part>,
        fault:     Option<Fault>,
}

impl Response {
    pub fn new() -> Response {
        Response {
            operation: String::new(),
            responses: hashmap!{},
            fault:     None,
        }
    }

    pub fn fault(&mut self, fault: Fault) {
        self.fault = Some(fault);
    }

    pub fn to_xml_string(&self) -> String {
        let package  = Package::new();
        let document = package.as_document();

        let envelope = document.create_element("SOAP-ENV:Envelope");
        envelope.set_attribute_value("xmlns:xsd", "http://www.w3.org/2001/XMLSchema");
        envelope.set_attribute_value("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance");
        envelope.set_attribute_value("xmlns:SOAP-ENV", "http://schemas.xmlsoap.org/soap/envelope/");

        let body = document.create_element("SOAP-ENV:Body");

        let mut res_name = String::from("ns1:");
        res_name.push_str(self.operation.as_str());
        res_name.push_str("Response");

        let res = document.create_element(res_name.as_str());

        for (_, part) in self.responses.iter() {
            let ret = document.create_element("return");
            ret.set_attribute_value("xsi:type", part.xsd_type().as_str());

            let content = match part {
                &Part::String(ref string) => string.clone(),
                _ => String::new(),
            };

            if content != "" {
                ret.append_child(document.create_text(content.as_str()));
            }
            
            res.append_child(ret);
        }

        if let Some(ref fault) = self.fault {
            let res_fault = fault.to_xml(&document);
            body.append_child(res_fault);
        }

        body.append_child(res);
        envelope.append_child(body);
        document.root().append_child(envelope);

        let mut buffer = vec![];
        format_document(&document, &mut buffer).ok()
            .expect("Error while formatting SOAP XML");
                    
        String::from_utf8(buffer).unwrap()
    }
}

