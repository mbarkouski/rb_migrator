use std::{fs::File, io::Read};

use crate::errors::{self, Error};

use super::options::Certificate;

///write by mbarkouski
#[cfg(feature = "tls")]
pub struct ClickHouseCertificateWrapper(Option<Certificate>);
///write by mbarkouski
#[cfg(feature = "tls")]
impl ClickHouseCertificateWrapper{
    pub fn get_cert(self)->Option<Certificate>{
        self.0
    }
    pub fn new(path_to_cert:Option<String>)->errors::Result<Self>{
        if let Some(path)=path_to_cert{
            let mut file = File::open(path).map_err(|err|{
                Error::Other(err.to_string().into())
            })?;
            let mut buff = Vec::new();
            file.read_to_end(&mut buff).map_err(|err|{
                Error::Other(err.to_string().into())
            })?;
            let inner=match native_tls::Certificate::from_pem(&buff) {
                Ok(certificate)=>{
                    certificate
                }
                Err(err)=>{
                    return  Err(Error::Other(err.to_string().into()))
                }
            };
            let inner_cert=Certificate::new(inner);
            return Ok(ClickHouseCertificateWrapper(Some(inner_cert)));
        }
        Ok(ClickHouseCertificateWrapper(None))
    }
}