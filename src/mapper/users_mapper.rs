use crate::pojo::users::Users;
use rbatis::impl_select;

impl_select!(Users{login(username:String,password:String) -> Option => "`where username = #{username} AND password = #{password} limit 1`"});
