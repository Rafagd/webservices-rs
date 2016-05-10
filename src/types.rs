#[derive(PartialEq)]
pub enum Type {
    Void,
    Int,
    Float,
    String,
}

pub enum Value {
    Null,
    Int(i32),
    Float(f32),
    String(String),
}

impl From<()> for Value {
    fn from(_other: ()) -> Value {
        Value::Null
    }
}

pub struct Data {
    pub name: String,
    pub ty:   Type,
    pub vl:   Value,
}

impl Data {
    pub fn new(name: &str, ty: Type) -> Data {
        Data {
            name: name.to_string(),
            ty:   ty,
            vl:   Value::Null,
        }
    }
}

pub type  FnArgs   = Vec<Data>;
pub type  FnReturn = Data;

pub struct RemoteCall {
        doc:       String,
    pub name:      String,
        arguments: FnArgs,
        result:    FnReturn,
        body:      Box<FnMut(&FnArgs) -> FnReturn>,
}

impl RemoteCall {
    pub fn new<F: FnMut(&FnArgs) -> FnReturn + Sized + 'static>(
        doc: &str, name: &str, args: FnArgs, result: FnReturn, body: F
    ) -> RemoteCall
    {
        RemoteCall {
            doc:       doc.to_string(),
            name:      name.to_string(),
            arguments: args,
            result:    result,
            body:      Box::new(body),
        }
    }

    pub fn call(&mut self, data: Vec<Data>) -> Result<FnReturn, String> {
        for arg in self.arguments.iter() {
            for d in data.iter() {
                if arg.ty != d.ty {
                    return Err("Error".to_string());
                }
            }
        }

        Ok((self.body)(&data))
    }
}

unsafe impl Send for RemoteCall {
}

/*
#[macro_export]
macro_rules! remote {
    
    
    
    /*
    ($name:ident ( ) -> $result:ident $body:block) => {{
        remote! {
            ///
            $name () -> $result $body
        }
    }};
    ($meta:meta $name:ident ( ) -> $result:ident $body:block) => {{
        use simple_soap::types::*;

        let args = vec![
        ];

        let result = Data::new(Type::$result);
        
        RemoteCall::new(stringify!($name), args, result,
            move |args: &FnArgs| -> FnReturn {
                let mut data = Data::new(Type::$result);
                data.vl = From::from({ $body });
                data
            }
        )
    }} */
}
*/