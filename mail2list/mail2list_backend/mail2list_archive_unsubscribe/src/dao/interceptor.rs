use rbatis::plugin::intercept::SqlIntercept;
use rbatis::rbatis::Rbatis;
use rbatis::Error;
use rbson::Bson;

#[derive(Debug)]
pub struct AgencyInterceptor {}

impl SqlIntercept for AgencyInterceptor {
    fn do_intercept(
        &self,
        rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<Bson>,
        is_prepared_sql: bool,
    ) -> Result<(), Error> {
        println!("sql:{}", sql.clone());
        println!("args:{:?}", args.clone());
        return Ok(());
    }
}
