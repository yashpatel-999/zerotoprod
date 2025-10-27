use validator::validate_email;
use crate::error::SubscribeError;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail{
    pub fn parse(s:String)->Result<SubscriberEmail,SubscribeError>{
        if validate_email(&s){
            Ok(Self(s))
        } else{
            Err(SubscribeError::ValidationError(format!("Invalid email address: '{}'",s)))
        }
    }
}
impl AsRef<str> for SubscriberEmail{
    fn as_ref(&self)->&str{
        &self.0
    }
}

#[cfg(test)]
mod tests{
    use super::SubscriberEmail;
    use claim::assert_err;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn empty_string_is_rejected(){
        let email="".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected(){
        let email="ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected(){
        let email="@domian.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[derive(Debug,Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(_g: &mut G)->Self{
            let email=SafeEmail().fake();
            Self(email)
        }
    }
    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }
}